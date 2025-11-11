use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde_json;

use crate::retention::MemoryLink;

pub struct Store {
    conn: Connection,
}

impl Store {
    pub fn open(path: &str) -> Result<Self> {
        let conn = Connection::open(path)?;
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS events(
              id INTEGER PRIMARY KEY,
              ts TEXT NOT NULL,
              kind TEXT NOT NULL,
              payload TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS memory_links(
              id INTEGER PRIMARY KEY,
              phrase TEXT UNIQUE NOT NULL,
              last_seen TEXT NOT NULL,
              wave REAL NOT NULL,
              decay_alpha REAL NOT NULL,
              success_count INTEGER DEFAULT 0,
              fail_count INTEGER DEFAULT 0,
              use_in_wild_count INTEGER DEFAULT 0
            );

            CREATE TABLE IF NOT EXISTS sessions(
              id INTEGER PRIMARY KEY,
              script_id TEXT NOT NULL,
              started_at TEXT NOT NULL,
              completed_at TEXT,
              progress REAL DEFAULT 0.0
            );

            CREATE INDEX IF NOT EXISTS idx_events_ts ON events(ts);
            CREATE INDEX IF NOT EXISTS idx_events_kind ON events(kind);
            CREATE INDEX IF NOT EXISTS idx_memory_wave ON memory_links(wave);
        "#,
        )?;
        Ok(Self { conn })
    }

    // ========================================================================
    // Events
    // ========================================================================

    pub fn add_event(&self, kind: &str, payload: &str) -> Result<()> {
        let ts: DateTime<Utc> = Utc::now();
        self.conn.execute(
            "INSERT INTO events(ts,kind,payload) VALUES(?1,?2,?3)",
            params![ts.to_rfc3339(), kind, payload],
        )?;
        Ok(())
    }

    pub fn get_events(&self, kind: Option<&str>, limit: usize) -> Result<Vec<serde_json::Value>> {
        let mut query = "SELECT ts, kind, payload FROM events".to_string();
        if kind.is_some() {
            query.push_str(" WHERE kind = ?1");
        }
        query.push_str(" ORDER BY ts DESC LIMIT ?");

        let mut stmt = self.conn.prepare(&query)?;
        let rows = if let Some(k) = kind {
            stmt.query_map(params![k, limit], |row| {
                Ok(serde_json::json!({
                    "ts": row.get::<_, String>(0)?,
                    "kind": row.get::<_, String>(1)?,
                    "payload": row.get::<_, String>(2)?,
                }))
            })?
        } else {
            stmt.query_map(params![limit], |row| {
                Ok(serde_json::json!({
                    "ts": row.get::<_, String>(0)?,
                    "kind": row.get::<_, String>(1)?,
                    "payload": row.get::<_, String>(2)?,
                }))
            })?
        };

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // ========================================================================
    // Memory Links
    // ========================================================================

    pub fn save_memory_link(&self, link: &MemoryLink) -> Result<()> {
        self.conn.execute(
            r#"
            INSERT INTO memory_links(phrase, last_seen, wave, decay_alpha, success_count, fail_count, use_in_wild_count)
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7)
            ON CONFLICT(phrase) DO UPDATE SET
              last_seen = excluded.last_seen,
              wave = excluded.wave,
              success_count = excluded.success_count,
              fail_count = excluded.fail_count,
              use_in_wild_count = excluded.use_in_wild_count
            "#,
            params![
                link.phrase,
                link.last_seen.to_rfc3339(),
                link.wave,
                link.decay_alpha,
                link.success_count,
                link.fail_count,
                link.use_in_wild_count,
            ],
        )?;
        Ok(())
    }

    pub fn load_memory_link(&self, phrase: &str) -> Result<Option<MemoryLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT phrase, last_seen, wave, decay_alpha, success_count, fail_count, use_in_wild_count
             FROM memory_links WHERE phrase = ?1",
        )?;

        let mut rows = stmt.query(params![phrase])?;
        if let Some(row) = rows.next()? {
            let last_seen_str: String = row.get(1)?;
            let last_seen = DateTime::parse_from_rfc3339(&last_seen_str)?.with_timezone(&Utc);

            Ok(Some(MemoryLink {
                phrase: row.get(0)?,
                last_seen,
                wave: row.get(2)?,
                decay_alpha: row.get(3)?,
                success_count: row.get(4)?,
                fail_count: row.get(5)?,
                use_in_wild_count: row.get(6)?,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_all_memory_links(&self) -> Result<Vec<MemoryLink>> {
        let mut stmt = self.conn.prepare(
            "SELECT phrase, last_seen, wave, decay_alpha, success_count, fail_count, use_in_wild_count
             FROM memory_links ORDER BY wave ASC",
        )?;

        let rows = stmt.query_map([], |row| {
            let last_seen_str: String = row.get(1)?;
            let last_seen = DateTime::parse_from_rfc3339(&last_seen_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(1, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(MemoryLink {
                phrase: row.get(0)?,
                last_seen,
                wave: row.get(2)?,
                decay_alpha: row.get(3)?,
                success_count: row.get(4)?,
                fail_count: row.get(5)?,
                use_in_wild_count: row.get(6)?,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // ========================================================================
    // Sessions
    // ========================================================================

    pub fn start_session(&self, script_id: &str) -> Result<i64> {
        let ts = Utc::now().to_rfc3339();
        self.conn.execute(
            "INSERT INTO sessions(script_id, started_at) VALUES(?1, ?2)",
            params![script_id, ts],
        )?;
        Ok(self.conn.last_insert_rowid())
    }

    pub fn complete_session(&self, session_id: i64, progress: f32) -> Result<()> {
        let ts = Utc::now().to_rfc3339();
        self.conn.execute(
            "UPDATE sessions SET completed_at = ?1, progress = ?2 WHERE id = ?3",
            params![ts, progress, session_id],
        )?;
        Ok(())
    }

    // ========================================================================
    // Statistics
    // ========================================================================

    pub fn get_streak(&self) -> Result<u32> {
        // Simple implementation: count consecutive days with completed sessions
        // TODO: More sophisticated streak calculation
        let count: u32 = self.conn.query_row(
            "SELECT COUNT(DISTINCT DATE(started_at)) FROM sessions WHERE completed_at IS NOT NULL",
            [],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    pub fn get_use_in_wild_count(&self) -> Result<u32> {
        let count: u32 = self
            .conn
            .query_row("SELECT SUM(use_in_wild_count) FROM memory_links", [], |row| {
                row.get(0)
            })
            .unwrap_or(0);
        Ok(count)
    }

    pub fn export_json(&self) -> Result<String> {
        let events = self.get_events(None, 1000)?;
        let links = self.get_all_memory_links()?;

        let export = serde_json::json!({
            "events": events,
            "memory_links": links,
            "exported_at": Utc::now().to_rfc3339(),
        });

        Ok(serde_json::to_string_pretty(&export)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_store_open() {
        let store = Store::open(":memory:").unwrap();
        assert!(store.add_event("test", "{}").is_ok());
    }

    #[test]
    fn test_memory_link_save_load() {
        let store = Store::open(":memory:").unwrap();
        let link = MemoryLink::new("test phrase".to_string(), 0.8);

        store.save_memory_link(&link).unwrap();
        let loaded = store.load_memory_link("test phrase").unwrap();

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.phrase, "test phrase");
        assert_eq!(loaded.decay_alpha, 0.8);
    }
}
