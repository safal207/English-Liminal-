import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Liminal Transition Widget
///
/// A 2.5-second animated transition that appears when users complete a scene
/// or reach a role milestone. Features spiral → sphere morphing animation
/// with color gradient from blue (#4A90E2) to green (#7ED321).
///
/// Usage:
/// ```dart
/// LiminalTransition.show(
///   context: context,
///   message: "You've taken the first step...",
///   prevCoherence: 0.65,
///   currCoherence: 0.75,
/// );
/// ```
class LiminalTransition extends StatefulWidget {
  final String message;
  final double prevCoherence;
  final double currCoherence;
  final String colorFrom;
  final String colorTo;
  final VoidCallback? onComplete;

  const LiminalTransition({
    super.key,
    required this.message,
    required this.prevCoherence,
    required this.currCoherence,
    this.colorFrom = '#4A90E2',
    this.colorTo = '#7ED321',
    this.onComplete,
  });

  /// Show transition as a fullscreen overlay
  static void show({
    required BuildContext context,
    required String message,
    required double prevCoherence,
    required double currCoherence,
    String colorFrom = '#4A90E2',
    String colorTo = '#7ED321',
    VoidCallback? onComplete,
  }) {
    Navigator.of(context).push(
      PageRouteBuilder(
        opaque: false,
        barrierDismissible: false,
        pageBuilder: (context, animation, secondaryAnimation) {
          return LiminalTransition(
            message: message,
            prevCoherence: prevCoherence,
            currCoherence: currCoherence,
            colorFrom: colorFrom,
            colorTo: colorTo,
            onComplete: () {
              Navigator.of(context).pop();
              onComplete?.call();
            },
          );
        },
      ),
    );
  }

  @override
  State<LiminalTransition> createState() => _LiminalTransitionState();
}

class _LiminalTransitionState extends State<LiminalTransition>
    with TickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _spiralAnimation;
  late Animation<double> _sphereAnimation;
  late Animation<double> _fadeAnimation;
  late Animation<Color?> _colorAnimation;

  @override
  void initState() {
    super.initState();

    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 2500),
    );

    // Spiral phase: 0.0 → 0.6 (1.5s)
    _spiralAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.0, 0.6, curve: Curves.easeOut),
      ),
    );

    // Sphere phase: 0.6 → 1.0 (0.9s)
    _sphereAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.6, 1.0, curve: Curves.easeInOut),
      ),
    );

    // Fade in text: 0.7 → 1.0
    _fadeAnimation = Tween<double>(begin: 0.0, end: 1.0).animate(
      CurvedAnimation(
        parent: _controller,
        curve: const Interval(0.7, 1.0, curve: Curves.easeIn),
      ),
    );

    // Color gradient: blue → green
    _colorAnimation = ColorTween(
      begin: _hexToColor(widget.colorFrom),
      end: _hexToColor(widget.colorTo),
    ).animate(_controller);

    _controller.forward().then((_) {
      Future.delayed(const Duration(milliseconds: 500), () {
        widget.onComplete?.call();
      });
    });
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Material(
      color: Colors.black.withOpacity(0.9),
      child: AnimatedBuilder(
        animation: _controller,
        builder: (context, child) {
          return Stack(
            children: [
              // Animated spiral/sphere background
              Center(
                child: CustomPaint(
                  painter: SpiralSpherePainter(
                    spiralProgress: _spiralAnimation.value,
                    sphereProgress: _sphereAnimation.value,
                    color: _colorAnimation.value ?? Colors.blue,
                  ),
                  size: const Size(300, 300),
                ),
              ),

              // Coherence progress indicator
              Positioned(
                top: 100,
                left: 0,
                right: 0,
                child: Center(
                  child: CoherenceBar(
                    prevValue: widget.prevCoherence,
                    currValue: widget.currCoherence,
                    animationValue: _controller.value,
                  ),
                ),
              ),

              // Transition message
              Positioned(
                bottom: 120,
                left: 40,
                right: 40,
                child: Opacity(
                  opacity: _fadeAnimation.value,
                  child: Text(
                    widget.message,
                    textAlign: TextAlign.center,
                    style: const TextStyle(
                      color: Colors.white,
                      fontSize: 20,
                      fontWeight: FontWeight.w300,
                      letterSpacing: 0.8,
                      height: 1.4,
                    ),
                  ),
                ),
              ),
            ],
          );
        },
      ),
    );
  }

  Color _hexToColor(String hex) {
    hex = hex.replaceAll('#', '');
    return Color(int.parse('FF$hex', radix: 16));
  }
}

