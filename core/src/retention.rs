use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryLink {
    pub phrase: String,
    pub last_seen: DateTime<Utc>,
    pub wave: f32,       // 0..1
    pub decay_alpha: f32, // 0.7..0.9
    pub success_count: u32,
    pub fail_count: u32,
    pub use_in_wild_count: u32,
}

impl MemoryLink {
    pub fn new(phrase: String, decay_alpha: f32) -> Self {
        Self {
            phrase,
            last_seen: Utc::now(),
            wave: 1.0,
            decay_alpha: decay_alpha.clamp(0.7, 0.9),
            success_count: 0,
            fail_count: 0,
            use_in_wild_count: 0,
        }
    }

    /// Apply time-based decay to wave amplitude
    pub fn tick(&mut self, now: DateTime<Utc>) {
        let dt_secs = (now - self.last_seen).num_seconds().max(0) as f32;
        // Exponential decay: wave *= alpha^(dt/T), T=600s
        let t = 600.0_f32;
        let factor = self.decay_alpha.powf(dt_secs / t);
        self.wave = (self.wave * factor).clamp(0.0, 1.0);
    }

    /// Reinforce memory with positive gain
    pub fn reinforce(&mut self, now: DateTime<Utc>, gain: f32) {
        self.wave = (self.wave + gain).min(1.0);
        self.last_seen = now;
        self.success_count += 1;
    }

    /// Weaken memory (failure case)
    pub fn weaken(&mut self, now: DateTime<Utc>) {
        self.wave = (self.wave * self.decay_alpha).clamp(0.0, 1.0);
        self.last_seen = now;
        self.fail_count += 1;
    }

    /// Mark as used in real life (strong positive signal)
    pub fn mark_used_in_wild(&mut self, now: DateTime<Utc>) {
        self.wave = (self.wave + 0.2).min(1.0);
        self.last_seen = now;
        self.use_in_wild_count += 1;
    }
}

/// Calculate next ping delay in seconds based on wave amplitude
/// Lower wave → shorter delay (needs practice)
/// Higher wave → longer delay (already strong)
pub fn next_ping_seconds(link: &MemoryLink, min_s: u32, max_s: u32) -> u32 {
    let w = link.wave.clamp(0.0, 1.0);
    let span = (max_s - min_s) as f32;
    // Inverse relationship: low wave = min delay, high wave = max delay
    let when = min_s as f32 + w * span;
    when.round() as u32
}

/// Calculate priority for showing a memory link
/// Returns 0.0-1.0 (higher = more urgent)
pub fn calculate_priority(link: &MemoryLink, now: DateTime<Utc>) -> f32 {
    // Apply decay first
    let mut temp_link = link.clone();
    temp_link.tick(now);

    // Base priority from inverse of wave
    let mut priority = 1.0 - temp_link.wave;

    // Boost for phrases not seen recently
    let days_since = (now - link.last_seen).num_days() as f32;
    let time_boost = (days_since / 7.0).min(1.0) * 0.3;
    priority += time_boost;

    // Boost for phrases used in wild (shows real value)
    let wild_boost = (link.use_in_wild_count as f32 / 5.0).min(1.0) * 0.2;
    priority += wild_boost;

    priority.clamp(0.0, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_link_decay() {
        let mut link = MemoryLink::new("test phrase".to_string(), 0.8);
        let initial_wave = link.wave;

        // Simulate time passing
        let future = Utc::now() + chrono::Duration::seconds(600);
        link.tick(future);

        // Wave should decay
        assert!(link.wave < initial_wave);
    }

    #[test]
    fn test_reinforce() {
        let mut link = MemoryLink::new("test".to_string(), 0.8);
        link.wave = 0.5;

        link.reinforce(Utc::now(), 0.3);

        assert_eq!(link.wave, 0.8);
        assert_eq!(link.success_count, 1);
    }

    #[test]
    fn test_next_ping_seconds() {
        let mut link = MemoryLink::new("test".to_string(), 0.8);

        // Low wave = short delay
        link.wave = 0.1;
        let delay_low = next_ping_seconds(&link, 90, 3600);

        // High wave = long delay
        link.wave = 0.9;
        let delay_high = next_ping_seconds(&link, 90, 3600);

        assert!(delay_low < delay_high);
    }
}
