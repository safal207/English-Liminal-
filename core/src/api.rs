use anyhow::Result;
use flutter_rust_bridge::frb;
use once_cell::sync::Lazy;
use parking_lot::Mutex;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::runner::RunnerState;
use crate::scripts::Script;
use crate::storage::Store;

// Global state
static SCRIPTS: Lazy<Mutex<HashMap<String, Script>>> = Lazy::new(|| Mutex::new(HashMap::new()));
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
    let store = guard.as_ref().ok_or_else(|| "Storage not initialized".to_string())?;
    store.add_event(&kind, &payload).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_events(kind: Option<String>, limit: u32) -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard.as_ref().ok_or_else(|| "Storage not initialized".to_string())?;
    let events = store
        .get_events(kind.as_deref(), limit as usize)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(&events).map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn export_data() -> Result<String, String> {
    let guard = STORE.lock();
    let store = guard.as_ref().ok_or_else(|| "Storage not initialized".to_string())?;
    store.export_json().map_err(|e| e.to_string())
}

// ============================================================================
// Statistics
// ============================================================================

#[frb(sync)]
pub fn get_streak() -> Result<u32, String> {
    let guard = STORE.lock();
    let store = guard.as_ref().ok_or_else(|| "Storage not initialized".to_string())?;
    store.get_streak().map_err(|e| e.to_string())
}

#[frb(sync)]
pub fn get_use_in_wild_count() -> Result<u32, String> {
    let guard = STORE.lock();
    let store = guard.as_ref().ok_or_else(|| "Storage not initialized".to_string())?;
    store.get_use_in_wild_count().map_err(|e| e.to_string())
}

// ============================================================================
// Health Check
// ============================================================================

#[frb(sync)]
pub fn health_check() -> String {
    "OK".to_string()
}
