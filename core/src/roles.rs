use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Role: A professional or life identity that user embodies through scenarios
///
/// Example roles:
/// - QA Engineer Abroad
/// - Visa Applicant
/// - Diplomatic Communicator
/// - Hospitality Professional
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Role {
    pub id: String,
    pub title: String,
    pub description: String,
    #[serde(default)]
    pub goal: Option<String>,
    #[serde(default)]
    pub icon: Option<String>,
    #[serde(default)]
    pub benchmarks: Vec<String>,
    #[serde(default, alias = "scenes")]
    pub scenario_ids: Vec<String>,
    #[serde(default)]
    pub difficulty: Difficulty,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
}

impl Default for Difficulty {
    fn default() -> Self {
        Difficulty::Intermediate
    }
}

/// Role Coherence Score: measures how naturally user embodies the role
///
/// Calculation:
/// - Base: % of scenarios completed in role
/// - Bonus: use-in-wild mentions (+10% each, max +50%)
/// - Penalty: skipped steps (-5% each)
/// - Range: 0.0 - 1.0
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleCoherenceScore {
    pub role_id: String,
    pub completed_scenarios: u32,
    pub total_scenarios: u32,
    pub use_in_wild_count: u32,
    pub skipped_steps: u32,
    pub score: f32,
}

impl RoleCoherenceScore {
    pub fn new(role_id: String, total_scenarios: u32) -> Self {
        Self {
            role_id,
            completed_scenarios: 0,
            total_scenarios,
            use_in_wild_count: 0,
            skipped_steps: 0,
            score: 0.0,
        }
    }

    /// Calculate coherence score based on user progress
    pub fn calculate(&mut self) {
        if self.total_scenarios == 0 {
            self.score = 0.0;
            return;
        }

        // Base score: completion percentage
        let base = self.completed_scenarios as f32 / self.total_scenarios as f32;

        // Bonus: use-in-wild (real-world application)
        // Each use-in-wild adds 10%, max +50%
        let wild_bonus = (self.use_in_wild_count as f32 * 0.1).min(0.5);

        // Penalty: skipped steps (shows lack of engagement)
        // Each skip removes 5%, max -30%
        let skip_penalty = (self.skipped_steps as f32 * 0.05).min(0.3);

        // Final score
        self.score = (base + wild_bonus - skip_penalty).clamp(0.0, 1.0);
    }

    /// Get human-readable coherence level
    pub fn level(&self) -> &'static str {
        match self.score {
            s if s >= 0.9 => "Mastered",
            s if s >= 0.75 => "Proficient",
            s if s >= 0.5 => "Developing",
            s if s >= 0.25 => "Emerging",
            _ => "Novice",
        }
    }

    /// Mark a scenario as completed
    pub fn complete_scenario(&mut self) {
        self.completed_scenarios += 1;
        self.calculate();
    }

    /// Mark use-in-wild (real-world application)
    pub fn mark_wild(&mut self) {
        self.use_in_wild_count += 1;
        self.calculate();
    }

    /// Record a skipped step
    pub fn skip_step(&mut self) {
        self.skipped_steps += 1;
        self.calculate();
    }
}

/// Role Path: sequence of roles for progressive development
///
/// Example path: "Work Abroad"
/// 1. Visa Applicant
/// 2. Job Seeker
/// 3. QA Engineer
/// 4. Team Collaborator
/// 5. Mentor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RolePath {
    pub id: String,
    pub title: String,
    pub description: String,
    pub role_ids: Vec<String>,
}

impl RolePath {
    /// Get next role in path after current_role_id
    pub fn next_role(&self, current_role_id: &str) -> Option<&str> {
        let pos = self.role_ids.iter().position(|id| id == current_role_id)?;
        self.role_ids.get(pos + 1).map(|s| s.as_str())
    }

    /// Calculate overall path progress
    pub fn progress(&self, completed_roles: &[String]) -> f32 {
        if self.role_ids.is_empty() {
            return 0.0;
        }

        let completed_count = self
            .role_ids
            .iter()
            .filter(|id| completed_roles.contains(id))
            .count();

        completed_count as f32 / self.role_ids.len() as f32
    }
}

// ============================================================================
// v1.1: Role Progress & Emotion Tracking
// ============================================================================

