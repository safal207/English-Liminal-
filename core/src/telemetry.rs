use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Types of telemetry events we track
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
    // Session events
    SessionStart,
    SessionEnd,

    // Scenario events
    ScenarioStart,
    ScenarioComplete,
    ScenarioAbandoned,
    StepComplete,

    // Learning events
    PhraseReviewed,
    PhraseMarkedWild, // Used in real life
    RetentionWaveUpdated,

    // Voice events
    VoicePractice,
    PronunciationScore,

    // UI interactions
    ScreenView,
    ButtonClick,

    // Progress milestones
    StreakAchieved,
    RoleCompleted,

    // System events
    Error,
    CrashReport,
}

/// A single telemetry event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    /// Unique event ID
    pub id: String,

    /// Event type
    pub event_type: EventType,

    /// Event timestamp (UTC)
    pub timestamp: DateTime<Utc>,

    /// Session ID (for grouping events)
    pub session_id: Option<String>,

    /// User ID (anonymized hash)
    pub user_id: Option<String>,

    /// Event properties (flexible key-value data)
    pub properties: HashMap<String, serde_json::Value>,

    /// Device context
    pub context: DeviceContext,
}

/// Device and app context for events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceContext {
    /// App version (e.g., "1.0.0")
    pub app_version: String,

    /// Platform (iOS, Android, etc.)
    pub platform: String,

    /// OS version
    pub os_version: Option<String>,

    /// Device model (optional for privacy)
    pub device_model: Option<String>,

    /// Locale (e.g., "en-US")
    pub locale: String,
}

impl TelemetryEvent {
    /// Create a new telemetry event
    pub fn new(event_type: EventType, context: DeviceContext) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event_type,
            timestamp: Utc::now(),
            session_id: None,
            user_id: None,
            properties: HashMap::new(),
            context,
        }
    }

    /// Add a property to the event
    pub fn with_property(mut self, key: impl Into<String>, value: impl Into<serde_json::Value>) -> Self {
        self.properties.insert(key.into(), value.into());
        self
    }

    /// Set session ID
    pub fn with_session(mut self, session_id: impl Into<String>) -> Self {
        self.session_id = Some(session_id.into());
        self
    }

    /// Set user ID
    pub fn with_user(mut self, user_id: impl Into<String>) -> Self {
        self.user_id = Some(user_id.into());
        self
    }
}

impl Default for DeviceContext {
    fn default() -> Self {
        Self {
            app_version: env!("CARGO_PKG_VERSION").to_string(),
            platform: "unknown".to_string(),
            os_version: None,
            device_model: None,
            locale: "en-US".to_string(),
        }
    }
}

/// Batch of telemetry events for efficient transmission
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBatch {
    /// Batch ID
    pub batch_id: String,

    /// Events in this batch
    pub events: Vec<TelemetryEvent>,

    /// When the batch was created
    pub created_at: DateTime<Utc>,

    /// Batch size in bytes (for tracking)
    pub size_bytes: usize,
}

impl EventBatch {
    /// Create a new batch from events
    pub fn new(events: Vec<TelemetryEvent>) -> Self {
        let serialized = serde_json::to_string(&events).unwrap_or_default();
        let size_bytes = serialized.len();

        Self {
            batch_id: uuid::Uuid::new_v4().to_string(),
            events,
            created_at: Utc::now(),
            size_bytes,
        }
    }

    /// Check if batch is ready to send (size or age based)
    pub fn is_ready_to_send(&self, max_age_seconds: i64, max_size_bytes: usize) -> bool {
        let age = Utc::now().signed_duration_since(self.created_at);
        age.num_seconds() >= max_age_seconds || self.size_bytes >= max_size_bytes
    }
}

/// Configuration for telemetry
#[derive(Debug, Clone)]
pub struct TelemetryConfig {
    /// Enable/disable telemetry
    pub enabled: bool,

    /// Batch size (number of events before sending)
    pub batch_size: usize,

    /// Max batch age in seconds before forcing send
    pub batch_max_age_seconds: i64,

    /// Max batch size in bytes
    pub batch_max_size_bytes: usize,

    /// Only send on WiFi
    pub wifi_only: bool,

    /// API endpoint (optional, for future PostHog integration)
    pub endpoint: Option<String>,
}

impl Default for TelemetryConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            batch_size: 50,
            batch_max_age_seconds: 3600, // 1 hour
            batch_max_size_bytes: 100_000, // 100KB
            wifi_only: true,
            endpoint: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_event() {
        let context = DeviceContext::default();
        let event = TelemetryEvent::new(EventType::SessionStart, context)
            .with_property("test_key", "test_value")
            .with_session("session-123")
            .with_user("user-456");

        assert_eq!(event.event_type, EventType::SessionStart);
        assert_eq!(event.session_id, Some("session-123".to_string()));
        assert_eq!(event.user_id, Some("user-456".to_string()));
        assert_eq!(
            event.properties.get("test_key"),
            Some(&serde_json::Value::String("test_value".to_string()))
        );
    }

    #[test]
    fn test_event_batch() {
        let context = DeviceContext::default();
        let events = vec![
            TelemetryEvent::new(EventType::SessionStart, context.clone()),
            TelemetryEvent::new(EventType::ScenarioStart, context.clone()),
        ];

        let batch = EventBatch::new(events);
        assert_eq!(batch.events.len(), 2);
        assert!(batch.size_bytes > 0);
    }

    #[test]
    fn test_batch_ready_to_send() {
        let context = DeviceContext::default();
        let events = vec![TelemetryEvent::new(EventType::SessionStart, context)];

        let batch = EventBatch::new(events);

        // Not ready immediately (age = 0, size small)
        assert!(!batch.is_ready_to_send(3600, 100_000));

        // Would be ready if max_age was 0
        assert!(batch.is_ready_to_send(0, 100_000));
    }
}
