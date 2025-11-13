import 'dart:async';
import 'dart:convert';
import 'package:flutter/foundation.dart';

/// Monetization service for handling subscriptions, purchases, and content access
///
/// This service wraps the Rust FFI calls for monetization features and provides
/// a Flutter-friendly API for subscription management, IAP processing, and content unlocking.
///
/// Example usage:
/// ```dart
/// final monetization = MonetizationService();
/// await monetization.initialize(userId: 'user_123');
///
/// // Check subscription status
/// final subscription = await monetization.getUserSubscription();
/// if (subscription != null && subscription.hasPremiumAccess) {
///   print('User has premium access!');
/// }
///
/// // Check content access
/// final canAccess = await monetization.checkContentAccess(
///   contentAccess: ContentAccess.premium,
///   contentType: ContentType.role,
///   contentId: 'qa_engineer_abroad',
/// );
/// ```
class MonetizationService {
  String? _userId;
  Subscription? _cachedSubscription;
  DateTime? _lastSubscriptionCheck;

  // Cache duration for subscription status
  static const _subscriptionCacheDuration = Duration(minutes: 5);

  /// Initialize the monetization service
  Future<void> initialize({required String userId}) async {
    _userId = userId;
    await refreshSubscriptionStatus();
  }

  /// Get current user ID
  String? get userId => _userId;

  /// Get cached subscription if available
  Subscription? get cachedSubscription => _cachedSubscription;

  /// Check if user has premium access (cached)
  bool get hasPremiumAccess {
    if (_cachedSubscription == null) return false;
    return _cachedSubscription!.hasPremiumAccess;
  }

  // ==========================================================================
  // Subscriptions
  // ==========================================================================

  /// Refresh subscription status from storage
  Future<Subscription?> refreshSubscriptionStatus() async {
    if (_userId == null) {
      throw StateError('MonetizationService not initialized');
    }

    try {
      // Call FFI
      final subscriptionJson = await getUserSubscriptionFfi(_userId!);
      if (subscriptionJson == null) {
        _cachedSubscription = null;
        _lastSubscriptionCheck = DateTime.now();
        return null;
      }

      final data = jsonDecode(subscriptionJson);
      _cachedSubscription = Subscription.fromJson(data);
      _lastSubscriptionCheck = DateTime.now();
      return _cachedSubscription;
    } catch (e) {
      print('Error refreshing subscription: $e');
      return null;
    }
  }

  /// Get user subscription (uses cache if fresh)
  Future<Subscription?> getUserSubscription({bool forceRefresh = false}) async {
    if (!forceRefresh && _cachedSubscription != null && _lastSubscriptionCheck != null) {
      final age = DateTime.now().difference(_lastSubscriptionCheck!);
      if (age < _subscriptionCacheDuration) {
        return _cachedSubscription;
      }
    }

    return await refreshSubscriptionStatus();
  }

  /// Save a new subscription
  Future<void> saveSubscription(Subscription subscription) async {
    try {
      final subscriptionJson = jsonEncode(subscription.toJson());
      await saveSubscriptionFfi(subscriptionJson);

      // Update cache
      if (subscription.userId == _userId) {
        _cachedSubscription = subscription;
        _lastSubscriptionCheck = DateTime.now();
      }
    } catch (e) {
      print('Error saving subscription: $e');
      rethrow;
    }
  }

  /// Start a premium subscription
  Future<void> startPremiumSubscription({
    required SubscriptionTier tier,
    required Platform platform,
    required String transactionId,
    Duration? duration,
  }) async {
    if (_userId == null) {
      throw StateError('MonetizationService not initialized');
    }

    final now = DateTime.now();
    final subscription = Subscription(
      id: _generateId(),
      userId: _userId!,
      tier: tier,
      status: SubscriptionStatus.active,
      startedAt: now,
      expiresAt: duration != null ? now.add(duration) : null,
      cancelledAt: null,
      platform: platform,
      transactionId: transactionId,
    );

    await saveSubscription(subscription);
  }