/// RoleProgress: User's active progress through a role
///
/// Tracks completion, emotion patterns, and liminal transitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleProgress {
    pub role_id: String,
    pub current_scene_index: usize,
    pub total_scenes: usize,
    pub coherence: f32,
    pub emotion_tags: Vec<EmotionTag>,
    pub last_transition: Option<DateTime<Utc>>,
    pub consecutive_days: u32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl RoleProgress {
    pub fn new(role_id: String, total_scenes: usize) -> Self {
        let now = Utc::now();
        Self {
            role_id,
            current_scene_index: 0,
            total_scenes,
            coherence: 0.0,
            emotion_tags: Vec::new(),
            last_transition: None,
            consecutive_days: 0,
            created_at: now,
            updated_at: now,
        }
    }

    /// Complete current scene and trigger liminal transition
    pub fn complete_scene(&mut self, emotion: EmotionTag) {
        self.current_scene_index += 1;
        self.emotion_tags.push(emotion);
        self.last_transition = Some(Utc::now());
        self.updated_at = Utc::now();
        self.calculate_coherence();
    }

    /// Calculate RoleFlow coherence
    pub fn calculate_coherence(&mut self) {
        if self.total_scenes == 0 {
            self.coherence = 0.0;
            return;
        }

        // Base: completion percentage
        let base = self.current_scene_index as f32 / self.total_scenes as f32;

        // Consistency multiplier
        let consistency = self.consistency_multiplier();

        self.coherence = (base * consistency).clamp(0.0, 1.0);
    }

    /// Calculate consistency multiplier based on engagement pattern
    fn consistency_multiplier(&self) -> f32 {
        match self.consecutive_days {
            d if d >= 7 => 1.5, // Week+ streak
            d if d >= 3 => 1.2, // 3-6 days
            d if d >= 1 => 1.0, // Daily
            _ => 0.8,           // Sporadic
        }
    }

    /// Get emotion balance (confident vs nervous)
    pub fn emotion_balance(&self) -> f32 {
        if self.emotion_tags.is_empty() {
            return 0.5; // neutral
        }

        let confident_count = self
            .emotion_tags
            .iter()
            .filter(|e| matches!(e.tone.as_str(), "Calm" | "Confident" | "Clear"))
            .count();

        let total = self.emotion_tags.len();
        confident_count as f32 / total as f32
    }

    /// Check if ready for liminal transition (>= 75% coherence)
    pub fn is_transition_ready(&self) -> bool {
        self.coherence >= 0.75
    }
}

/// EmotionTag: Emotional analysis from voice interaction
///
/// Generated from Whisper ASR + tone analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionTag {
    pub scene_id: String,
    pub tone: String,    // "Calm", "Nervous", "Confident", "Uncertain"
    pub confidence: f32, // 0.0 - 1.0
    pub timestamp: DateTime<Utc>,
}

impl EmotionTag {
    pub fn new(scene_id: String, tone: String, confidence: f32) -> Self {
        Self {
            scene_id,
            tone,
            confidence,
            timestamp: Utc::now(),
        }
    }

    /// Get color code for UI feedback
    pub fn color_hex(&self) -> &'static str {
        match self.tone.as_str() {
            "Calm" | "Confident" | "Clear" => "#7ED321",     // Green
            "Nervous" | "Uncertain" | "Rushed" => "#F5A623", // Amber
            _ => "#4A90E2",                                  // Blue (neutral)
        }
    }

    /// Get wave amplitude for visualization
    pub fn wave_amplitude(&self) -> u8 {
        match self.tone.as_str() {
            "Calm" => 3,
            "Confident" | "Clear" => 5,
            "Excited" | "Energetic" => 8,
            _ => 4, // neutral
        }
    }
}

// ============================================================================
// v1.1: Social Resonance
// ============================================================================

/// ResonanceTrace: Anonymous social sharing of role experience
///
/// Users can share their journey moments, others can reflect
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResonanceTrace {
    pub id: String,
    pub role_id: String,
    pub scene_id: String,
    pub message: String,
    pub reflections: Vec<Reflection>,
    pub created_at: DateTime<Utc>,
}

impl ResonanceTrace {
    pub fn new(id: String, role_id: String, scene_id: String, message: String) -> Self {
        Self {
            id,
            role_id,
            scene_id,
            message,
            reflections: Vec::new(),
            created_at: Utc::now(),
        }
    }

    /// Add a reflection from another user
    pub fn add_reflection(&mut self, reflection: Reflection) {
        self.reflections.push(reflection);
    }

    /// Get SocialEcho score
    pub fn social_echo_score(&self) -> u32 {
        self.reflections.len() as u32
    }
}

/// Reflection: Anonymous response to a ResonanceTrace
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reflection {
    pub trace_id: String,
    pub message: String,
    pub created_at: DateTime<Utc>,
}

impl Reflection {
    pub fn new(trace_id: String, message: String) -> Self {
        Self {
            trace_id,
            message,
            created_at: Utc::now(),
        }
    }
}

// ============================================================================
// v1.1: Liminal Transition Logic
// ============================================================================

/// Trigger liminal transition when scene completes
///
/// Returns transition message and animation config
pub fn liminal_transition(progress: &RoleProgress, role_title: &str) -> LiminalTransition {
    let prev_coherence = if progress.current_scene_index > 0 {
        (progress.current_scene_index - 1) as f32 / progress.total_scenes as f32
    } else {
        0.0
    };

    let curr_coherence = progress.coherence;

    let message = match curr_coherence {
        c if c >= 0.9 => format!(
            "You've mastered being {}. Ready for the next chapter?",
            role_title
        ),
        c if c >= 0.75 => format!(
            "You're one step closer to being the {} you want to be.",
            role_title
        ),
        c if c >= 0.5 => format!("Half way there. The {} in you is emerging.", role_title),
        c if c >= 0.25 => format!("You're on the path to becoming {}.", role_title),
        _ => format!("First step taken as {}. Keep going.", role_title),
    };

    LiminalTransition {
        message,
        prev_coherence,
        curr_coherence,
        animation_duration_ms: 2500,
        color_from: "#4A90E2".to_string(), // Blue
        color_to: "#7ED321".to_string(),   // Green
        sound: "chime_gentle.mp3".to_string(),
    }
}

