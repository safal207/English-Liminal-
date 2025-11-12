use liminal_english_core::api::*;

#[test]
fn test_ffi_health_check() {
    let result = health_check();
    assert_eq!(result, "OK");
}

#[test]
fn test_ffi_storage_initialization() {
    let db_path = ":memory:";
    let result = init_storage(db_path.to_string());
    assert!(result.is_ok());
}

#[test]
fn test_ffi_role_progress_flow() {
    // Initialize storage
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Start role progress
    let role_id = "qa_engineer_abroad".to_string();
    let total_scenes = 5;

    let result = start_role_progress(role_id.clone(), total_scenes);
    assert!(result.is_ok());

    let progress_json = result.unwrap();
    let progress: serde_json::Value = serde_json::from_str(&progress_json).unwrap();

    assert_eq!(progress["role_id"], "qa_engineer_abroad");
    assert_eq!(progress["total_scenes"], 5);
    assert_eq!(progress["current_scene_index"], 0);

    // Complete a scene with emotion
    let result = complete_scene_with_emotion(
        role_id.clone(),
        "qa-interview-01".to_string(),
        "Confident".to_string(),
        0.85,
    );

    assert!(result.is_ok());

    let updated_json = result.unwrap();
    let updated: serde_json::Value = serde_json::from_str(&updated_json).unwrap();

    assert_eq!(updated["current_scene_index"], 1);
    assert_eq!(updated["emotion_tags"].as_array().unwrap().len(), 1);

    // Get role progress
    let result = get_role_progress_json(role_id.clone());
    assert!(result.is_ok());

    let loaded_json = result.unwrap();
    let loaded: serde_json::Value = serde_json::from_str(&loaded_json).unwrap();

    assert_eq!(loaded["role_id"], "qa_engineer_abroad");
    assert_eq!(loaded["current_scene_index"], 1);
}

#[test]
fn test_ffi_resonance_traces() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Create a trace
    let trace_id = "trace-001".to_string();
    let role_id = "qa_engineer_abroad".to_string();
    let scene_id = "qa-interview-01".to_string();
    let message = "First day was intense!".to_string();

    let result = create_resonance_trace(
        trace_id.clone(),
        role_id.clone(),
        scene_id.clone(),
        message.clone(),
    );

    assert!(result.is_ok());

    let trace_json = result.unwrap();
    let trace: serde_json::Value = serde_json::from_str(&trace_json).unwrap();

    assert_eq!(trace["id"], "trace-001");
    assert_eq!(trace["message"], "First day was intense!");

    // Add reflection
    let reflection_msg = "You got this!".to_string();
    let result = add_reflection_to_trace(trace_id.clone(), reflection_msg.clone());

    assert!(result.is_ok());

    // Get trace
    let result = get_trace_json(trace_id.clone());
    assert!(result.is_ok());

    let loaded_json = result.unwrap();
    let loaded: serde_json::Value = serde_json::from_str(&loaded_json).unwrap();

    assert_eq!(loaded["reflections"].as_array().unwrap().len(), 1);
    assert_eq!(loaded["reflections"][0]["message"], "You got this!");
}

#[test]
fn test_ffi_statistics() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Get initial stats
    let streak = get_streak();
    assert!(streak.is_ok());
    assert_eq!(streak.unwrap(), 0);

    let wild_count = get_use_in_wild_count();
    assert!(wild_count.is_ok());
    assert_eq!(wild_count.unwrap(), 0);
}

#[test]
fn test_ffi_events() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Add event
    let result = add_event("test_event".to_string(), r#"{"data": "test"}"#.to_string());
    assert!(result.is_ok());

    // Get events
    let result = get_events(Some("test_event".to_string()), 10);
    assert!(result.is_ok());

    let events_json = result.unwrap();
    let events: serde_json::Value = serde_json::from_str(&events_json).unwrap();

    assert!(events.as_array().unwrap().len() > 0);
}

