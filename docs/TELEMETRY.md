# Telemetry & Analytics

Privacy-first telemetry system for tracking user behavior and app performance.

## Architecture

```
┌─────────────────┐
│   Flutter App   │
│  (UI Events)    │
└────────┬────────┘
         │ FFI
         ▼
┌─────────────────┐      ┌──────────────────┐
│  Rust Core API  │─────>│  SQLite Storage  │
│  (track_event)  │      │  (event queue)   │
└─────────────────┘      └──────────────────┘
         │                        │
         │                        │
         ▼                        ▼
┌─────────────────┐      ┌──────────────────┐
│  Event Batch    │─────>│  PostHog/Cloud   │
│  (local queue)  │      │  (future)        │
└─────────────────┘      └──────────────────┘
```

## Event Types

### Session Events
- `session_start` - User opens app
- `session_end` - User closes app

### Scenario Events
- `scenario_start` - Begin scenario
- `scenario_complete` - Finish scenario successfully
- `scenario_abandoned` - Exit without completing
- `step_complete` - Complete individual step

### Learning Events
- `phrase_reviewed` - Practice a phrase
- `phrase_marked_wild` - Mark phrase as used in real life
- `retention_wave_updated` - Memory wave update

### Voice Events
- `voice_practice` - User practices pronunciation
- `pronunciation_score` - Score from pronunciation check

### UI Events
- `screen_view` - Navigate to screen
- `button_click` - User interaction

### Progress Events
- `streak_achieved` - Daily streak milestone
- `role_completed` - Complete role path

### System Events
- `error` - App error occurred
- `crash_report` - App crashed

## Rust API

### Track Event

```rust
use liminal_english_core::{TelemetryEvent, EventType, DeviceContext};

let context = DeviceContext {
    app_version: "1.0.0".to_string(),
    platform: "android".to_string(),
    os_version: Some("14".to_string()),
    device_model: Some("Pixel 8".to_string()),
    locale: "en-US".to_string(),
};

let event = TelemetryEvent::new(EventType::SessionStart, context)
    .with_property("source", "home_screen")
    .with_session("session-123")
    .with_user("user-hash-456");

store.add_telemetry_event(&event)?;
```

### Get Pending Events

```rust
let pending = store.get_pending_events(50)?;
println!("Pending events: {}", pending.len());
```

### Create and Send Batch

```rust
let events = store.get_pending_events(50)?;
let batch = EventBatch::new(events);

store.save_batch(&batch)?;

// Send to server...
// send_to_posthog(&batch)?;

store.mark_batch_sent(&batch.batch_id)?;
```

### Get Stats

```rust
let stats = store.get_telemetry_stats()?;
println!("Pending: {}", stats.pending_events);
println!("Batched: {}", stats.batched_events);
println!("Sent batches: {}", stats.sent_batches);
```

### Cleanup Old Data

```rust
// Delete events older than 30 days
let deleted = store.cleanup_old_telemetry(30)?;
println!("Deleted {} old events", deleted);
```

## Flutter API

### Track Event

```dart
import 'package:liminal_app/bridge/bridge.generated.dart';

// Initialize context once
final context = {
  'app_version': '1.0.0',
  'platform': 'android',
  'os_version': '14',
  'device_model': 'Pixel 8',
  'locale': 'en-US',
};

// Track event
await trackEvent(
  'scenario_complete',
  sessionId: sessionManager.currentSessionId,
  userId: userManager.anonymousUserId,
  propertiesJson: jsonEncode({
    'scenario_id': 'morning-warmup-01',
    'duration_seconds': 180,
    'score': 85,
  }),
  contextJson: jsonEncode(context),
);
```

### Get Stats

```dart
final statsJson = await getTelemetryStats();
final stats = jsonDecode(statsJson);

print('Pending events: ${stats['pending_events']}');
print('Sent batches: ${stats['sent_batches']}');
```

### Batch and Send

```dart
// Get pending events
final eventsJson = await getPendingTelemetryEvents(50);
final events = jsonDecode(eventsJson);

if (events.isNotEmpty) {
  // Create batch
  final batchJson = await createTelemetryBatch(eventsJson);
  final batch = jsonDecode(batchJson);

  // Send to server
  try {
    await sendToPostHog(batch);
    await markTelemetryBatchSent(batch['batch_id']);
  } catch (e) {
    print('Failed to send batch: $e');
    // Batch remains in 'pending' state, will retry later
  }
}
```

### Background Sync

```dart
// Run every hour
Timer.periodic(Duration(hours: 1), (_) async {
  if (await isWifiConnected()) {
    await syncTelemetry();
  }
});

Future<void> syncTelemetry() async {
  try {
    final eventsJson = await getPendingTelemetryEvents(100);
    final events = jsonDecode(eventsJson);

    if (events.isEmpty) return;

    final batchJson = await createTelemetryBatch(eventsJson);
    final batch = jsonDecode(batchJson);

    await sendToPostHog(batch);
    await markTelemetryBatchSent(batch['batch_id']);

    print('✅ Synced ${events.length} events');
  } catch (e) {
    print('❌ Sync failed: $e');
  }
}
```

