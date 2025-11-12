use anyhow::Result;
use chrono::{DateTime, Utc};
use rusqlite::{params, Connection};
use serde_json;

use crate::retention::MemoryLink;
use crate::roles::{EmotionTag, Reflection, ResonanceTrace, RoleProgress};

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

            CREATE TABLE IF NOT EXISTS role_progress(
              id INTEGER PRIMARY KEY,
              role_id TEXT UNIQUE NOT NULL,
              current_scene_index INTEGER NOT NULL,
              total_scenes INTEGER NOT NULL,
              coherence REAL NOT NULL,
              last_transition TEXT,
              consecutive_days INTEGER DEFAULT 0,
              created_at TEXT NOT NULL,
              updated_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS emotion_tags(
              id INTEGER PRIMARY KEY,
              role_id TEXT NOT NULL,
              scene_id TEXT NOT NULL,
              tone TEXT NOT NULL,
              confidence REAL NOT NULL,
              timestamp TEXT NOT NULL,
              FOREIGN KEY(role_id) REFERENCES role_progress(role_id)
            );

            CREATE TABLE IF NOT EXISTS resonance_traces(
              id TEXT PRIMARY KEY,
              role_id TEXT NOT NULL,
              scene_id TEXT NOT NULL,
              message TEXT NOT NULL,
              created_at TEXT NOT NULL
            );

            CREATE TABLE IF NOT EXISTS reflections(
              id INTEGER PRIMARY KEY,
              trace_id TEXT NOT NULL,
              message TEXT NOT NULL,
              created_at TEXT NOT NULL,
              FOREIGN KEY(trace_id) REFERENCES resonance_traces(id)
            );

            CREATE INDEX IF NOT EXISTS idx_events_ts ON events(ts);
            CREATE INDEX IF NOT EXISTS idx_events_kind ON events(kind);
            CREATE INDEX IF NOT EXISTS idx_memory_wave ON memory_links(wave);
            CREATE INDEX IF NOT EXISTS idx_emotion_tags_role ON emotion_tags(role_id);
            CREATE INDEX IF NOT EXISTS idx_resonance_traces_role ON resonance_traces(role_id);
            CREATE INDEX IF NOT EXISTS idx_reflections_trace ON reflections(trace_id);
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
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(k) = kind {
            query.push_str(" WHERE kind = ?1");
            params_vec.push(Box::new(k.to_string()));
        }
        query.push_str(" ORDER BY ts DESC LIMIT ?");
        params_vec.push(Box::new(limit));

        let mut stmt = self.conn.prepare(&query)?;

        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            Ok(serde_json::json!({
                "ts": row.get::<_, String>(0)?,
                "kind": row.get::<_, String>(1)?,
                "payload": row.get::<_, String>(2)?,
            }))
        })?;

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

    // ========================================================================
    // v1.1: Role Progress
    // ========================================================================

    pub fn save_role_progress(&self, progress: &RoleProgress) -> Result<()> {
        // Save role progress
        self.conn.execute(
            r#"
            INSERT INTO role_progress(role_id, current_scene_index, total_scenes, coherence, last_transition, consecutive_days, created_at, updated_at)
            VALUES(?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)
            ON CONFLICT(role_id) DO UPDATE SET
              current_scene_index = excluded.current_scene_index,
              total_scenes = excluded.total_scenes,
              coherence = excluded.coherence,
              last_transition = excluded.last_transition,
              consecutive_days = excluded.consecutive_days,
              updated_at = excluded.updated_at
            "#,
            params![
                progress.role_id,
                progress.current_scene_index as i64,
                progress.total_scenes as i64,
                progress.coherence,
                progress.last_transition.map(|dt| dt.to_rfc3339()),
                progress.consecutive_days as i64,
                progress.created_at.to_rfc3339(),
                progress.updated_at.to_rfc3339(),
            ],
        )?;

        // Clear old emotion tags for this role
        self.conn.execute(
            "DELETE FROM emotion_tags WHERE role_id = ?1",
            params![progress.role_id],
        )?;

        // Save emotion tags
        for emotion in &progress.emotion_tags {
            self.save_emotion_tag(&progress.role_id, emotion)?;
        }

        Ok(())
    }

    pub fn load_role_progress(&self, role_id: &str) -> Result<Option<RoleProgress>> {
        let mut stmt = self.conn.prepare(
            "SELECT role_id, current_scene_index, total_scenes, coherence, last_transition, consecutive_days, created_at, updated_at
             FROM role_progress WHERE role_id = ?1",
        )?;

        let mut rows = stmt.query(params![role_id])?;
        if let Some(row) = rows.next()? {
            let created_at_str: String = row.get(6)?;
            let updated_at_str: String = row.get(7)?;
            let last_transition_str: Option<String> = row.get(4)?;

            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);
            let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)?.with_timezone(&Utc);
            let last_transition = last_transition_str
                .map(|s| DateTime::parse_from_rfc3339(&s).map(|dt| dt.with_timezone(&Utc)))
                .transpose()?;

            // Load emotion tags
            let emotion_tags = self.load_emotion_tags(role_id)?;

            Ok(Some(RoleProgress {
                role_id: row.get(0)?,
                current_scene_index: row.get::<_, i64>(1)? as usize,
                total_scenes: row.get::<_, i64>(2)? as usize,
                coherence: row.get(3)?,
                last_transition,
                consecutive_days: row.get::<_, i64>(5)? as u32,
                created_at,
                updated_at,
                emotion_tags,
            }))
        } else {
            Ok(None)
        }
    }

    fn save_emotion_tag(&self, role_id: &str, emotion: &EmotionTag) -> Result<()> {
        self.conn.execute(
            "INSERT INTO emotion_tags(role_id, scene_id, tone, confidence, timestamp) VALUES(?1, ?2, ?3, ?4, ?5)",
            params![
                role_id,
                emotion.scene_id,
                emotion.tone,
                emotion.confidence,
                emotion.timestamp.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    fn load_emotion_tags(&self, role_id: &str) -> Result<Vec<EmotionTag>> {
        let mut stmt = self.conn.prepare(
            "SELECT scene_id, tone, confidence, timestamp FROM emotion_tags WHERE role_id = ?1 ORDER BY timestamp ASC",
        )?;

        let rows = stmt.query_map(params![role_id], |row| {
            let timestamp_str: String = row.get(3)?;
            let timestamp = DateTime::parse_from_rfc3339(&timestamp_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(3, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(EmotionTag {
                scene_id: row.get(0)?,
                tone: row.get(1)?,
                confidence: row.get(2)?,
                timestamp,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
    }

    // ========================================================================
    // v1.1: Social Resonance
    // ========================================================================

    pub fn save_resonance_trace(&self, trace: &ResonanceTrace) -> Result<()> {
        self.conn.execute(
            "INSERT INTO resonance_traces(id, role_id, scene_id, message, created_at) VALUES(?1, ?2, ?3, ?4, ?5)",
            params![
                trace.id,
                trace.role_id,
                trace.scene_id,
                trace.message,
                trace.created_at.to_rfc3339(),
            ],
        )?;

        // Save reflections
        for reflection in &trace.reflections {
            self.save_reflection(reflection)?;
        }

        Ok(())
    }

    pub fn load_resonance_trace(&self, trace_id: &str) -> Result<Option<ResonanceTrace>> {
        let mut stmt = self.conn.prepare(
            "SELECT id, role_id, scene_id, message, created_at FROM resonance_traces WHERE id = ?1",
        )?;

        let mut rows = stmt.query(params![trace_id])?;
        if let Some(row) = rows.next()? {
            let created_at_str: String = row.get(4)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)?.with_timezone(&Utc);

            let reflections = self.get_reflections_for_trace(trace_id)?;

            Ok(Some(ResonanceTrace {
                id: row.get(0)?,
                role_id: row.get(1)?,
                scene_id: row.get(2)?,
                message: row.get(3)?,
                created_at,
                reflections,
            }))
        } else {
            Ok(None)
        }
    }

    pub fn get_recent_traces(&self, role_id: Option<&str>, limit: usize) -> Result<Vec<ResonanceTrace>> {
        let mut query = "SELECT id, role_id, scene_id, message, created_at FROM resonance_traces".to_string();
        let mut params_vec: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

        if let Some(rid) = role_id {
            query.push_str(" WHERE role_id = ?1");
            params_vec.push(Box::new(rid.to_string()));
        }
        query.push_str(" ORDER BY created_at DESC LIMIT ?");
        params_vec.push(Box::new(limit));

        let mut stmt = self.conn.prepare(&query)?;
        let params_refs: Vec<&dyn rusqlite::ToSql> =
            params_vec.iter().map(|p| p.as_ref() as &dyn rusqlite::ToSql).collect();

        let rows = stmt.query_map(params_refs.as_slice(), |row| {
            let created_at_str: String = row.get(4)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(4, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok((
                row.get::<_, String>(0)?, // id
                row.get::<_, String>(1)?, // role_id
                row.get::<_, String>(2)?, // scene_id
                row.get::<_, String>(3)?, // message
                created_at,
            ))
        })?;

        let mut traces = Vec::new();
        for row_result in rows {
            let (id, role_id, scene_id, message, created_at) = row_result?;
            let reflections = self.get_reflections_for_trace(&id)?;
            traces.push(ResonanceTrace {
                id,
                role_id,
                scene_id,
                message,
                created_at,
                reflections,
            });
        }

        Ok(traces)
    }

    fn save_reflection(&self, reflection: &Reflection) -> Result<()> {
        self.conn.execute(
            "INSERT INTO reflections(trace_id, message, created_at) VALUES(?1, ?2, ?3)",
            params![
                reflection.trace_id,
                reflection.message,
                reflection.created_at.to_rfc3339(),
            ],
        )?;
        Ok(())
    }

    pub fn get_reflections_for_trace(&self, trace_id: &str) -> Result<Vec<Reflection>> {
        let mut stmt = self.conn.prepare(
            "SELECT trace_id, message, created_at FROM reflections WHERE trace_id = ?1 ORDER BY created_at ASC",
        )?;

        let rows = stmt.query_map(params![trace_id], |row| {
            let created_at_str: String = row.get(2)?;
            let created_at = DateTime::parse_from_rfc3339(&created_at_str)
                .map_err(|e| rusqlite::Error::FromSqlConversionFailure(2, rusqlite::types::Type::Text, Box::new(e)))?
                .with_timezone(&Utc);

            Ok(Reflection {
                trace_id: row.get(0)?,
                message: row.get(1)?,
                created_at,
            })
        })?;

        rows.collect::<Result<Vec<_>, _>>().map_err(Into::into)
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

    #[test]
    fn test_role_progress_save_load() {
        let store = Store::open(":memory:").unwrap();
        let progress = RoleProgress::new("qa_abroad".to_string(), 5);

        store.save_role_progress(&progress).unwrap();
        let loaded = store.load_role_progress("qa_abroad").unwrap();

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.role_id, "qa_abroad");
        assert_eq!(loaded.total_scenes, 5);
        assert_eq!(loaded.current_scene_index, 0);
        assert_eq!(loaded.coherence, 0.0);
    }

    #[test]
    fn test_role_progress_with_emotions() {
        let store = Store::open(":memory:").unwrap();
        let mut progress = RoleProgress::new("qa_abroad".to_string(), 5);

        let emotion1 = EmotionTag::new("scene1".to_string(), "Calm".to_string(), 0.85);
        let emotion2 = EmotionTag::new("scene2".to_string(), "Confident".to_string(), 0.92);
        progress.complete_scene(emotion1);
        progress.complete_scene(emotion2);

        store.save_role_progress(&progress).unwrap();
        let loaded = store.load_role_progress("qa_abroad").unwrap();

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.emotion_tags.len(), 2);
        assert_eq!(loaded.emotion_tags[0].tone, "Calm");
        assert_eq!(loaded.emotion_tags[1].tone, "Confident");
        assert_eq!(loaded.current_scene_index, 2);
    }

    #[test]
    fn test_role_progress_update() {
        let store = Store::open(":memory:").unwrap();
        let mut progress = RoleProgress::new("qa_abroad".to_string(), 5);

        store.save_role_progress(&progress).unwrap();

        progress.current_scene_index = 3;
        progress.coherence = 0.6;
        progress.consecutive_days = 7;

        store.save_role_progress(&progress).unwrap();
        let loaded = store.load_role_progress("qa_abroad").unwrap().unwrap();

        assert_eq!(loaded.current_scene_index, 3);
        assert_eq!(loaded.coherence, 0.6);
        assert_eq!(loaded.consecutive_days, 7);
    }

    #[test]
    fn test_resonance_trace_save_load() {
        let store = Store::open(":memory:").unwrap();
        let trace = ResonanceTrace::new(
            "trace1".to_string(),
            "qa_abroad".to_string(),
            "scene1".to_string(),
            "Felt nervous but pushed through!".to_string(),
        );

        store.save_resonance_trace(&trace).unwrap();
        let loaded = store.load_resonance_trace("trace1").unwrap();

        assert!(loaded.is_some());
        let loaded = loaded.unwrap();
        assert_eq!(loaded.id, "trace1");
        assert_eq!(loaded.role_id, "qa_abroad");
        assert_eq!(loaded.message, "Felt nervous but pushed through!");
        assert_eq!(loaded.reflections.len(), 0);
    }

    #[test]
    fn test_resonance_trace_with_reflections() {
        let store = Store::open(":memory:").unwrap();
        let mut trace = ResonanceTrace::new(
            "trace1".to_string(),
            "qa_abroad".to_string(),
            "scene1".to_string(),
            "Felt nervous but pushed through!".to_string(),
        );

        let reflection1 = Reflection::new("trace1".to_string(), "Same here!".to_string());
        let reflection2 = Reflection::new("trace1".to_string(), "Keep going!".to_string());
        trace.add_reflection(reflection1);
        trace.add_reflection(reflection2);

        store.save_resonance_trace(&trace).unwrap();
        let loaded = store.load_resonance_trace("trace1").unwrap().unwrap();

        assert_eq!(loaded.reflections.len(), 2);
        assert_eq!(loaded.reflections[0].message, "Same here!");
        assert_eq!(loaded.reflections[1].message, "Keep going!");
    }

    #[test]
    fn test_get_recent_traces() {
        let store = Store::open(":memory:").unwrap();

        let trace1 = ResonanceTrace::new(
            "trace1".to_string(),
            "qa_abroad".to_string(),
            "scene1".to_string(),
            "Message 1".to_string(),
        );
        let trace2 = ResonanceTrace::new(
            "trace2".to_string(),
            "qa_abroad".to_string(),
            "scene2".to_string(),
            "Message 2".to_string(),
        );
        let trace3 = ResonanceTrace::new(
            "trace3".to_string(),
            "visa_journey".to_string(),
            "scene1".to_string(),
            "Message 3".to_string(),
        );

        store.save_resonance_trace(&trace1).unwrap();
        store.save_resonance_trace(&trace2).unwrap();
        store.save_resonance_trace(&trace3).unwrap();

        let all_traces = store.get_recent_traces(None, 10).unwrap();
        assert_eq!(all_traces.len(), 3);

        let qa_traces = store.get_recent_traces(Some("qa_abroad"), 10).unwrap();
        assert_eq!(qa_traces.len(), 2);

        let limited_traces = store.get_recent_traces(None, 2).unwrap();
        assert_eq!(limited_traces.len(), 2);
    }

    #[test]
    fn test_emotion_tags_per_role() {
        let store = Store::open(":memory:").unwrap();

        let mut progress1 = RoleProgress::new("qa_abroad".to_string(), 3);
        progress1.complete_scene(EmotionTag::new("s1".to_string(), "Calm".to_string(), 0.8));
        progress1.complete_scene(EmotionTag::new("s2".to_string(), "Confident".to_string(), 0.9));

        let mut progress2 = RoleProgress::new("visa_journey".to_string(), 2);
        progress2.complete_scene(EmotionTag::new("s1".to_string(), "Nervous".to_string(), 0.7));

        store.save_role_progress(&progress1).unwrap();
        store.save_role_progress(&progress2).unwrap();

        let loaded1 = store.load_role_progress("qa_abroad").unwrap().unwrap();
        let loaded2 = store.load_role_progress("visa_journey").unwrap().unwrap();

        assert_eq!(loaded1.emotion_tags.len(), 2);
        assert_eq!(loaded2.emotion_tags.len(), 1);
        assert_eq!(loaded1.emotion_tags[0].tone, "Calm");
        assert_eq!(loaded2.emotion_tags[0].tone, "Nervous");
    }
}
