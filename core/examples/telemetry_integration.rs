/// Telemetry Integration Examples
///
/// This file demonstrates how to integrate telemetry tracking
/// into the English-Liminal app at various points.

use liminal_english_core::{
    DeviceContext, EventBatch, EventType, Store, TelemetryConfig, TelemetryEvent,
};

/// Example 1: Track session start
pub fn track_session_start(
    store: &Store,
    session_id: &str,
    user_id: &str,
    context: DeviceContext,
) -> anyhow::Result<()> {
    let event = TelemetryEvent::new(EventType::SessionStart, context)
        .with_session(session_id)
        .with_user(user_id)
        .with_property("entry_point", "app_icon");

    store.add_telemetry_event(&event)?;
    Ok(())
}

/// Example 2: Track scenario completion
pub fn track_scenario_complete(
    store: &Store,
    session_id: &str,
    scenario_id: &str,
    duration_seconds: u32,
    score: f32,
    context: DeviceContext,
) -> anyhow::Result<()> {
    let event = TelemetryEvent::new(EventType::ScenarioComplete, context)
        .with_session(session_id)
        .with_property("scenario_id", scenario_id)
        .with_property("duration_seconds", duration_seconds)
        .with_property("score", score)
        .with_property("completion_rate", 1.0);

    store.add_telemetry_event(&event)?;
    Ok(())
}

/// Example 3: Track phrase marked as used in wild
pub fn track_phrase_wild(
    store: &Store,
    session_id: &str,
    phrase: &str,
    context: DeviceContext,
) -> anyhow::Result<()> {
    let event = TelemetryEvent::new(EventType::PhraseMarkedWild, context)
        .with_session(session_id)
        .with_property("phrase", phrase)
        .with_property("source", "lesson");

    store.add_telemetry_event(&event)?;
    Ok(())
}

/// Example 4: Track role completion
pub fn track_role_complete(
    store: &Store,
    session_id: &str,
    role_id: &str,
    coherence_score: f32,
    consecutive_days: u32,
    context: DeviceContext,
) -> anyhow::Result<()> {
    let event = TelemetryEvent::new(EventType::RoleCompleted, context)
        .with_session(session_id)
        .with_property("role_id", role_id)
        .with_property("coherence_score", coherence_score)
        .with_property("consecutive_days", consecutive_days);

    store.add_telemetry_event(&event)?;
    Ok(())
}

/// Example 5: Track error
pub fn track_error(
    store: &Store,
    session_id: &str,
    error_type: &str,
    error_message: &str,
    context: DeviceContext,
) -> anyhow::Result<()> {
    let event = TelemetryEvent::new(EventType::Error, context)
        .with_session(session_id)
        .with_property("error_type", error_type)
        .with_property("error_message", error_message)
        .with_property("severity", "error");

    store.add_telemetry_event(&event)?;
    Ok(())
}

/// Example 6: Batch and send workflow
pub fn batch_and_send_workflow(store: &Store) -> anyhow::Result<()> {
    // 1. Get pending events
    let pending = store.get_pending_events(50)?;

    if pending.is_empty() {
        println!("No pending events to send");
        return Ok(());
    }

    // 2. Create batch
    let batch = EventBatch::new(pending);
    store.save_batch(&batch)?;

    println!("Created batch {} with {} events", batch.batch_id, batch.events.len());

    // 3. Send to server (pseudocode)
    // match send_to_posthog(&batch).await {
    //     Ok(_) => {
    //         store.mark_batch_sent(&batch.batch_id)?;
    //         println!("✅ Batch sent successfully");
    //     }
    //     Err(e) => {
    //         println!("❌ Failed to send batch: {}", e);
    //         // Batch remains in 'pending' state
    //     }
    // }

    Ok(())
}

/// Example 7: Background sync with config
pub fn background_sync_with_config(
    store: &Store,
    config: &TelemetryConfig,
    is_wifi: bool,
) -> anyhow::Result<()> {
    // Check if telemetry is enabled
    if !config.enabled {
        println!("Telemetry disabled by user");
        return Ok(());
    }

    // Check WiFi requirement
    if config.wifi_only && !is_wifi {
        println!("Skipping sync - WiFi only mode enabled");
        return Ok(());
    }

    // Get stats
    let stats = store.get_telemetry_stats()?;
    println!("Telemetry stats: {:?}", stats);

    // Batch if we have enough events
    if stats.pending_events >= config.batch_size {
        let events = store.get_pending_events(config.batch_size)?;
        let batch = EventBatch::new(events);

        // Check batch size
        if batch.size_bytes > config.batch_max_size_bytes {
            println!("⚠️  Batch too large: {} bytes", batch.size_bytes);
            // Split into smaller batches
            return Ok(());
        }

        store.save_batch(&batch)?;
        println!("📦 Batched {} events", batch.events.len());

        // Send batch (pseudocode)
        // if let Some(endpoint) = &config.endpoint {
        //     send_to_endpoint(endpoint, &batch).await?;
        //     store.mark_batch_sent(&batch.batch_id)?;
        // }
    }

    Ok(())
}