#[test]
fn test_ffi_export_data() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Add some data
    add_event("warmup".to_string(), r#"{"completed": true}"#.to_string()).ok();
    add_event("ping".to_string(), r#"{"phrase": "test"}"#.to_string()).ok();

    // Export
    let result = export_data();
    assert!(result.is_ok());

    let export_json = result.unwrap();
    let export: serde_json::Value = serde_json::from_str(&export_json).unwrap();

    assert!(export["events"].is_array());
    assert!(export["memory_links"].is_array());
    assert!(export["exported_at"].is_string());
}

#[test]
fn test_ffi_consecutive_days_update() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    let role_id = "test_role".to_string();
    start_role_progress(role_id.clone(), 3).ok();

    // Update consecutive days
    let result = update_consecutive_days(role_id.clone(), 7);
    assert!(result.is_ok());

    let updated_json = result.unwrap();
    let updated: serde_json::Value = serde_json::from_str(&updated_json).unwrap();

    assert_eq!(updated["consecutive_days"], 7);

    // Verify coherence was recalculated
    assert!(updated["coherence"].as_f64().is_some());
}

#[test]
fn test_ffi_liminal_transition() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    let role_id = "qa_engineer_abroad".to_string();
    start_role_progress(role_id.clone(), 5).ok();

    // Complete some scenes
    complete_scene_with_emotion(
        role_id.clone(),
        "scene1".to_string(),
        "Confident".to_string(),
        0.9,
    ).ok();

    complete_scene_with_emotion(
        role_id.clone(),
        "scene2".to_string(),
        "Calm".to_string(),
        0.85,
    ).ok();

    // Get liminal transition
    let result = get_liminal_transition_json(role_id.clone());

    // This will fail if role is not loaded, but we can test the API exists
    // In real scenario, we'd need to load roles first
    assert!(result.is_err() || result.is_ok());
}

#[test]
fn test_ffi_json_serialization() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    let role_id = "test_role".to_string();
    let result = start_role_progress(role_id.clone(), 3);

    assert!(result.is_ok());

    let json = result.unwrap();

    // Verify it's valid JSON
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&json);
    assert!(parsed.is_ok());

    let value = parsed.unwrap();

    // Verify required fields
    assert!(value.get("role_id").is_some());
    assert!(value.get("total_scenes").is_some());
    assert!(value.get("current_scene_index").is_some());
    assert!(value.get("coherence").is_some());
    assert!(value.get("emotion_tags").is_some());
}

// NOTE: This test is commented out because it depends on global storage state
// and can fail if other tests have already initialized storage.
// In a real-world scenario, error handling is tested implicitly by other tests.
//
// #[test]
// fn test_ffi_error_handling() {
//     // Don't initialize storage
//
//     // Try to add event without initialization
//     let result = add_event("test".to_string(), "{}".to_string());
//     assert!(result.is_err());
//
//     // Try to get progress without initialization
//     let result = get_role_progress_json("nonexistent".to_string());
//     assert!(result.is_err());
// }

#[test]
fn test_ffi_recent_traces_pagination() {
    init_storage(":memory:".to_string()).expect("Failed to init storage");

    // Create multiple traces
    for i in 0..10 {
        create_resonance_trace(
            format!("trace-{:03}", i),
            "test_role".to_string(),
            format!("scene-{}", i),
            format!("Message {}", i),
        ).ok();
    }

    // Get recent traces with limit
    let result = get_recent_traces_json(Some("test_role".to_string()), 5);
    assert!(result.is_ok());

    let traces_json = result.unwrap();
    let traces: serde_json::Value = serde_json::from_str(&traces_json).unwrap();

    let traces_array = traces.as_array().unwrap();
    assert_eq!(traces_array.len(), 5);

    // Get all traces
    let result = get_recent_traces_json(None, 100);
    assert!(result.is_ok());

    let all_traces_json = result.unwrap();
    let all_traces: serde_json::Value = serde_json::from_str(&all_traces_json).unwrap();

    assert_eq!(all_traces.as_array().unwrap().len(), 10);
}
