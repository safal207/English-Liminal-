#![allow(unexpected_cfgs)]

mod retention;
mod roles;
mod runner;
mod scripts;
mod storage;
pub mod telemetry;
pub mod validator;

// Export FFI API
pub mod api;

// Re-export key types for internal use
pub use retention::{calculate_priority, next_ping_seconds, MemoryLink};
pub use roles::{
    liminal_transition, Difficulty, EmotionTag, LiminalTransition, Reflection, ResonanceTrace,
    Role, RoleCoherenceScore, RolePath, RoleProgress,
};
pub use runner::RunnerState;
pub use scripts::{Answer, Rehearsal, Script, Step, StepType};
pub use storage::{Store, TelemetryStats};
pub use telemetry::{DeviceContext, EventBatch, EventType, TelemetryConfig, TelemetryEvent};
pub use validator::{ContentValidator, ValidationReport};