/// Custom painter for spiral → sphere morphing animation
class SpiralSpherePainter extends CustomPainter {
  final double spiralProgress;
  final double sphereProgress;
  final Color color;

  SpiralSpherePainter({
    required this.spiralProgress,
    required this.sphereProgress,
    required this.color,
  });

  @override
  void paint(Canvas canvas, Size size) {
    final center = Offset(size.width / 2, size.height / 2);
    final paint = Paint()
      ..color = color.withOpacity(0.6)
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.0;

    if (sphereProgress < 0.5) {
      // Draw spiral
      _drawSpiral(canvas, center, size, paint);
    } else {
      // Morph to sphere
      _drawSphere(canvas, center, size, paint);
    }
  }

  void _drawSpiral(Canvas canvas, Offset center, Size size, Paint paint) {
    final path = Path();
    final maxRadius = size.width / 2 * spiralProgress;
    final turns = 3.0;

    path.moveTo(center.dx, center.dy);

    for (double t = 0; t <= turns * 2 * math.pi; t += 0.1) {
      final r = maxRadius * (t / (turns * 2 * math.pi));
      final x = center.dx + r * math.cos(t);
      final y = center.dy + r * math.sin(t);
      path.lineTo(x, y);
    }

    canvas.drawPath(path, paint);

    // Draw particles along spiral
    for (int i = 0; i < 12; i++) {
      final t = (turns * 2 * math.pi) * (i / 12.0) * spiralProgress;
      final r = maxRadius * (t / (turns * 2 * math.pi));
      final x = center.dx + r * math.cos(t);
      final y = center.dy + r * math.sin(t);

      canvas.drawCircle(
        Offset(x, y),
        3.0,
        Paint()..color = color.withOpacity(0.8),
      );
    }
  }

  void _drawSphere(Canvas canvas, Offset center, Size size, Paint paint) {
    final radius = (size.width / 2) * sphereProgress;

    // Draw outer sphere
    canvas.drawCircle(center, radius, paint);

    // Draw inner glow circles
    for (int i = 1; i <= 3; i++) {
      final innerRadius = radius * (0.3 + i * 0.2);
      canvas.drawCircle(
        center,
        innerRadius,
        Paint()
          ..color = color.withOpacity(0.3 - i * 0.08)
          ..style = PaintingStyle.stroke
          ..strokeWidth = 1.5,
      );
    }

    // Draw pulsing particles around sphere
    for (int i = 0; i < 8; i++) {
      final angle = (i / 8.0) * 2 * math.pi;
      final x = center.dx + radius * math.cos(angle);
      final y = center.dy + radius * math.sin(angle);

      canvas.drawCircle(
        Offset(x, y),
        4.0,
        Paint()..color = color.withOpacity(0.9),
      );
    }
  }

  @override
  bool shouldRepaint(SpiralSpherePainter oldDelegate) => true;
}

/// Coherence progress bar showing prev → curr values
class CoherenceBar extends StatelessWidget {
  final double prevValue;
  final double currValue;
  final double animationValue;

  const CoherenceBar({
    super.key,
    required this.prevValue,
    required this.currValue,
    required this.animationValue,
  });

  @override
  Widget build(BuildContext context) {
    final displayValue = prevValue + (currValue - prevValue) * animationValue;

    return Column(
      children: [
        Text(
          'Role Coherence',
          style: TextStyle(
            color: Colors.white.withOpacity(0.7),
            fontSize: 14,
            letterSpacing: 1.2,
          ),
        ),
        const SizedBox(height: 12),
        SizedBox(
          width: 200,
          child: ClipRRect(
            borderRadius: BorderRadius.circular(10),
            child: LinearProgressIndicator(
              value: displayValue,
              minHeight: 8,
              backgroundColor: Colors.white.withOpacity(0.2),
              valueColor: AlwaysStoppedAnimation<Color>(
                Color.lerp(
                  const Color(0xFF4A90E2),
                  const Color(0xFF7ED321),
                  displayValue,
                )!,
              ),
            ),
          ),
        ),
        const SizedBox(height: 8),
        Text(
          '${(displayValue * 100).toInt()}%',
          style: const TextStyle(
            color: Colors.white,
            fontSize: 24,
            fontWeight: FontWeight.bold,
          ),
        ),
      ],
    );
  }
}
