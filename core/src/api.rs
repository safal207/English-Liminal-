use anyhow::Result;
use flutter_rust_bridge::frb;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::roles::{
    liminal_transition, EmotionTag, Reflection, ResonanceTrace, Role, RoleCoherenceScore,
    RoleProgress,
};
use crate::runner::RunnerState;
use crate::scripts::Script;
use crate::storage::Store;
use crate::telemetry::{DeviceContext, EventBatch, TelemetryEvent};

// Global state
static SCRIPTS: Lazy<Mutex<HashMap<String, Script>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static ROLES: Lazy<Mutex<HashMap<String, Role>>> = Lazy::new(|| Mutex::new(HashMap::new()));
static STORE: Lazy<Mutex<Option<Store>>> = Lazy::new(|| Mutex::new(None));

// ============================================================================
// Script Management
// ============================================================================

#[frb(sync)]
pub fn load_scripts_from_dir(dir: String) -> Result<u32, String> {
    let mut map = SCRIPTS.lock();
    map.clear();

    let path = PathBuf::from(&dir);
    if !path.exists() {
        return Err(format!("Directory not found: {}", dir));
    }

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let txt = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let script: Script = serde_yaml::from_str(&txt).map_err(|e| e.to_string())?;
            map.insert(script.id.clone(), script);
        }
    }

    Ok(map.len() as u32)
}

#[frb(sync)]
pub fn get_script_ids() -> Vec<String> {
    SCRIPTS.lock().keys().cloned().collect()
}

#[frb(sync)]
pub fn get_script_json(script_id: String) -> Result<String, String> {
    let map = SCRIPTS.lock();
    let script = map
        .get(&script_id)
        .ok_or_else(|| format!("Script not found: {}", script_id))?;
    serde_json::to_string(script).map_err(|e| e.to_string())
}

// ============================================================================
// Runner
// ============================================================================

#[frb(sync)]
pub fn start_runner(script_id: String) -> Result<String, String> {
    let map = SCRIPTS.lock();
    let script = map
        .get(&script_id)
        .ok_or_else(|| format!("Script not found: {}", script_id))?;
    let state = RunnerState::new(script);
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn runner_next(state_json: String) -> Result<String, String> {
    let mut state: RunnerState = serde_json::from_str(&state_json).map_err(|e| e.to_string())?;
    let map = SCRIPTS.lock();
    let script = map
        .get(&state.script_id)
        .ok_or_else(|| format!("Script not found: {}", state.script_id))?;
    state.next(script);
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn runner_prev(state_json: String) -> Result<String, String> {
    let mut state: RunnerState = serde_json::from_str(&state_json).map_err(|e| e.to_string())?;
    state.prev();
    serde_json::to_string(&state).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn runner_progress(state_json: String) -> Result<f32, String> {
    let state: RunnerState = serde_json::from_str(&state_json).map_err(|e| e.to_string())?;
    let map = SCRIPTS.lock();
    let script = map
        .get(&state.script_id)
        .ok_or_else(|| format!("Script not found: {}", state.script_id))?;
    Ok(state.progress(script))
}

#[frb(sync)]
pub fn runner_current_step(state_json: String) -> Result<String, String> {
    let state: RunnerState = serde_json::from_str(&state_json).map_err(|e| e.to_string())?;
    let map = SCRIPTS.lock();
    let script = map
        .get(&state.script_id)
        .ok_or_else(|| format!("Script not found: {}", state.script_id))?;

    let step = state
        .current_step(script)
        .ok_or_else(|| "No current step".to_string())?;

    serde_json::to_string(step).map_err(|e| e.to_string())
}

// ============================================================================
// Storage
// ============================================================================

#[frb(sync)]
pub fn init_storage(db_path: String) -> Result<(), String> {
    let store = Store::open(&db_path).map_err(|e| e.to_string())?;
    let mut guard = STORE.lock();
    *guard = Some(store);
    Ok(())
}

#[frb(sync)]
pub fn add_event(kind: String, payload: String) -> Result<(), String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store.add_event(&kind, &payload).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_events(kind: Option<String>, limit: u32) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    let events = store
        .get_events(kind.as_deref(), limit as usize)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&events).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn export_data() -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store.export_json().map_err(|e| e.to_string())
}

// ============================================================================
// Statistics
// ============================================================================

#[frb(sync)]
pub fn get_streak() -> Result<u32, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store.get_streak().map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_use_in_wild_count() -> Result<u32, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store.get_use_in_wild_count().map_err(|e| e.to_string())
}

// ============================================================================
// Role Management
// ============================================================================

#[frb(sync)]
pub fn load_roles_from_dir(dir: String) -> Result<u32, String> {
    let mut map = ROLES.lock();
    map.clear();

    let path = PathBuf::from(&dir);
    if !path.exists() {
        return Err(format!("Directory not found: {}", dir));
    }

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let txt = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            let role: Role = serde_yaml::from_str(&txt).map_err(|e| e.to_string())?;
            map.insert(role.id.clone(), role);
        }
    }

    Ok(map.len() as u32)
}

#[frb(sync)]
pub fn get_role_ids() -> Vec<String> {
    ROLES.lock().keys().cloned().collect()
}

