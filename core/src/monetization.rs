use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Subscription tier levels
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionTier {
    /// Free tier - basic content only
    Free,
    /// Premium monthly subscription
    PremiumMonthly,
    /// Premium yearly subscription (discounted)
    PremiumYearly,
    /// Lifetime access
    Lifetime,
}

impl SubscriptionTier {
    /// Check if this tier includes premium features
    pub fn is_premium(&self) -> bool {
        matches!(
            self,
            SubscriptionTier::PremiumMonthly
                | SubscriptionTier::PremiumYearly
                | SubscriptionTier::Lifetime
        )
    }

    /// Get tier priority (higher = more access)
    pub fn priority(&self) -> u8 {
        match self {
            SubscriptionTier::Free => 0,
            SubscriptionTier::PremiumMonthly => 10,
            SubscriptionTier::PremiumYearly => 10,
            SubscriptionTier::Lifetime => 20,
        }
    }
}

/// Subscription status
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionStatus {
    /// Active subscription
    Active,
    /// Expired subscription
    Expired,
    /// Cancelled (still valid until expiry)
    Cancelled,
    /// Grace period after payment failure
    GracePeriod,
    /// Trial period
    Trial,
}

/// User subscription record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Subscription {
    pub id: String,
    pub user_id: String,
    pub tier: SubscriptionTier,
    pub status: SubscriptionStatus,
    pub started_at: DateTime<Utc>,
    pub expires_at: Option<DateTime<Utc>>,
    pub cancelled_at: Option<DateTime<Utc>>,
    pub platform: Platform,
    pub transaction_id: Option<String>,
}

impl Subscription {
    /// Create a new free subscription
    pub fn new_free(user_id: String) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            tier: SubscriptionTier::Free,
            status: SubscriptionStatus::Active,
            started_at: Utc::now(),
            expires_at: None,
            cancelled_at: None,
            platform: Platform::Direct,
            transaction_id: None,
        }
    }

    /// Check if subscription is currently active
    pub fn is_active(&self) -> bool {
        if self.status != SubscriptionStatus::Active
            && self.status != SubscriptionStatus::Trial
            && self.status != SubscriptionStatus::Cancelled
        {
            return false;
        }

        if let Some(expires) = self.expires_at {
            Utc::now() < expires
        } else {
            true
        }
    }

    /// Check if user has premium access
    pub fn has_premium_access(&self) -> bool {
        self.is_active() && self.tier.is_premium()
    }
}

/// Purchase platform
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum Platform {
    /// iOS App Store
    AppStore,
    /// Google Play Store
    PlayStore,
    /// Direct purchase (web/other)
    Direct,
}

/// In-app purchase record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Purchase {
    pub id: String,
    pub user_id: String,
    pub product_id: String,
    pub platform: Platform,
    pub transaction_id: String,
    pub purchased_at: DateTime<Utc>,
    pub price_cents: i64,
    pub currency: String,
    pub verified: bool,
    pub metadata: HashMap<String, serde_json::Value>,
}

impl Purchase {
    pub fn new(
        user_id: String,
        product_id: String,
        platform: Platform,
        transaction_id: String,
        price_cents: i64,
        currency: String,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            product_id,
            platform,
            transaction_id,
            purchased_at: Utc::now(),
            price_cents,
            currency,
            verified: false,
            metadata: HashMap::new(),
        }
    }
}

/// Content access level
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentAccess {
    /// Free - available to all users
    Free,
    /// Premium - requires active premium subscription
    Premium,
    /// Unlockable - can be purchased individually
    Unlockable,
}

/// Content unlock record (for individual role/scenario purchases)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentUnlock {
    pub id: String,
    pub user_id: String,
    pub content_type: ContentType,
    pub content_id: String,
    pub unlocked_at: DateTime<Utc>,
    pub purchase_id: Option<String>,
}

impl ContentUnlock {
    pub fn new(
        user_id: String,
        content_type: ContentType,
        content_id: String,
        purchase_id: Option<String>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            content_type,
            content_id,
            unlocked_at: Utc::now(),
            purchase_id,
        }
    }
}

/// Type of unlockable content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ContentType {
    /// Full role with all scenarios
    Role,
    /// Individual scenario
    Scenario,
    /// Voice pack
    VoicePack,
}

