import 'dart:convert';

/// V1.1 API Service
///
/// Wrapper for all v1.1 FFI API calls. Replace mock implementations
/// with actual FFI bridge calls when flutter_rust_bridge is generated.
///
/// To generate FFI bindings:
/// ```bash
/// cd core
/// flutter_rust_bridge_codegen generate
/// ```
///
/// Then import: import '../bridge/bridge.generated.dart' as native;
class V11ApiService {
  // TODO: Initialize FFI bridge
  // final native.NativeApi _api = native.NativeApi();

  // ====================================================================
  // Role Progress API
  // ====================================================================

  /// Start tracking progress for a role
  Future<RoleProgressModel> startRoleProgress({
    required String roleId,
    required int totalScenes,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.startRoleProgress(
    //   roleId: roleId,
    //   totalScenes: totalScenes,
    // );
    // return RoleProgressModel.fromJson(jsonDecode(json));

    // Mock data
    return RoleProgressModel(
      roleId: roleId,
      currentSceneIndex: 0,
      totalScenes: totalScenes,
      coherence: 0.0,
      emotionTags: [],
      lastTransition: null,
      consecutiveDays: 0,
      createdAt: DateTime.now(),
      updatedAt: DateTime.now(),
    );
  }

  /// Complete a scene with emotion feedback
  Future<RoleProgressModel> completeSceneWithEmotion({
    required String roleId,
    required String sceneId,
    required String tone,
    required double confidence,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.completeSceneWithEmotion(
    //   roleId: roleId,
    //   sceneId: sceneId,
    //   tone: tone,
    //   confidence: confidence,
    // );
    // return RoleProgressModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Scene completion');
  }

  /// Get current role progress
  Future<RoleProgressModel> getRoleProgress(String roleId) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.getRoleProgressJson(roleId: roleId);
    // return RoleProgressModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Get role progress');
  }

  /// Get liminal transition data
  Future<LiminalTransitionModel> getLiminalTransition(String roleId) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.getLiminalTransitionJson(roleId: roleId);
    // return LiminalTransitionModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Get liminal transition');
  }

  /// Update consecutive days streak
  Future<RoleProgressModel> updateConsecutiveDays({
    required String roleId,
    required int days,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.updateConsecutiveDays(roleId: roleId, days: days);
    // return RoleProgressModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Update consecutive days');
  }

  // ====================================================================
  // Social Resonance API
  // ====================================================================

  /// Create a resonance trace (anonymous share)
  Future<ResonanceTraceModel> createResonanceTrace({
    required String traceId,
    required String roleId,
    required String sceneId,
    required String message,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.createResonanceTrace(
    //   traceId: traceId,
    //   roleId: roleId,
    //   sceneId: sceneId,
    //   message: message,
    // );
    // return ResonanceTraceModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Create resonance trace');
  }

  /// Add a reflection to a trace
  Future<ResonanceTraceModel> addReflectionToTrace({
    required String traceId,
    required String message,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.addReflectionToTrace(
    //   traceId: traceId,
    //   message: message,
    // );
    // return ResonanceTraceModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Add reflection');
  }

  /// Get recent resonance traces
  Future<List<ResonanceTraceModel>> getRecentTraces({
    String? roleId,
    int limit = 20,
  }) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.getRecentTracesJson(
    //   roleId: roleId,
    //   limit: limit,
    // );
    // final List<dynamic> data = jsonDecode(json);
    // return data.map((e) => ResonanceTraceModel.fromJson(e)).toList();

    throw UnimplementedError('Mock: Get recent traces');
  }

  /// Get a specific trace by ID
  Future<ResonanceTraceModel> getTrace(String traceId) async {
    // TODO: Replace with actual FFI call
    // final json = await _api.getTraceJson(traceId: traceId);
    // return ResonanceTraceModel.fromJson(jsonDecode(json));

    throw UnimplementedError('Mock: Get trace');
  }
}

// ======================================================================
// Data Models
// ======================================================================

class RoleProgressModel {
  final String roleId;
  final int currentSceneIndex;
  final int totalScenes;
  final double coherence;
  final List<EmotionTagModel> emotionTags;
  final DateTime? lastTransition;
  final int consecutiveDays;
  final DateTime createdAt;
  final DateTime updatedAt;

  RoleProgressModel({
    required this.roleId,
    required this.currentSceneIndex,
    required this.totalScenes,
    required this.coherence,
    required this.emotionTags,
    required this.lastTransition,
    required this.consecutiveDays,
    required this.createdAt,
    required this.updatedAt,
  });

  factory RoleProgressModel.fromJson(Map<String, dynamic> json) {
    return RoleProgressModel(
      roleId: json['role_id'],
      currentSceneIndex: json['current_scene_index'],
      totalScenes: json['total_scenes'],
      coherence: json['coherence'],
      emotionTags: (json['emotion_tags'] as List)
          .map((e) => EmotionTagModel.fromJson(e))
          .toList(),
      lastTransition: json['last_transition'] != null
          ? DateTime.parse(json['last_transition'])
          : null,
      consecutiveDays: json['consecutive_days'],
      createdAt: DateTime.parse(json['created_at']),
      updatedAt: DateTime.parse(json['updated_at']),
    );
  }

  double get roleFlow {
    final completion = currentSceneIndex / totalScenes;
    final multiplier = _consistencyMultiplier();
    return completion * multiplier;
  }

  double _consistencyMultiplier() {
    if (consecutiveDays >= 7) return 1.5;
    if (consecutiveDays >= 3) return 1.2;
    if (consecutiveDays >= 1) return 1.0;
    return 0.8;
  }

  double get emotionBalance {
    if (emotionTags.isEmpty) return 0.5;
    final confidentCount = emotionTags.where((e) {
      final tone = e.tone.toLowerCase();
      return tone == 'calm' || tone == 'confident' || tone == 'clear';
    }).length;
    return confidentCount / emotionTags.length;
  }
}

class EmotionTagModel {
  final String sceneId;
  final String tone;
  final double confidence;
  final DateTime timestamp;

  EmotionTagModel({
    required this.sceneId,
    required this.tone,
    required this.confidence,
    required this.timestamp,
  });

  factory EmotionTagModel.fromJson(Map<String, dynamic> json) {
    return EmotionTagModel(
      sceneId: json['scene_id'],
      tone: json['tone'],
      confidence: json['confidence'],
      timestamp: DateTime.parse(json['timestamp']),
    );
  }
}

class LiminalTransitionModel {
  final String message;
  final double prevCoherence;
  final double currCoherence;
  final int animationDurationMs;
  final String colorFrom;
  final String colorTo;
  final String sound;

  LiminalTransitionModel({
    required this.message,
    required this.prevCoherence,
    required this.currCoherence,
    required this.animationDurationMs,
    required this.colorFrom,
    required this.colorTo,
    required this.sound,
  });

  factory LiminalTransitionModel.fromJson(Map<String, dynamic> json) {
    return LiminalTransitionModel(
      message: json['message'],
      prevCoherence: json['prev_coherence'],
      currCoherence: json['curr_coherence'],
      animationDurationMs: json['animation_duration_ms'],
      colorFrom: json['color_from'],
      colorTo: json['color_to'],
      sound: json['sound'],
    );
  }
}

class ResonanceTraceModel {
  final String id;
  final String roleId;
  final String sceneId;
  final String message;
  final List<ReflectionModel> reflections;
  final DateTime createdAt;

  ResonanceTraceModel({
    required this.id,
    required this.roleId,
    required this.sceneId,
    required this.message,
    required this.reflections,
    required this.createdAt,
  });

  factory ResonanceTraceModel.fromJson(Map<String, dynamic> json) {
    return ResonanceTraceModel(
      id: json['id'],
      roleId: json['role_id'],
      sceneId: json['scene_id'],
      message: json['message'],
      reflections: (json['reflections'] as List)
          .map((e) => ReflectionModel.fromJson(e))
          .toList(),
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}

class ReflectionModel {
  final String traceId;
  final String message;
  final DateTime createdAt;

  ReflectionModel({
    required this.traceId,
    required this.message,
    required this.createdAt,
  });

  factory ReflectionModel.fromJson(Map<String, dynamic> json) {
    return ReflectionModel(
      traceId: json['trace_id'],
      message: json['message'],
      createdAt: DateTime.parse(json['created_at']),
    );
  }
}