  /// Cancel subscription (marks as cancelled but keeps access until expiry)
  Future<void> cancelSubscription() async {
    final subscription = await getUserSubscription(forceRefresh: true);
    if (subscription == null) {
      throw StateError('No active subscription to cancel');
    }

    final updated = subscription.copyWith(
      status: SubscriptionStatus.cancelled,
      cancelledAt: DateTime.now(),
    );

    await saveSubscription(updated);
  }

  // ==========================================================================
  // Purchases
  // ==========================================================================

  /// Record a purchase
  Future<void> recordPurchase(Purchase purchase) async {
    try {
      final purchaseJson = jsonEncode(purchase.toJson());
      await savePurchaseFfi(purchaseJson);
    } catch (e) {
      print('Error recording purchase: $e');
      rethrow;
    }
  }

  /// Verify a purchase (mark as verified)
  Future<void> verifyPurchase(String purchaseId) async {
    try {
      await verifyPurchaseFfi(purchaseId);
    } catch (e) {
      print('Error verifying purchase: $e');
      rethrow;
    }
  }

  // ==========================================================================
  // Content Access
  // ==========================================================================

  /// Check if user can access content
  Future<Entitlement> checkContentAccess({
    required ContentAccess contentAccess,
    ContentType? contentType,
    String? contentId,
  }) async {
    if (_userId == null) {
      throw StateError('MonetizationService not initialized');
    }

    try {
      final contentAccessJson = jsonEncode(contentAccess.toJson());
      final contentTypeJson = contentType != null ? jsonEncode(contentType.toJson()) : null;

      final entitlementJson = await checkContentAccessFfi(
        _userId!,
        contentAccessJson,
        contentTypeJson,
        contentId,
      );

      final data = jsonDecode(entitlementJson);
      return Entitlement.fromJson(data);
    } catch (e) {
      print('Error checking content access: $e');
      rethrow;
    }
  }

  /// Unlock content for user
  Future<void> unlockContent({
    required ContentType contentType,
    required String contentId,
    String? purchaseId,
  }) async {
    if (_userId == null) {
      throw StateError('MonetizationService not initialized');
    }

    final unlock = ContentUnlock(
      id: _generateId(),
      userId: _userId!,
      contentType: contentType,
      contentId: contentId,
      unlockedAt: DateTime.now(),
      purchaseId: purchaseId,
    );

    try {
      final unlockJson = jsonEncode(unlock.toJson());
      await unlockContentFfi(unlockJson);
    } catch (e) {
      print('Error unlocking content: $e');
      rethrow;
    }
  }

  /// Get all unlocked content for user
  Future<List<ContentUnlock>> getUserUnlocks() async {
    if (_userId == null) {
      throw StateError('MonetizationService not initialized');
    }

    try {
      final unlocksJson = await getUserUnlocksFfi(_userId!);
      final List<dynamic> data = jsonDecode(unlocksJson);
      return data.map((json) => ContentUnlock.fromJson(json)).toList();
    } catch (e) {
      print('Error getting user unlocks: $e');
      return [];
    }
  }

  // ==========================================================================
  // Helpers
  // ==========================================================================

  String _generateId() {
    return DateTime.now().millisecondsSinceEpoch.toString();
  }
}

// ============================================================================
// Data Models
// ============================================================================

enum SubscriptionTier {
  free,
  premiumMonthly,
  premiumYearly,
  lifetime;

  String toJson() => name;
  static SubscriptionTier fromJson(String json) {
    return SubscriptionTier.values.firstWhere((e) => e.name == json);
  }

  bool get isPremium => this != free;
}

enum SubscriptionStatus {
  active,
  expired,
  cancelled,
  gracePeriod,
  trial;

  String toJson() => name;
  static SubscriptionStatus fromJson(String json) {
    return SubscriptionStatus.values.firstWhere((e) => e.name == json);
  }
}

enum Platform {
  appStore,
  playStore,
  samCart,
  direct;