/// Entitlement check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entitlement {
    pub has_access: bool,
    pub reason: EntitlementReason,
    pub subscription: Option<Subscription>,
}

impl Entitlement {
    pub fn granted(reason: EntitlementReason, subscription: Option<Subscription>) -> Self {
        Self {
            has_access: true,
            reason,
            subscription,
        }
    }

    pub fn denied(reason: EntitlementReason) -> Self {
        Self {
            has_access: false,
            reason,
            subscription: None,
        }
    }
}

/// Reason for entitlement decision
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum EntitlementReason {
    /// Content is free for all users
    FreeContent,
    /// User has active premium subscription
    PremiumSubscription,
    /// Content individually unlocked
    IndividualUnlock,
    /// Trial access
    TrialAccess,
    /// No access - requires premium
    RequiresPremium,
    /// No access - requires unlock
    RequiresUnlock,
    /// No access - subscription expired
    SubscriptionExpired,
}

/// Monetization configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonetizationConfig {
    /// Enable in-app purchases
    pub iap_enabled: bool,
    /// Trial duration in days
    pub trial_days: u32,
    /// Grace period after payment failure (days)
    pub grace_period_days: u32,
    /// Product IDs by tier
    pub product_ids: HashMap<SubscriptionTier, String>,
}

impl Default for MonetizationConfig {
    fn default() -> Self {
        let mut product_ids = HashMap::new();
        product_ids.insert(
            SubscriptionTier::PremiumMonthly,
            "liminal_premium_monthly".to_string(),
        );
        product_ids.insert(
            SubscriptionTier::PremiumYearly,
            "liminal_premium_yearly".to_string(),
        );
        product_ids.insert(SubscriptionTier::Lifetime, "liminal_lifetime".to_string());

        Self {
            iap_enabled: true,
            trial_days: 7,
            grace_period_days: 3,
            product_ids,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_tiers() {
        assert!(!SubscriptionTier::Free.is_premium());
        assert!(SubscriptionTier::PremiumMonthly.is_premium());
        assert!(SubscriptionTier::PremiumYearly.is_premium());
        assert!(SubscriptionTier::Lifetime.is_premium());

        assert!(
            SubscriptionTier::Lifetime.priority() > SubscriptionTier::PremiumMonthly.priority()
        );
        assert!(SubscriptionTier::PremiumMonthly.priority() > SubscriptionTier::Free.priority());
    }

    #[test]
    fn test_free_subscription() {
        let sub = Subscription::new_free("user123".to_string());
        assert_eq!(sub.tier, SubscriptionTier::Free);
        assert_eq!(sub.status, SubscriptionStatus::Active);
        assert!(sub.is_active());
        assert!(!sub.has_premium_access());
    }

    #[test]
    fn test_premium_subscription() {
        let mut sub = Subscription::new_free("user123".to_string());
        sub.tier = SubscriptionTier::PremiumMonthly;
        sub.status = SubscriptionStatus::Active;
        sub.expires_at = Some(Utc::now() + chrono::Duration::days(30));

        assert!(sub.is_active());
        assert!(sub.has_premium_access());
    }

    #[test]
    fn test_expired_subscription() {
        let mut sub = Subscription::new_free("user123".to_string());
        sub.tier = SubscriptionTier::PremiumMonthly;
        sub.expires_at = Some(Utc::now() - chrono::Duration::days(1));

        assert!(!sub.is_active());
        assert!(!sub.has_premium_access());
    }

    #[test]
    fn test_content_unlock() {
        let unlock = ContentUnlock::new(
            "user123".to_string(),
            ContentType::Role,
            "qa_engineer_abroad".to_string(),
            Some("purchase123".to_string()),
        );

        assert_eq!(unlock.user_id, "user123");
        assert_eq!(unlock.content_type, ContentType::Role);
        assert_eq!(unlock.content_id, "qa_engineer_abroad");
    }

    #[test]
    fn test_entitlement_granted() {
        let ent = Entitlement::granted(EntitlementReason::FreeContent, None);
        assert!(ent.has_access);
        assert_eq!(ent.reason, EntitlementReason::FreeContent);
    }

    #[test]
    fn test_entitlement_denied() {
        let ent = Entitlement::denied(EntitlementReason::RequiresPremium);
        assert!(!ent.has_access);
        assert_eq!(ent.reason, EntitlementReason::RequiresPremium);
    }
}
