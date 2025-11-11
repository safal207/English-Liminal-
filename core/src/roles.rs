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
    pub goal: String,
    pub icon: Option<String>,
    pub benchmarks: Vec<String>,
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
#[derive(Debug, Clone)]
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
}
