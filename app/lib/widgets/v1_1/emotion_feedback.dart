import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Emotion Color Feedback Widget
///
/// Real-time visual feedback during voice recording that changes color
/// based on detected emotional tone. Pulsates with varying amplitude.
///
/// Colors:
/// - Green (#7ED321): Calm, Confident, Clear
/// - Amber (#F5A623): Nervous, Uncertain, Rushed
/// - Blue (#4A90E2): Neutral / default
///
/// Usage:
/// ```dart
/// EmotionFeedback(
///   tone: 'Confident',
///   confidence: 0.85,
///   isActive: true,
/// )
/// ```
class EmotionFeedback extends StatefulWidget {
  final String tone;
  final double confidence;
  final bool isActive;

  const EmotionFeedback({
    super.key,
    required this.tone,
    required this.confidence,
    this.isActive = false,
  });

  @override
  State<EmotionFeedback> createState() => _EmotionFeedbackState();
}

class _EmotionFeedbackState extends State<EmotionFeedback>
    with TickerProviderStateMixin {
  late AnimationController _pulseController;
  late Animation<double> _pulseAnimation;

  @override
  void initState() {
    super.initState();

    _pulseController = AnimationController(
      vsync: this,
      duration: Duration(milliseconds: _getPulseDuration(widget.tone)),
    )..repeat(reverse: true);

    _pulseAnimation = Tween<double>(
      begin: 1.0,
      end: 1.0 + (_getWaveAmplitude(widget.tone) / 100.0),
    ).animate(
      CurvedAnimation(parent: _pulseController, curve: Curves.easeInOut),
    );
  }

  @override
  void didUpdateWidget(EmotionFeedback oldWidget) {
    super.didUpdateWidget(oldWidget);

    if (oldWidget.tone != widget.tone || oldWidget.isActive != widget.isActive) {
      _pulseController.duration = Duration(
        milliseconds: _getPulseDuration(widget.tone),
      );
    }
  }

  @override
  void dispose() {
    _pulseController.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    if (!widget.isActive) {
      return _buildInactiveState();
    }

    return AnimatedBuilder(
      animation: _pulseAnimation,
      builder: (context, child) {
        return Transform.scale(
          scale: _pulseAnimation.value,
          child: Container(
            width: 200,
            height: 200,
            decoration: BoxDecoration(
              shape: BoxShape.circle,
              gradient: RadialGradient(
                colors: [
                  _getEmotionColor(widget.tone).withOpacity(0.8),
                  _getEmotionColor(widget.tone).withOpacity(0.3),
                  _getEmotionColor(widget.tone).withOpacity(0.0),
                ],
                stops: const [0.0, 0.6, 1.0],
              ),
            ),
            child: Center(
              child: Column(
                mainAxisAlignment: MainAxisAlignment.center,
                children: [
                  Icon(
                    _getEmotionIcon(widget.tone),
                    size: 48,
                    color: Colors.white,
                  ),
                  const SizedBox(height: 12),
                  Text(
                    widget.tone,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 18,
                      fontWeight: FontWeight.w600,
                      letterSpacing: 0.5,
                    ),
                  ),
                  const SizedBox(height: 4),
                  Text(
                    '${(widget.confidence * 100).toInt()}% confident',
                    style: TextStyle(
                      color: Colors.white.withOpacity(0.8),
                      fontSize: 12,
                    ),
                  ),
                ],
              ),
            ),
          ),
        );
      },
    );
  }

  Widget _buildInactiveState() {
    return Container(
      width: 200,
      height: 200,
      decoration: BoxDecoration(
        shape: BoxShape.circle,
        color: Colors.grey.withOpacity(0.2),
        border: Border.all(
          color: Colors.grey.withOpacity(0.4),
          width: 2,
        ),
      ),
      child: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: [
            Icon(
              Icons.mic,
              size: 48,
              color: Colors.grey[600],
            ),
            const SizedBox(height: 12),
            Text(
              'Start speaking',
              style: TextStyle(
                color: Colors.grey[600],
                fontSize: 16,
              ),
            ),
          ],
        ),
      ),
    );
  }

  Color _getEmotionColor(String tone) {
    switch (tone.toLowerCase()) {
      case 'calm':
      case 'confident':
      case 'clear':
        return const Color(0xFF7ED321); // Green
      case 'nervous':
      case 'uncertain':
      case 'rushed':
        return const Color(0xFFF5A623); // Amber
      case 'excited':
      case 'energetic':
        return const Color(0xFFE91E63); // Pink
      default:
        return const Color(0xFF4A90E2); // Blue (neutral)
    }
  }

  IconData _getEmotionIcon(String tone) {
    switch (tone.toLowerCase()) {
      case 'calm':
        return Icons.spa;
      case 'confident':
        return Icons.star;
      case 'clear':
        return Icons.check_circle;
      case 'nervous':
        return Icons.warning_amber;
      case 'uncertain':
        return Icons.help_outline;
      case 'rushed':
        return Icons.fast_forward;
      case 'excited':
        return Icons.bolt;
      case 'energetic':
        return Icons.flash_on;
      default:
        return Icons.circle;
    }
  }

  int _getWaveAmplitude(String tone) {
    // Based on EmotionTag::wave_amplitude() in Rust
    switch (tone.toLowerCase()) {
      case 'calm':
        return 3;
      case 'confident':
      case 'clear':
        return 5;
      case 'excited':
      case 'energetic':
        return 8;
      default:
        return 4;
    }
  }

  int _getPulseDuration(String tone) {
    final amplitude = _getWaveAmplitude(tone);
    // Higher amplitude = faster pulse
    return math.max(400, 1200 - amplitude * 100);
  }
}