  String toJson() => name;
  static Platform fromJson(String json) {
    return Platform.values.firstWhere((e) => e.name == json);
  }
}

class Subscription {
  final String id;
  final String userId;
  final SubscriptionTier tier;
  final SubscriptionStatus status;
  final DateTime startedAt;
  final DateTime? expiresAt;
  final DateTime? cancelledAt;
  final Platform platform;
  final String? transactionId;

  Subscription({
    required this.id,
    required this.userId,
    required this.tier,
    required this.status,
    required this.startedAt,
    this.expiresAt,
    this.cancelledAt,
    required this.platform,
    this.transactionId,
  });

  bool get isActive {
    if (status != SubscriptionStatus.active &&
        status != SubscriptionStatus.trial &&
        status != SubscriptionStatus.cancelled) {
      return false;
    }
    if (expiresAt != null && DateTime.now().isAfter(expiresAt!)) {
      return false;
    }
    return true;
  }

  bool get hasPremiumAccess => isActive && tier.isPremium;

  factory Subscription.fromJson(Map<String, dynamic> json) {
    return Subscription(
      id: json['id'],
      userId: json['user_id'],
      tier: SubscriptionTier.fromJson(json['tier']),
      status: SubscriptionStatus.fromJson(json['status']),
      startedAt: DateTime.parse(json['started_at']),
      expiresAt: json['expires_at'] != null ? DateTime.parse(json['expires_at']) : null,
      cancelledAt: json['cancelled_at'] != null ? DateTime.parse(json['cancelled_at']) : null,
      platform: Platform.fromJson(json['platform']),
      transactionId: json['transaction_id'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'user_id': userId,
      'tier': tier.toJson(),
      'status': status.toJson(),
      'started_at': startedAt.toIso8601String(),
      'expires_at': expiresAt?.toIso8601String(),
      'cancelled_at': cancelledAt?.toIso8601String(),
      'platform': platform.toJson(),
      'transaction_id': transactionId,
    };
  }

  Subscription copyWith({
    String? id,
    String? userId,
    SubscriptionTier? tier,
    SubscriptionStatus? status,
    DateTime? startedAt,
    DateTime? expiresAt,
    DateTime? cancelledAt,
    Platform? platform,
    String? transactionId,
  }) {
    return Subscription(
      id: id ?? this.id,
      userId: userId ?? this.userId,
      tier: tier ?? this.tier,
      status: status ?? this.status,
      startedAt: startedAt ?? this.startedAt,
      expiresAt: expiresAt ?? this.expiresAt,
      cancelledAt: cancelledAt ?? this.cancelledAt,
      platform: platform ?? this.platform,
      transactionId: transactionId ?? this.transactionId,
    );
  }
}

class Purchase {
  final String id;
  final String userId;
  final String productId;
  final Platform platform;
  final String transactionId;
  final DateTime purchasedAt;
  final int priceCents;
  final String currency;
  final bool verified;
  final Map<String, dynamic> metadata;

  Purchase({
    required this.id,
    required this.userId,
    required this.productId,
    required this.platform,
    required this.transactionId,
    required this.purchasedAt,
    required this.priceCents,
    required this.currency,
    this.verified = false,
    this.metadata = const {},
  });

  factory Purchase.fromJson(Map<String, dynamic> json) {
    return Purchase(
      id: json['id'],
      userId: json['user_id'],
      productId: json['product_id'],
      platform: Platform.fromJson(json['platform']),
      transactionId: json['transaction_id'],
      purchasedAt: DateTime.parse(json['purchased_at']),
      priceCents: json['price_cents'],
      currency: json['currency'],
      verified: json['verified'] ?? false,
      metadata: Map<String, dynamic>.from(json['metadata'] ?? {}),
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'user_id': userId,
      'product_id': productId,
      'platform': platform.toJson(),
      'transaction_id': transactionId,
      'purchased_at': purchasedAt.toIso8601String(),
      'price_cents': priceCents,
      'currency': currency,
      'verified': verified,
      'metadata': metadata,
    };
  }
}

enum ContentAccess {
  free,
  premium,
  unlockable;

