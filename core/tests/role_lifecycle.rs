use liminal_english_core::{
    EmotionTag, RoleProgress, Store, ResonanceTrace, Reflection,
};
use chrono::Utc;

#[test]
fn test_complete_role_lifecycle() {
    // Setup: Create in-memory database
    let store = Store::open(":memory:").expect("Failed to open store");

    // Step 1: Start a new role
    let role_id = "qa_engineer_abroad";
    let total_scenes = 5;
    let mut progress = RoleProgress::new(role_id.to_string(), total_scenes);

    assert_eq!(progress.current_scene_index, 0);
    assert_eq!(progress.total_scenes, 5);
    assert_eq!(progress.coherence, 0.0);

    // Step 2: Save initial progress
    store.save_role_progress(&progress).expect("Failed to save progress");

    // Step 3: Complete first scene with emotion
    let emotion1 = EmotionTag::new(
        "qa-interview-01".to_string(),
        "Nervous".to_string(),
        0.7,
    );
    progress.complete_scene(emotion1);

    assert_eq!(progress.current_scene_index, 1);
    assert_eq!(progress.emotion_tags.len(), 1);

    // Save progress
    store.save_role_progress(&progress).expect("Failed to save progress");

    // Step 4: Complete more scenes with varying emotions
    let emotions = vec![
        ("qa-bugreport-02", "Confident", 0.85),
        ("qa-standup-03", "Calm", 0.92),
        ("qa-review-04", "Professional", 0.88),
        ("qa-relax-05", "Relaxed", 0.95),
    ];

    for (scene_id, tone, confidence) in emotions {
        let emotion = EmotionTag::new(
            scene_id.to_string(),
            tone.to_string(),
            confidence,
        );
        progress.complete_scene(emotion);
        store.save_role_progress(&progress).expect("Failed to save progress");
    }

    // Step 5: Verify final state
    assert_eq!(progress.current_scene_index, 5);
    assert_eq!(progress.emotion_tags.len(), 5);

    // Step 6: Calculate coherence
    progress.consecutive_days = 7;
    progress.calculate_coherence();

    assert!(progress.coherence > 0.0);
    assert!(progress.coherence <= 1.0);

    // Save final state with consecutive days
    store.save_role_progress(&progress).expect("Failed to save final progress");

    // Step 7: Load from database and verify
    let loaded = store
        .load_role_progress(role_id)
        .expect("Failed to load progress")
        .expect("Progress not found");

    assert_eq!(loaded.role_id, role_id);
    assert_eq!(loaded.current_scene_index, 5);
    assert_eq!(loaded.emotion_tags.len(), 5);
    assert_eq!(loaded.consecutive_days, 7);
    assert_eq!(loaded.coherence, progress.coherence);

    // Verify emotions were saved correctly
    assert_eq!(loaded.emotion_tags[0].scene_id, "qa-interview-01");
    assert_eq!(loaded.emotion_tags[0].tone, "Nervous");
    assert_eq!(loaded.emotion_tags[4].scene_id, "qa-relax-05");
    assert_eq!(loaded.emotion_tags[4].tone, "Relaxed");
}

#[test]
fn test_multiple_roles_isolation() {
    let store = Store::open(":memory:").expect("Failed to open store");

    // Create progress for two different roles
    let mut qa_progress = RoleProgress::new("qa_engineer_abroad".to_string(), 5);
    let mut visa_progress = RoleProgress::new("visa_journey".to_string(), 3);

    // Add emotions to QA role
    qa_progress.complete_scene(EmotionTag::new(
        "scene1".to_string(),
        "Confident".to_string(),
        0.9,
    ));
    qa_progress.complete_scene(EmotionTag::new(
        "scene2".to_string(),
        "Calm".to_string(),
        0.85,
    ));

    // Add emotions to Visa role
    visa_progress.complete_scene(EmotionTag::new(
        "visa-scene1".to_string(),
        "Nervous".to_string(),
        0.6,
    ));

    // Save both
    store.save_role_progress(&qa_progress).expect("Failed to save QA progress");
    store.save_role_progress(&visa_progress).expect("Failed to save Visa progress");

    // Load and verify isolation
    let loaded_qa = store
        .load_role_progress("qa_engineer_abroad")
        .expect("Failed to load QA")
        .expect("QA not found");

    let loaded_visa = store
        .load_role_progress("visa_journey")
        .expect("Failed to load Visa")
        .expect("Visa not found");

    assert_eq!(loaded_qa.emotion_tags.len(), 2);
    assert_eq!(loaded_visa.emotion_tags.len(), 1);
    assert_eq!(loaded_qa.emotion_tags[0].tone, "Confident");
    assert_eq!(loaded_visa.emotion_tags[0].tone, "Nervous");
}