#[frb(sync)]
pub fn get_role_json(role_id: String) -> Result<String, String> {
    let map = ROLES.lock();
    let role = map
        .get(&role_id)
        .ok_or_else(|| format!("Role not found: {}", role_id))?;
    serde_json::to_string(role).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn calculate_role_coherence(
    role_id: String,
    completed_scenarios: u32,
    use_in_wild_count: u32,
    skipped_steps: u32,
) -> Result<String, String> {
    let map = ROLES.lock();
    let role = map
        .get(&role_id)
        .ok_or_else(|| format!("Role not found: {}", role_id))?;

    let total_scenarios = role.scenario_ids.len() as u32;
    let mut coherence = RoleCoherenceScore {
        role_id: role_id.clone(),
        completed_scenarios,
        total_scenarios,
        use_in_wild_count,
        skipped_steps,
        score: 0.0,
    };

    coherence.calculate();
    serde_json::to_string(&coherence).map_err(|e| e.to_string())
}

// ============================================================================
// v1.1: Role Progress & Liminal Transitions
// ============================================================================

#[frb(sync)]
pub fn start_role_progress(role_id: String, total_scenes: u32) -> Result<String, String> {
    let progress = RoleProgress::new(role_id, total_scenes as usize);
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store
        .save_role_progress(&progress)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&progress).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn complete_scene_with_emotion(
    role_id: String,
    scene_id: String,
    tone: String,
    confidence: f32,
) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let mut progress = store
        .load_role_progress(&role_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Role progress not found: {}", role_id))?;

    let emotion = EmotionTag::new(scene_id, tone, confidence);
    progress.complete_scene(emotion);

    store
        .save_role_progress(&progress)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&progress).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_role_progress_json(role_id: String) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let progress = store
        .load_role_progress(&role_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Role progress not found: {}", role_id))?;

    serde_json::to_string(&progress).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_liminal_transition_json(role_id: String) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let progress = store
        .load_role_progress(&role_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Role progress not found: {}", role_id))?;

    let roles_map = ROLES.lock();
    let role = roles_map
        .get(&role_id)
        .ok_or_else(|| format!("Role not found: {}", role_id))?;

    let transition = liminal_transition(&progress, &role.title);
    serde_json::to_string(&transition).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn update_consecutive_days(role_id: String, days: u32) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let mut progress = store
        .load_role_progress(&role_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Role progress not found: {}", role_id))?;

    progress.consecutive_days = days;
    progress.calculate_coherence();

    store
        .save_role_progress(&progress)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&progress).map_err(|e| e.to_string())
}

// ============================================================================
// v1.1: Social Resonance
// ============================================================================

#[frb(sync)]
pub fn create_resonance_trace(
    trace_id: String,
    role_id: String,
    scene_id: String,
    message: String,
) -> Result<String, String> {
    let trace = ResonanceTrace::new(trace_id, role_id, scene_id, message);
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;
    store
        .save_resonance_trace(&trace)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&trace).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn add_reflection_to_trace(trace_id: String, message: String) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let mut trace = store
        .load_resonance_trace(&trace_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Resonance trace not found: {}", trace_id))?;

    let reflection = Reflection::new(trace_id.clone(), message);
    trace.add_reflection(reflection);

    store
        .save_resonance_trace(&trace)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&trace).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_recent_traces_json(role_id: Option<String>, limit: u32) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let traces = store
        .get_recent_traces(role_id.as_deref(), limit as usize)
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&traces).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_trace_json(trace_id: String) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let trace = store
        .load_resonance_trace(&trace_id)
        .map_err(|e| e.to_string())?
        .ok_or_else(|| format!("Resonance trace not found: {}", trace_id))?;

    serde_json::to_string(&trace).map_err(|e| e.to_string())
}

// ============================================================================
// v1.2: Telemetry & Analytics
// ============================================================================

#[frb(sync)]
pub fn track_event(
    event_type: String,
    session_id: Option<String>,
    user_id: Option<String>,
    properties_json: String,
    context_json: String,
) -> Result<(), String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    // Parse EventType from string
    let event_type_parsed: crate::telemetry::EventType =
        serde_json::from_str(&format!("\"{}\"", event_type)).map_err(|e| e.to_string())?;

    // Parse properties
    let properties: std::collections::HashMap<String, serde_json::Value> =
        serde_json::from_str(&properties_json).map_err(|e| e.to_string())?;

    // Parse context
    let context: DeviceContext = serde_json::from_str(&context_json).map_err(|e| e.to_string())?;

    // Create event
    let mut event = TelemetryEvent::new(event_type_parsed, context);
    event.session_id = session_id;
    event.user_id = user_id;
    event.properties = properties;

    store.add_telemetry_event(&event).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_pending_telemetry_events(limit: u32) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let events = store
        .get_pending_events(limit as usize)
        .map_err(|e| e.to_string())?;

    serde_json::to_string(&events).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn create_telemetry_batch(events_json: String) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let events: Vec<TelemetryEvent> =
        serde_json::from_str(&events_json).map_err(|e| e.to_string())?;

    let batch = EventBatch::new(events);
    store.save_batch(&batch).map_err(|e| e.to_string())?;

    serde_json::to_string(&batch).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn mark_telemetry_batch_sent(batch_id: String) -> Result<(), String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    store.mark_batch_sent(&batch_id).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_telemetry_stats() -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let stats = store.get_telemetry_stats().map_err(|e| e.to_string())?;
    serde_json::to_string(&stats).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn cleanup_old_telemetry(days: i64) -> Result<u32, String> {
    let guard = STORE.lock();
    let store = guard
        .as_ref()
        .ok_or_else(|| "Storage not initialized".to_string())?;

    let deleted = store
        .cleanup_old_telemetry(days)
        .map_err(|e| e.to_string())?;

    Ok(deleted as u32)
}

// ============================================================================
// Health Check
// ============================================================================

#[frb(sync)]
pub fn health_check() -> String {
    "OK".to_string()
}
