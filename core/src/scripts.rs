use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StepType {
    Listen,
    SpeakCheck,
    Contrast,
    ApplyToLife,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub r#type: StepType,
    pub prompt: String,
    #[serde(default)]
    pub content: Option<String>,
    #[serde(default)]
    pub ref_text: Option<String>,
    #[serde(default)]
    pub answers: Option<Vec<String>>,
    #[serde(default)]
    pub hints: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rehearsal {
    #[serde(default = "default_decay")]
    pub decay_alpha: f32,           // 0.7..0.9
    #[serde(default = "default_ping_min")]
    pub next_ping_sec_min: u32,     // 90
    #[serde(default = "default_ping_max")]
    pub next_ping_sec_max: u32,     // 3600
}

fn default_decay() -> f32 { 0.82 }
fn default_ping_min() -> u32 { 90 }
fn default_ping_max() -> u32 { 3600 }

impl Default for Rehearsal {
    fn default() -> Self {
        Self {
            decay_alpha: default_decay(),
            next_ping_sec_min: default_ping_min(),
            next_ping_sec_max: default_ping_max(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Script {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub context_triggers: serde_json::Value,
    #[serde(default)]
    pub goals: Vec<String>,
    pub steps: Vec<Step>,
    #[serde(default)]
    pub rehearsal: Rehearsal,
}