/// Example 8: Cleanup old telemetry
pub fn cleanup_old_events(store: &Store, days: i64) -> anyhow::Result<()> {
    let deleted = store.cleanup_old_telemetry(days)?;
    println!("🗑️  Deleted {} old telemetry events", deleted);
    Ok(())
}

/// Example 9: Get telemetry health status
pub fn get_telemetry_health(store: &Store) -> anyhow::Result<String> {
    let stats = store.get_telemetry_stats()?;

    let status = if stats.pending_events > 100 {
        "⚠️  WARNING: High pending event count"
    } else if stats.pending_events > 50 {
        "⚡ ATTENTION: Moderate pending events"
    } else {
        "✅ HEALTHY: Normal operation"
    };

    Ok(format!(
        "{}\n\
         Pending: {} events\n\
         Batched: {} events\n\
         Total batches: {}\n\
         Sent batches: {}",
        status, stats.pending_events, stats.batched_events, stats.total_batches, stats.sent_batches
    ))
}

/// Example 10: Full integration example
pub fn full_integration_example() -> anyhow::Result<()> {
    // Initialize storage
    let store = Store::open("app.db")?;

    // Create device context
    let context = DeviceContext {
        app_version: "1.0.0".to_string(),
        platform: "android".to_string(),
        os_version: Some("14".to_string()),
        device_model: Some("Pixel 8".to_string()),
        locale: "en-US".to_string(),
    };

    // Session ID (generate per app session)
    // In production, use uuid crate: uuid::Uuid::new_v4().to_string()
    let session_id = format!("session-{}", std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis());

    // User ID (anonymized hash)
    let user_id = "user_hash_123456";

    println!("📱 App started - Session: {}", session_id);

    // Track session start
    track_session_start(&store, &session_id, user_id, context.clone())?;

    // Simulate user completing a scenario
    println!("🎭 User completes scenario");
    track_scenario_complete(
        &store,
        &session_id,
        "morning-warmup-01",
        180, // 3 minutes
        85.5,
        context.clone(),
    )?;

    // User marks phrase as used
    println!("⭐ User marks phrase as used in real life");
    track_phrase_wild(&store, &session_id, "How's it going?", context.clone())?;

    // Check stats
    let health = get_telemetry_health(&store)?;
    println!("\n{}", health);

    // Simulate background sync (every hour)
    println!("\n⏰ Background sync triggered");
    let config = TelemetryConfig::default();
    background_sync_with_config(&store, &config, true)?;

    // Weekly cleanup
    println!("\n🗑️  Weekly cleanup");
    cleanup_old_events(&store, 30)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_track_session_start() {
        let store = Store::open(":memory:").unwrap();
        let context = DeviceContext::default();

        track_session_start(&store, "session-123", "user-456", context).unwrap();

        let events = store.get_pending_events(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::SessionStart);
    }

    #[test]
    fn test_track_scenario_complete() {
        let store = Store::open(":memory:").unwrap();
        let context = DeviceContext::default();

        track_scenario_complete(&store, "session-123", "scenario-01", 120, 90.0, context).unwrap();

        let events = store.get_pending_events(10).unwrap();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, EventType::ScenarioComplete);
    }

    #[test]
    fn test_batch_workflow() {
        let store = Store::open(":memory:").unwrap();
        let context = DeviceContext::default();

        // Add 5 events
        for i in 0..5 {
            track_scenario_complete(
                &store,
                "session-123",
                &format!("scenario-{}", i),
                120,
                85.0,
                context.clone(),
            )
            .unwrap();
        }

        // Batch and send
        batch_and_send_workflow(&store).unwrap();

        // Verify pending is now 0
        let events = store.get_pending_events(10).unwrap();
        assert_eq!(events.len(), 0);

        let stats = store.get_telemetry_stats().unwrap();
        assert_eq!(stats.batched_events, 5);
    }
}

/// Main function for running examples
fn main() -> anyhow::Result<()> {
    println!("🚀 Telemetry Integration Examples\n");

    // Run full integration example
    full_integration_example()?;

    println!("\n✅ All examples completed successfully!");
    Ok(())
}