/// LiminalTransition: Configuration for transition animation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiminalTransition {
    pub message: String,
    pub prev_coherence: f32,
    pub curr_coherence: f32,
    pub animation_duration_ms: u64,
    pub color_from: String,
    pub color_to: String,
    pub sound: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_coherence_score_calculation() {
        let mut score = RoleCoherenceScore::new("qa-engineer".to_string(), 5);

        // Initially 0
        assert_eq!(score.score, 0.0);
        assert_eq!(score.level(), "Novice");

        // Complete 3/5 scenarios
        score.completed_scenarios = 3;
        score.calculate();
        assert_eq!(score.score, 0.6); // 60%
        assert_eq!(score.level(), "Developing");

        // Add wild usage
        score.mark_wild();
        score.mark_wild();
        assert!((score.score - 0.8).abs() < 0.01); // 60% + 20% = 80%
        assert_eq!(score.level(), "Proficient");

        // Skip a step
        score.skip_step();
        assert!((score.score - 0.75).abs() < 0.01); // 80% - 5% = 75%
    }

    #[test]
    fn test_role_path_progression() {
        let path = RolePath {
            id: "work-abroad".to_string(),
            title: "Work Abroad".to_string(),
            description: "Path to working professionally in English".to_string(),
            role_ids: vec![
                "visa-applicant".to_string(),
                "job-seeker".to_string(),
                "qa-engineer".to_string(),
            ],
        };

        // Next role after visa-applicant
        assert_eq!(path.next_role("visa-applicant"), Some("job-seeker"));

        // Progress calculation
        let completed = vec!["visa-applicant".to_string(), "job-seeker".to_string()];
        assert!((path.progress(&completed) - 0.666).abs() < 0.01); // 2/3
    }

    #[test]
    fn test_coherence_level_boundaries() {
        let mut score = RoleCoherenceScore::new("test".to_string(), 10);

        score.score = 0.95;
        assert_eq!(score.level(), "Mastered");

        score.score = 0.8;
        assert_eq!(score.level(), "Proficient");

        score.score = 0.6;
        assert_eq!(score.level(), "Developing");

        score.score = 0.3;
        assert_eq!(score.level(), "Emerging");

        score.score = 0.1;
        assert_eq!(score.level(), "Novice");
    }

    #[test]
    fn test_role_progress() {
        let mut progress = RoleProgress::new("qa-engineer".to_string(), 5);

        assert_eq!(progress.current_scene_index, 0);
        assert_eq!(progress.coherence, 0.0);

        // Complete first scene
        let emotion = EmotionTag::new("scene-1".to_string(), "Calm".to_string(), 0.85);
        progress.complete_scene(emotion);

        assert_eq!(progress.current_scene_index, 1);
        assert!(progress.coherence > 0.0);
        assert_eq!(progress.emotion_tags.len(), 1);
    }

    #[test]
    fn test_emotion_balance() {
        let mut progress = RoleProgress::new("test".to_string(), 5);

        // Add 3 confident, 1 nervous
        progress
            .emotion_tags
            .push(EmotionTag::new("s1".to_string(), "Calm".to_string(), 0.9));
        progress.emotion_tags.push(EmotionTag::new(
            "s2".to_string(),
            "Confident".to_string(),
            0.85,
        ));
        progress
            .emotion_tags
            .push(EmotionTag::new("s3".to_string(), "Clear".to_string(), 0.8));
        progress.emotion_tags.push(EmotionTag::new(
            "s4".to_string(),
            "Nervous".to_string(),
            0.6,
        ));

        let balance = progress.emotion_balance();
        assert!((balance - 0.75).abs() < 0.01); // 3/4 = 0.75
    }

    #[test]
    fn test_liminal_transition() {
        let progress = RoleProgress::new("qa-engineer".to_string(), 5);
        let transition = liminal_transition(&progress, "QA Engineer");

        assert!(transition.message.contains("QA Engineer"));
        assert_eq!(transition.prev_coherence, 0.0);
        assert_eq!(transition.animation_duration_ms, 2500);
    }

    #[test]
    fn test_resonance_trace() {
        let mut trace = ResonanceTrace::new(
            "trace-1".to_string(),
            "qa-engineer".to_string(),
            "scene-1".to_string(),
            "First interview done!".to_string(),
        );

        assert_eq!(trace.social_echo_score(), 0);

        // Add reflections
        trace.add_reflection(Reflection::new(
            "trace-1".to_string(),
            "Great job!".to_string(),
        ));
        trace.add_reflection(Reflection::new(
            "trace-1".to_string(),
            "Keep going!".to_string(),
        ));

        assert_eq!(trace.social_echo_score(), 2);
    }
}