/// Emotion History Bar
///
/// Shows a timeline of recent emotion tags as colored dots.
/// Useful for visualizing emotion patterns during a scene.
class EmotionHistoryBar extends StatelessWidget {
  final List<EmotionTagModel> emotions;
  final double maxWidth;

  const EmotionHistoryBar({
    super.key,
    required this.emotions,
    this.maxWidth = 300,
  });

  @override
  Widget build(BuildContext context) {
    if (emotions.isEmpty) {
      return const SizedBox.shrink();
    }

    return Container(
      width: maxWidth,
      height: 40,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      decoration: BoxDecoration(
        color: Colors.black.withOpacity(0.2),
        borderRadius: BorderRadius.circular(20),
      ),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.spaceEvenly,
        children: emotions.map((emotion) {
          return Tooltip(
            message: '${emotion.tone} (${(emotion.confidence * 100).toInt()}%)',
            child: Container(
              width: 12,
              height: 12,
              decoration: BoxDecoration(
                color: _getColorForTone(emotion.tone),
                shape: BoxShape.circle,
                border: Border.all(
                  color: Colors.white.withOpacity(0.5),
                  width: 1,
                ),
              ),
            ),
          );
        }).toList(),
      ),
    );
  }

  Color _getColorForTone(String tone) {
    switch (tone.toLowerCase()) {
      case 'calm':
      case 'confident':
      case 'clear':
        return const Color(0xFF7ED321);
      case 'nervous':
      case 'uncertain':
      case 'rushed':
        return const Color(0xFFF5A623);
      default:
        return const Color(0xFF4A90E2);
    }
  }
}

/// Emotion Balance Indicator
///
/// Shows the ratio of confident vs nervous emotions (EmotionBalance metric).
/// Green bar = confident, Amber bar = nervous.
class EmotionBalanceIndicator extends StatelessWidget {
  final double emotionBalance; // 0.0 to 1.0
  final int confidentCount;
  final int nervousCount;

  const EmotionBalanceIndicator({
    super.key,
    required this.emotionBalance,
    required this.confidentCount,
    required this.nervousCount,
  });

  @override
  Widget build(BuildContext context) {
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            const Text(
              'Emotion Balance',
              style: TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.w600,
                color: Colors.black87,
              ),
            ),
            Text(
              '${(emotionBalance * 100).toInt()}%',
              style: const TextStyle(
                fontSize: 14,
                fontWeight: FontWeight.bold,
                color: Color(0xFF7ED321),
              ),
            ),
          ],
        ),
        const SizedBox(height: 8),
        ClipRRect(
          borderRadius: BorderRadius.circular(8),
          child: LinearProgressIndicator(
            value: emotionBalance,
            minHeight: 12,
            backgroundColor: const Color(0xFFF5A623).withOpacity(0.3),
            valueColor: const AlwaysStoppedAnimation<Color>(Color(0xFF7ED321)),
          ),
        ),
        const SizedBox(height: 6),
        Row(
          mainAxisAlignment: MainAxisAlignment.spaceBetween,
          children: [
            _buildLegend(
              color: const Color(0xFF7ED321),
              label: 'Confident',
              count: confidentCount,
            ),
            _buildLegend(
              color: const Color(0xFFF5A623),
              label: 'Nervous',
              count: nervousCount,
            ),
          ],
        ),
      ],
    );
  }

  Widget _buildLegend({
    required Color color,
    required String label,
    required int count,
  }) {
    return Row(
      children: [
        Container(
          width: 10,
          height: 10,
          decoration: BoxDecoration(
            color: color,
            shape: BoxShape.circle,
          ),
        ),
        const SizedBox(width: 6),
        Text(
          '$label: $count',
          style: const TextStyle(
            fontSize: 12,
            color: Colors.black54,
          ),
        ),
      ],
    );
  }
}

/// Model for EmotionTag data
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

  // TODO: Add fromJson for FFI integration
  factory EmotionTagModel.fromJson(Map<String, dynamic> json) {
    return EmotionTagModel(
      sceneId: json['scene_id'],
      tone: json['tone'],
      confidence: json['confidence'],
      timestamp: DateTime.parse(json['timestamp']),
    );
  }
}