  String toJson() => name;
  static ContentAccess fromJson(String json) {
    return ContentAccess.values.firstWhere((e) => e.name == json);
  }
}

enum ContentType {
  role,
  scenario,
  voicePack;

  String toJson() => name;
  static ContentType fromJson(String json) {
    return ContentType.values.firstWhere((e) => e.name == json);
  }
}

class ContentUnlock {
  final String id;
  final String userId;
  final ContentType contentType;
  final String contentId;
  final DateTime unlockedAt;
  final String? purchaseId;

  ContentUnlock({
    required this.id,
    required this.userId,
    required this.contentType,
    required this.contentId,
    required this.unlockedAt,
    this.purchaseId,
  });

  factory ContentUnlock.fromJson(Map<String, dynamic> json) {
    return ContentUnlock(
      id: json['id'],
      userId: json['user_id'],
      contentType: ContentType.fromJson(json['content_type']),
      contentId: json['content_id'],
      unlockedAt: DateTime.parse(json['unlocked_at']),
      purchaseId: json['purchase_id'],
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'id': id,
      'user_id': userId,
      'content_type': contentType.toJson(),
      'content_id': contentId,
      'unlocked_at': unlockedAt.toIso8601String(),
      'purchase_id': purchaseId,
    };
  }
}

enum EntitlementReason {
  freeContent,
  premiumSubscription,
  individualUnlock,
  trialAccess,
  requiresPremium,
  requiresUnlock,
  subscriptionExpired;

  String toJson() => name;
  static EntitlementReason fromJson(String json) {
    return EntitlementReason.values.firstWhere((e) => e.name == json);
  }
}

class Entitlement {
  final bool hasAccess;
  final EntitlementReason reason;
  final Subscription? subscription;

  Entitlement({
    required this.hasAccess,
    required this.reason,
    this.subscription,
  });

  factory Entitlement.fromJson(Map<String, dynamic> json) {
    return Entitlement(
      hasAccess: json['has_access'],
      reason: EntitlementReason.fromJson(json['reason']),
      subscription: json['subscription'] != null
          ? Subscription.fromJson(json['subscription'])
          : null,
    );
  }

  Map<String, dynamic> toJson() {
    return {
      'has_access': hasAccess,
      'reason': reason.toJson(),
      'subscription': subscription?.toJson(),
    };
  }
}

// ============================================================================
// FFI Stubs (replace with actual flutter_rust_bridge generated code)
// ============================================================================

Future<String?> getUserSubscriptionFfi(String userId) async {
  // TODO: Replace with actual FFI call
  // return await api.getUserSubscription(userId: userId);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<void> saveSubscriptionFfi(String subscriptionJson) async {
  // TODO: Replace with actual FFI call
  // await api.saveSubscription(subscriptionJson: subscriptionJson);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<void> savePurchaseFfi(String purchaseJson) async {
  // TODO: Replace with actual FFI call
  // await api.savePurchase(purchaseJson: purchaseJson);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<void> verifyPurchaseFfi(String purchaseId) async {
  // TODO: Replace with actual FFI call
  // await api.verifyPurchase(purchaseId: purchaseId);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<String> checkContentAccessFfi(
  String userId,
  String contentAccessJson,
  String? contentTypeJson,
  String? contentId,
) async {
  // TODO: Replace with actual FFI call
  // return await api.checkContentAccess(
  //   userId: userId,
  //   contentAccessJson: contentAccessJson,
  //   contentTypeJson: contentTypeJson,
  //   contentId: contentId,
  // );
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<void> unlockContentFfi(String unlockJson) async {
  // TODO: Replace with actual FFI call
  // await api.unlockContent(unlockJson: unlockJson);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}

Future<String> getUserUnlocksFfi(String userId) async {
  // TODO: Replace with actual FFI call
  // return await api.getUserUnlocks(userId: userId);
  throw UnimplementedError('Replace with flutter_rust_bridge generated code');
}
