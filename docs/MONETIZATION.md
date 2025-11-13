# Monetization System

Complete freemium monetization system with subscriptions, in-app purchases, and content unlocking.

## Table of Contents

- [Overview](#overview)
- [Architecture](#architecture)
- [Subscription Tiers](#subscription-tiers)
- [Content Access Levels](#content-access-levels)
- [Rust API](#rust-api)
- [Flutter Integration](#flutter-integration)
- [Database Schema](#database-schema)
- [Business Logic](#business-logic)
- [Testing](#testing)

## Overview

The monetization system implements a **freemium** model with multiple revenue streams:

1. **Free Tier** - Basic content available to all users
2. **Premium Subscription** - Monthly/Yearly subscriptions unlock all content
3. **Individual Unlocks** - Purchase specific roles/scenarios separately
4. **Lifetime Access** - One-time purchase for permanent access

### Key Features

- ✅ Multiple subscription tiers (Free, Monthly, Yearly, Lifetime)
- ✅ Platform support (iOS App Store, Google Play, SamCart, Direct)
- ✅ Content unlocking (roles, scenarios, voice packs)
- ✅ Entitlement checking (what can user access?)
- ✅ Purchase verification
- ✅ Grace periods and trial support
- ✅ Subscription cancellation with access retention
- ✅ Local-first storage with SQLite

## Architecture

```
┌─────────────────────┐
│   Flutter App       │
│  (IAP Platform)     │
└──────────┬──────────┘
           │
           │ FFI Bridge
           ▼
┌─────────────────────┐
│   Rust Core         │
│  - Subscriptions    │
│  - Purchases        │
│  - Entitlements     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│   SQLite Storage    │
│  - subscriptions    │
│  - purchases        │
│  - content_unlocks  │
└─────────────────────┘
```

### Data Flow

1. **Purchase** → Platform (App Store/Play Store/SamCart) → FFI → Rust → SQLite
2. **Entitlement Check** → Rust (checks subscription + unlocks) → Returns access decision
3. **Content Access** → App checks entitlement → Shows/Hides premium content

## Subscription Tiers

### Tier Definitions

```rust
pub enum SubscriptionTier {
    Free,              // Basic content only
    PremiumMonthly,    // $9.99/month
    PremiumYearly,     // $79.99/year (33% discount)
    Lifetime,          // $199.99 one-time
}
```

### Tier Comparison

| Feature | Free | Premium Monthly | Premium Yearly | Lifetime |
|---------|------|----------------|----------------|----------|
| Basic roles | ✅ | ✅ | ✅ | ✅ |
| Premium roles | ❌ | ✅ | ✅ | ✅ |
| Voice practice | ❌ | ✅ | ✅ | ✅ |
| Offline access | ❌ | ✅ | ✅ | ✅ |
| Priority support | ❌ | ✅ | ✅ | ✅ |
| **Price** | Free | $9.99/mo | $79.99/yr | $199.99 |

## Content Access Levels

```rust
pub enum ContentAccess {
    Free,        // Available to all users
    Premium,     // Requires active premium subscription
    Unlockable,  // Can be purchased individually OR accessed with premium
}
```

### Content Classification

**Free Content** (Always accessible):
- First 2 roles: `qa_engineer_abroad`, `visa_journey`
- Basic scenarios (5 per free role)
- Text-only practice mode

**Premium Content** (Subscription required):
- Advanced roles: `global_citizen`, `family_abroad`, `tech_startup`, etc.
- Voice practice with pronunciation scoring
- Unlimited scenarios
- Offline content download

**Unlockable Content** (Individual purchase OR premium):
- Individual premium roles ($4.99 each)
- Voice packs for specific roles ($2.99 each)
- Special scenario packs ($1.99 each)

## Rust API

### Core Types

```rust
// Subscription record
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

// Purchase record
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

// Content unlock record
pub struct ContentUnlock {
    pub id: String,
    pub user_id: String,
    pub content_type: ContentType,  // Role, Scenario, VoicePack
    pub content_id: String,
    pub unlocked_at: DateTime<Utc>,
    pub purchase_id: Option<String>,
}

// Entitlement check result
pub struct Entitlement {
    pub has_access: bool,
    pub reason: EntitlementReason,
    pub subscription: Option<Subscription>,
}
```

### Storage Methods

```rust
impl Store {
    // Subscriptions
    pub fn save_subscription(&self, subscription: &Subscription) -> Result<()>;
    pub fn get_user_subscription(&self, user_id: &str) -> Result<Option<Subscription>>;

    // Purchases
    pub fn save_purchase(&self, purchase: &Purchase) -> Result<()>;
    pub fn get_purchase_by_transaction(&self, transaction_id: &str) -> Result<Option<Purchase>>;
    pub fn verify_purchase(&self, purchase_id: &str) -> Result<()>;

    // Content Unlocks
    pub fn unlock_content(&self, unlock: &ContentUnlock) -> Result<()>;
    pub fn has_content_unlocked(&self, user_id: &str, content_type: &ContentType, content_id: &str) -> Result<bool>;
    pub fn get_user_unlocks(&self, user_id: &str) -> Result<Vec<ContentUnlock>>;

    // Entitlements
    pub fn check_entitlement(
        &self,
        user_id: &str,
        content_access: &ContentAccess,
        content_type: Option<&ContentType>,
        content_id: Option<&str>,
    ) -> Result<Entitlement>;
}
```

### Example: Check Entitlement

```rust
use liminal_english_core::{Store, ContentAccess, ContentType};

let store = Store::open("app.db")?;

// Check if user can access premium role
let entitlement = store.check_entitlement(
    "user_123",
    &ContentAccess::Premium,
    Some(&ContentType::Role),
    Some("global_citizen"),
)?;

if entitlement.has_access {
    println!("Access granted: {:?}", entitlement.reason);
    // EntitlementReason::PremiumSubscription or EntitlementReason::IndividualUnlock
} else {
    println!("Access denied: {:?}", entitlement.reason);
    // EntitlementReason::RequiresPremium
}
```

### Example: Process Subscription Purchase

```rust
use liminal_english_core::{Subscription, SubscriptionTier, SubscriptionStatus, Platform};
use chrono::{Utc, Duration};

// Create premium monthly subscription
let mut subscription = Subscription::new_free("user_123".to_string());
subscription.tier = SubscriptionTier::PremiumMonthly;
subscription.status = SubscriptionStatus::Active;
subscription.expires_at = Some(Utc::now() + Duration::days(30));
subscription.platform = Platform::AppStore;
subscription.transaction_id = Some("txn_abc123".to_string());

store.save_subscription(&subscription)?;

// Verify user now has premium access
let sub = store.get_user_subscription("user_123")?.unwrap();
assert!(sub.has_premium_access());
```

### Example: SamCart Web Purchase

```rust
// Process SamCart webhook payment
let mut subscription = Subscription::new_free("user_456".to_string());
subscription.tier = SubscriptionTier::PremiumYearly;
subscription.status = SubscriptionStatus::Active;
subscription.expires_at = Some(Utc::now() + Duration::days(365));
subscription.platform = Platform::SamCart;
subscription.transaction_id = Some("samcart_order_789xyz".to_string());

// Record the purchase separately for analytics
let purchase = Purchase::new(
    "user_456".to_string(),
    "premium_yearly_web".to_string(),
    Platform::SamCart,
    "samcart_order_789xyz".to_string(),
    7999, // $79.99
    "USD".to_string(),
);

store.save_subscription(&subscription)?;
store.save_purchase(&purchase)?;
store.verify_purchase(&purchase.id)?; // Mark as verified after webhook confirmation
```

## Flutter Integration

See `examples/flutter_monetization_service.dart` for complete implementation.

### Quick Start

```dart
// Initialize service
final monetization = MonetizationService();
await monetization.initialize(userId: 'user_123');

// Check subscription status
final subscription = await monetization.getUserSubscription();
if (subscription?.hasPremiumAccess ?? false) {
  print('User has premium!');
}

// Check specific content access
final entitlement = await monetization.checkContentAccess(
  contentAccess: ContentAccess.premium,
  contentType: ContentType.role,
  contentId: 'global_citizen',
);

if (entitlement.hasAccess) {
  // Show premium role
} else {
  // Show paywall
  showPaywall(reason: entitlement.reason);
}
```

### Handle iOS In-App Purchase

```dart
import 'package:in_app_purchase/in_app_purchase.dart';

Future<void> handlePurchase(PurchaseDetails purchase) async {
  if (purchase.status == PurchaseStatus.purchased) {
    // Start subscription in Rust
    await monetization.startPremiumSubscription(
      tier: SubscriptionTier.premiumMonthly,
      platform: Platform.appStore,
      transactionId: purchase.purchaseID!,
      duration: Duration(days: 30),
    );

    // Record purchase
    final purchaseRecord = Purchase(
      id: DateTime.now().millisecondsSinceEpoch.toString(),
      userId: monetization.userId!,
      productId: purchase.productID,
      platform: Platform.appStore,
      transactionId: purchase.purchaseID!,
      purchasedAt: DateTime.now(),
      priceCents: 999, // $9.99
      currency: 'USD',
      verified: true,
    );

    await monetization.recordPurchase(purchaseRecord);
  }
}
```

### Content Gating Example

```dart
class RoleCard extends StatelessWidget {
  final Role role;

  @override
  Widget build(BuildContext context) {
    return FutureBuilder<Entitlement>(
      future: monetization.checkContentAccess(
        contentAccess: role.accessLevel,
        contentType: ContentType.role,
        contentId: role.id,
      ),
      builder: (context, snapshot) {
        final entitlement = snapshot.data;
        final hasAccess = entitlement?.hasAccess ?? false;

        return Card(
          child: Column(
            children: [
              Text(role.name),
              if (!hasAccess)
                _buildLockedOverlay(entitlement?.reason),
              if (hasAccess)
                ElevatedButton(
                  onPressed: () => Navigator.push(...),
                  child: Text('Start'),
                ),
            ],
          ),
        );
      },
    );
  }

  Widget _buildLockedOverlay(EntitlementReason? reason) {
    switch (reason) {
      case EntitlementReason.requiresPremium:
        return ElevatedButton(
          onPressed: () => showPremiumDialog(),
          child: Text('Upgrade to Premium'),
        );
      case EntitlementReason.requiresUnlock:
        return ElevatedButton(
          onPressed: () => purchaseRole(role),
          child: Text('Unlock for \$4.99'),
        );
      default:
        return Icon(Icons.lock);
    }
  }
}
```

## Database Schema

### subscriptions

```sql
CREATE TABLE subscriptions(
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  tier TEXT NOT NULL,                -- "free", "premium_monthly", "premium_yearly", "lifetime"
  status TEXT NOT NULL,              -- "active", "expired", "cancelled", "grace_period", "trial"
  started_at TEXT NOT NULL,
  expires_at TEXT,                   -- NULL for lifetime
  cancelled_at TEXT,
  platform TEXT NOT NULL,            -- "app_store", "play_store", "direct"
  transaction_id TEXT
);

CREATE INDEX idx_subscriptions_user ON subscriptions(user_id);
```

### purchases

```sql
CREATE TABLE purchases(
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  product_id TEXT NOT NULL,
  platform TEXT NOT NULL,
  transaction_id TEXT NOT NULL UNIQUE,
  purchased_at TEXT NOT NULL,
  price_cents INTEGER NOT NULL,
  currency TEXT NOT NULL,
  verified INTEGER NOT NULL DEFAULT 0,
  metadata TEXT NOT NULL              -- JSON metadata
);

CREATE INDEX idx_purchases_user ON purchases(user_id);
```

### content_unlocks

```sql
CREATE TABLE content_unlocks(
  id TEXT PRIMARY KEY,
  user_id TEXT NOT NULL,
  content_type TEXT NOT NULL,        -- "role", "scenario", "voice_pack"
  content_id TEXT NOT NULL,
  unlocked_at TEXT NOT NULL,
  purchase_id TEXT,
  FOREIGN KEY(purchase_id) REFERENCES purchases(id)
);

CREATE INDEX idx_content_unlocks_user ON content_unlocks(user_id);
CREATE INDEX idx_content_unlocks_content ON content_unlocks(content_type, content_id);
```

## Business Logic

### Entitlement Decision Tree

```
check_entitlement(user_id, content_access, content_type, content_id):

  if content_access == Free:
    return GRANTED (FreeContent)

  subscription = get_user_subscription(user_id)

  if content_access == Premium:
    if subscription.has_premium_access():
      return GRANTED (PremiumSubscription)
    else if subscription.status == Expired:
      return DENIED (SubscriptionExpired)
    else:
      return DENIED (RequiresPremium)

  if content_access == Unlockable:
    if subscription.has_premium_access():
      return GRANTED (PremiumSubscription)

    if has_content_unlocked(user_id, content_type, content_id):
      return GRANTED (IndividualUnlock)

    return DENIED (RequiresUnlock)
```

### Subscription Lifecycle

```
Free → Trial (7 days) → Active → [Cancelled] → Expired
                            ↓
                        GracePeriod (3 days) → Expired
```

1. **Trial**: User starts 7-day free trial
2. **Active**: Trial converted or direct subscription purchase
3. **Cancelled**: User cancels but retains access until `expires_at`
4. **GracePeriod**: Payment failed but user retains access for 3 days
5. **Expired**: No longer has access

### Pricing Strategy

**Monthly vs Yearly Discount**:
- Monthly: $9.99/month = $119.88/year
- Yearly: $79.99/year = **33% savings**

**Individual Role Pricing**:
- Premium role: $4.99
- Break-even: 16 roles = yearly subscription
- Most users need 4-5 roles → subscription better value

## Testing

### Unit Tests

Run tests with:
```bash
cd core
cargo test --lib
```

Key test coverage:
- ✅ `test_subscription_save_load` - CRUD operations
- ✅ `test_purchase_save_load` - Purchase recording and verification
- ✅ `test_content_unlock` - Content unlocking
- ✅ `test_entitlement_free_content` - Free content always accessible
- ✅ `test_entitlement_premium_subscription` - Premium subscribers get access
- ✅ `test_entitlement_individual_unlock` - Individual unlocks grant access
- ✅ `test_entitlement_denied` - Users without access are denied

### Integration Testing

Test full subscription flow:

```rust
#[test]
fn test_full_subscription_flow() {
    let store = Store::open(":memory:").unwrap();
    let user_id = "test_user";

    // 1. User starts as free
    let free_sub = Subscription::new_free(user_id.to_string());
    store.save_subscription(&free_sub).unwrap();
    assert!(!free_sub.has_premium_access());

    // 2. User purchases premium
    let mut premium = free_sub.clone();
    premium.tier = SubscriptionTier::PremiumMonthly;
    premium.expires_at = Some(Utc::now() + Duration::days(30));
    store.save_subscription(&premium).unwrap();

    // 3. Check access to premium content
    let entitlement = store.check_entitlement(
        user_id,
        &ContentAccess::Premium,
        None,
        None,
    ).unwrap();
    assert!(entitlement.has_access);

    // 4. User cancels
    premium.status = SubscriptionStatus::Cancelled;
    premium.cancelled_at = Some(Utc::now());
    store.save_subscription(&premium).unwrap();

    // 5. Still has access until expiry
    assert!(premium.is_active());
}
```

## Best Practices

### 1. Cache Subscription Status

```dart
// Cache for 5 minutes to avoid excessive DB queries
final cachedSub = await monetization.getUserSubscription();
// Uses cache if fresh
```

### 2. Handle Failures Gracefully

```dart
try {
  final entitlement = await monetization.checkContentAccess(...);
  // Use entitlement
} catch (e) {
  // Fall back to showing paywall or cached state
  print('Error checking access: $e');
  showPaywall();
}
```

### 3. Verify Purchases Server-Side

While local verification is supported, always verify purchases with Apple/Google servers in production:

```dart
// After successful IAP
await verifyWithApple(transactionId);
await monetization.verifyPurchase(purchaseId);
```

### 4. Clear Cache on Subscription Change

```dart
// After purchase or cancellation
await monetization.refreshSubscriptionStatus();
```

### 5. Track Revenue with Telemetry

```dart
await telemetry.trackEvent(
  eventType: EventType.purchase,
  properties: {
    'product_id': productId,
    'price_cents': priceCents,
    'currency': currency,
    'platform': platform.name,
  },
);
```

## Roadmap

- [ ] Family sharing support
- [ ] Gift subscriptions
- [ ] Promo codes
- [ ] Regional pricing
- [ ] Server-side receipt validation
- [ ] Subscription pause/resume
- [ ] Upgrade/downgrade flows
- [ ] Referral rewards