## Configuration

### TelemetryConfig

```rust
use liminal_english_core::TelemetryConfig;

let config = TelemetryConfig {
    enabled: true,
    batch_size: 50,                // Events per batch
    batch_max_age_seconds: 3600,   // 1 hour
    batch_max_size_bytes: 100_000, // 100KB
    wifi_only: true,               // Only send on WiFi
    endpoint: Some("https://posthog.example.com".to_string()),
};
```

### Privacy Settings

- **User ID**: Always anonymized hash
- **Session ID**: Temporary, rotates per session
- **Device Model**: Optional, can be omitted
- **Local Storage**: Events stored locally, never sent without consent
- **Opt-out**: Set `TelemetryConfig.enabled = false`

## Database Schema

### telemetry_events

```sql
CREATE TABLE telemetry_events(
  id TEXT PRIMARY KEY,           -- UUID
  event_type TEXT NOT NULL,      -- session_start, etc.
  timestamp TEXT NOT NULL,       -- ISO 8601
  session_id TEXT,               -- Optional session
  user_id TEXT,                  -- Anonymized hash
  properties TEXT NOT NULL,      -- JSON
  context TEXT NOT NULL,         -- JSON (device info)
  status TEXT NOT NULL DEFAULT 'pending'  -- pending/batched/sent
);
```

### telemetry_batches

```sql
CREATE TABLE telemetry_batches(
  batch_id TEXT PRIMARY KEY,
  created_at TEXT NOT NULL,
  sent_at TEXT,
  size_bytes INTEGER NOT NULL,
  event_count INTEGER NOT NULL,
  status TEXT NOT NULL DEFAULT 'pending'
);
```

## Best Practices

### 1. Track Important Events Only

```dart
// ✅ Good - tracks user intent
await trackEvent('scenario_complete', ...);

// ❌ Bad - too granular, creates noise
await trackEvent('button_hover', ...);
```

### 2. Add Context to Events

```dart
await trackEvent(
  'scenario_complete',
  propertiesJson: jsonEncode({
    'scenario_id': 'morning-warmup-01',
    'duration_seconds': 180,
    'score': 85,
    'num_retries': 2,
    'completion_rate': 1.0,
  }),
  ...
);
```

### 3. Batch Events Efficiently

```dart
// Sync when:
// - WiFi is connected
// - Battery is not low
// - App is in foreground
// - 50+ pending events OR 1 hour elapsed

if (canSync()) {
  await syncTelemetry();
}
```

### 4. Handle Failures Gracefully

```dart
try {
  await syncTelemetry();
} catch (e) {
  // Don't crash app, events remain in queue
  log('Telemetry sync failed: $e');
}
```

### 5. Cleanup Regularly

```dart
// Weekly cleanup
await cleanupOldTelemetry(30); // Remove events older than 30 days
```

## Future Enhancements

- [ ] PostHog integration
- [ ] Real-time streaming for critical events
- [ ] Offline queue with retry logic
- [ ] Event sampling for high-frequency events
- [ ] User consent management UI
- [ ] GDPR compliance tools

## Testing

### Unit Tests

```bash
cd core
cargo test test_telemetry
```

### Integration Tests

```bash
cargo test --tests -- --test-threads=1
```

### Manual Testing

```dart
// 1. Track event
await trackEvent('test_event', ...);

// 2. Check stats
final stats = await getTelemetryStats();
expect(stats['pending_events'], 1);

// 3. Create batch
final batch = await createTelemetryBatch(...);

// 4. Verify batch
expect(batch['event_count'], 1);
```

## Monitoring

### Key Metrics

- **Pending Events**: Should stay < 100
- **Batched Events**: Should decrease after sync
- **Sent Batches**: Should increase over time
- **Failed Batches**: Should be rare (< 5%)

### Dashboard Queries (PostHog)

```sql
-- Event volume by type
SELECT event_type, COUNT(*)
FROM telemetry_events
GROUP BY event_type
ORDER BY COUNT(*) DESC;

-- Daily active users
SELECT DATE(timestamp), COUNT(DISTINCT user_id)
FROM telemetry_events
WHERE event_type = 'session_start'
GROUP BY DATE(timestamp);

-- Average session duration
SELECT AVG(duration_seconds)
FROM telemetry_events
WHERE event_type = 'session_end'
  AND properties->>'duration_seconds' IS NOT NULL;
```

## Support

For questions or issues:
- Check [ARCHITECTURE.md](./ARCHITECTURE.md) for system overview
- See [ISSUES.md](../ISSUES.md) for known issues
- Open GitHub issue for bugs