#[test]
fn test_social_resonance_flow() {
    let store = Store::open(":memory:").expect("Failed to open store");

    // Create a resonance trace
    let trace = ResonanceTrace::new(
        "trace-001".to_string(),
        "qa_engineer_abroad".to_string(),
        "qa-interview-01".to_string(),
        "First interview was tough but I pushed through!".to_string(),
    );

    store.save_resonance_trace(&trace).expect("Failed to save trace");

    // Load trace
    let loaded_trace = store
        .load_resonance_trace("trace-001")
        .expect("Failed to load trace")
        .expect("Trace not found");

    assert_eq!(loaded_trace.id, "trace-001");
    assert_eq!(loaded_trace.role_id, "qa_engineer_abroad");
    assert_eq!(loaded_trace.message, "First interview was tough but I pushed through!");
    assert_eq!(loaded_trace.reflections.len(), 0);

    // Add reflections
    let mut trace_with_reflections = loaded_trace.clone();

    let reflection1 = Reflection::new(
        "trace-001".to_string(),
        "Same here! You're not alone.".to_string(),
    );

    let reflection2 = Reflection::new(
        "trace-001".to_string(),
        "Keep going, it gets easier!".to_string(),
    );

    trace_with_reflections.add_reflection(reflection1);
    trace_with_reflections.add_reflection(reflection2);

    store.save_resonance_trace(&trace_with_reflections).expect("Failed to save updated trace");

    // Reload and verify
    let final_trace = store
        .load_resonance_trace("trace-001")
        .expect("Failed to load trace")
        .expect("Trace not found");

    assert_eq!(final_trace.reflections.len(), 2);
    assert_eq!(final_trace.reflections[0].message, "Same here! You're not alone.");
    assert_eq!(final_trace.reflections[1].message, "Keep going, it gets easier!");
}

#[test]
fn test_coherence_calculation() {
    let mut progress = RoleProgress::new("test_role".to_string(), 5);

    // Initial coherence should be 0
    assert_eq!(progress.coherence, 0.0);

    // Complete 3 scenes
    for i in 0..3 {
        progress.complete_scene(EmotionTag::new(
            format!("scene-{}", i),
            "Confident".to_string(),
            0.9,
        ));
    }

    // Set consecutive days
    progress.consecutive_days = 5;

    // Calculate coherence
    progress.calculate_coherence();

    // Should be > 0 since we completed scenes
    assert!(progress.coherence > 0.0);
    assert!(progress.coherence <= 1.0);

    // Add more scenes
    progress.complete_scene(EmotionTag::new(
        "scene-3".to_string(),
        "Calm".to_string(),
        0.95,
    ));
    progress.complete_scene(EmotionTag::new(
        "scene-4".to_string(),
        "Professional".to_string(),
        0.92,
    ));

    progress.consecutive_days = 10;
    progress.calculate_coherence();

    // Coherence should increase with more completed scenes
    assert!(progress.coherence > 0.5);
}

#[test]
fn test_emotion_tag_persistence() {
    let store = Store::open(":memory:").expect("Failed to open store");
    let role_id = "test_role";

    let mut progress = RoleProgress::new(role_id.to_string(), 3);

    // Add emotions with different timestamps
    let now = Utc::now();

    let mut emotion1 = EmotionTag::new("scene1".to_string(), "Happy".to_string(), 0.9);
    emotion1.timestamp = now - chrono::Duration::hours(2);

    let mut emotion2 = EmotionTag::new("scene2".to_string(), "Calm".to_string(), 0.85);
    emotion2.timestamp = now - chrono::Duration::hours(1);

    let emotion3 = EmotionTag::new("scene3".to_string(), "Confident".to_string(), 0.95);

    progress.emotion_tags.push(emotion1);
    progress.emotion_tags.push(emotion2);
    progress.emotion_tags.push(emotion3);
    progress.current_scene_index = 3;

    store.save_role_progress(&progress).expect("Failed to save");

    // Load and verify timestamps are preserved
    let loaded = store
        .load_role_progress(role_id)
        .expect("Failed to load")
        .expect("Not found");

    assert_eq!(loaded.emotion_tags.len(), 3);

    // Verify chronological order is maintained
    assert!(loaded.emotion_tags[0].timestamp < loaded.emotion_tags[1].timestamp);
    assert!(loaded.emotion_tags[1].timestamp < loaded.emotion_tags[2].timestamp);
}
