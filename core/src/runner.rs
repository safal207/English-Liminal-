use crate::scripts::{Script, StepType};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunnerState {
    pub script_id: String,
    pub current_index: usize,
    pub completed: bool,
}

impl RunnerState {
    pub fn new(script: &Script) -> Self {
        Self {
            script_id: script.id.clone(),
            current_index: 0,
            completed: false,
        }
    }

    pub fn next(&mut self, script: &Script) {
        if self.completed {
            return;
        }
        if self.current_index + 1 >= script.steps.len() {
            self.completed = true;
        } else {
            self.current_index += 1;
        }
    }

    pub fn prev(&mut self) {
        if self.current_index > 0 {
            self.current_index -= 1;
        }
    }

    pub fn progress(&self, script: &Script) -> f32 {
        ((self.current_index + 1) as f32 / script.steps.len().max(1) as f32).clamp(0.0, 1.0)
    }

    pub fn current_step_type(&self, script: &Script) -> Option<StepType> {
        script.steps.get(self.current_index).map(|s| s.r#type.clone())
    }

    pub fn current_step(&self, script: &Script) -> Option<&crate::scripts::Step> {
        script.steps.get(self.current_index)
    }

    pub fn reset(&mut self) {
        self.current_index = 0;
        self.completed = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scripts::{Script, Step, StepType, Rehearsal};

    fn mock_script() -> Script {
        Script {
            id: "test-01".to_string(),
            title: "Test Script".to_string(),
            description: None,
            context_triggers: serde_json::Value::Null,
            goals: vec![],
            steps: vec![
                Step {
                    r#type: StepType::Listen,
                    prompt: "Listen".to_string(),
                    content: None,
                    ref_text: None,
                    answers: None,
                    hints: None,
                },
                Step {
                    r#type: StepType::SpeakCheck,
                    prompt: "Speak".to_string(),
                    content: None,
                    ref_text: Some("test".to_string()),
                    answers: None,
                    hints: None,
                },
            ],
            rehearsal: Rehearsal::default(),
        }
    }

    #[test]
    fn test_runner_progress() {
        let script = mock_script();
        let mut runner = RunnerState::new(&script);

        assert_eq!(runner.progress(&script), 0.5); // 1/2
        runner.next(&script);
        assert_eq!(runner.progress(&script), 1.0); // 2/2
        assert!(runner.completed);
    }
}
