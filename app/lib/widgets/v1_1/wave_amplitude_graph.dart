import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Wave Amplitude Graph
///
/// Visualizes the JoyWave metric: spontaneous returns + positive emotion tags.
/// Displays engagement over time as an animated wave chart.
///
/// Usage:
/// ```dart
/// WaveAmplitudeGraph(
///   dataPoints: [0.3, 0.5, 0.7, 0.9, 0.6],
///   maxValue: 1.0,
/// )
/// ```
class WaveAmplitudeGraph extends StatefulWidget {
  final List<double> dataPoints;
  final double maxValue;
  final double width;
  final double height;
  final Color waveColor;

  const WaveAmplitudeGraph({
    super.key,
    required this.dataPoints,
    this.maxValue = 1.0,
    this.width = 300,
    this.height = 150,
    this.waveColor = const Color(0xFF7ED321),
  });

  @override
  State<WaveAmplitudeGraph> createState() => _WaveAmplitudeGraphState();
}

class _WaveAmplitudeGraphState extends State<WaveAmplitudeGraph>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _animation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 1500),
    );

    _animation = CurvedAnimation(
      parent: _controller,
      curve: Curves.easeInOut,
    );

    _controller.forward();
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AnimatedBuilder(
      animation: _animation,
      builder: (context, child) {
        return CustomPaint(
          painter: WaveAmplitudePainter(
            dataPoints: widget.dataPoints,
            maxValue: widget.maxValue,
            animationValue: _animation.value,
            waveColor: widget.waveColor,
          ),
          size: Size(widget.width, widget.height),
        );
      },
    );
  }
}

/// Custom painter for wave amplitude visualization
class WaveAmplitudePainter extends CustomPainter {
  final List<double> dataPoints;
  final double maxValue;
  final double animationValue;
  final Color waveColor;

  WaveAmplitudePainter({
    required this.dataPoints,
    required this.maxValue,
    required this.animationValue,
    required this.waveColor,
  });

  @override
  void paint(Canvas canvas, Size size) {
    if (dataPoints.isEmpty) return;

    final paint = Paint()
      ..color = waveColor
      ..style = PaintingStyle.stroke
      ..strokeWidth = 3.0
      ..strokeCap = StrokeCap.round;

    final fillPaint = Paint()
      ..shader = LinearGradient(
        begin: Alignment.topCenter,
        end: Alignment.bottomCenter,
        colors: [
          waveColor.withOpacity(0.3),
          waveColor.withOpacity(0.0),
        ],
      ).createShader(Rect.fromLTWH(0, 0, size.width, size.height));

    final path = Path();
    final fillPath = Path();

    final segmentWidth = size.width / (dataPoints.length - 1);

    // Start paths
    final firstX = 0.0;
    final firstY = size.height - (dataPoints[0] / maxValue) * size.height * animationValue;
    path.moveTo(firstX, firstY);
    fillPath.moveTo(firstX, size.height);
    fillPath.lineTo(firstX, firstY);

    // Draw smooth curve through points
    for (int i = 0; i < dataPoints.length - 1; i++) {
      final x1 = i * segmentWidth;
      final y1 = size.height - (dataPoints[i] / maxValue) * size.height * animationValue;
      final x2 = (i + 1) * segmentWidth;
      final y2 = size.height - (dataPoints[i + 1] / maxValue) * size.height * animationValue;

      final controlX = (x1 + x2) / 2;
      final controlY1 = y1;
      final controlY2 = y2;

      path.quadraticBezierTo(controlX, controlY1, x2, y2);
      fillPath.quadraticBezierTo(controlX, controlY1, x2, y2);
    }

    // Close fill path
    fillPath.lineTo(size.width, size.height);
    fillPath.close();

    // Draw fill and stroke
    canvas.drawPath(fillPath, fillPaint);
    canvas.drawPath(path, paint);

    // Draw data points
    for (int i = 0; i < dataPoints.length; i++) {
      final x = i * segmentWidth;
      final y = size.height - (dataPoints[i] / maxValue) * size.height * animationValue;

      canvas.drawCircle(
        Offset(x, y),
        5.0,
        Paint()..color = waveColor,
      );

      canvas.drawCircle(
        Offset(x, y),
        3.0,
        Paint()..color = Colors.white,
      );
    }
  }

  @override
  bool shouldRepaint(WaveAmplitudePainter oldDelegate) => true;
}

/// Joy Wave Card
///
/// Displays JoyWave metric with trend indicator and sparkline.
class JoyWaveCard extends StatelessWidget {
  final int joyWaveScore;
  final int spontaneousReturns;
  final int positiveEmotions;
  final List<double> trendData;
  final String trendDirection; // 'up', 'down', 'stable'

  const JoyWaveCard({
    super.key,
    required this.joyWaveScore,
    required this.spontaneousReturns,
    required this.positiveEmotions,
    required this.trendData,
    this.trendDirection = 'stable',
  });

  @override
  Widget build(BuildContext context) {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Container(
        padding: const EdgeInsets.all(20),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(16),
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              const Color(0xFF7ED321).withOpacity(0.1),
              Colors.white,
            ],
          ),
        ),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                Column(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Row(
                      children: [
                        const Icon(
                          Icons.waves,
                          color: Color(0xFF7ED321),
                          size: 24,
                        ),
                        const SizedBox(width: 8),
                        const Text(
                          'JoyWave',
                          style: TextStyle(
                            fontSize: 18,
                            fontWeight: FontWeight.w600,
                            color: Colors.black87,
                          ),
                        ),
                      ],
                    ),
                    const SizedBox(height: 4),
                    Text(
                      'Your engagement pulse',
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey[600],
                      ),
                    ),
                  ],
                ),
                _buildTrendIndicator(),
              ],
            ),
            const SizedBox(height: 20),
            Row(
              children: [
                Text(
                  joyWaveScore.toString(),
                  style: const TextStyle(
                    fontSize: 48,
                    fontWeight: FontWeight.bold,
                    color: Color(0xFF7ED321),
                  ),
                ),
                const SizedBox(width: 12),
                Expanded(
                  child: Column(
                    crossAxisAlignment: CrossAxisAlignment.start,
                    children: [
                      _buildMetricRow(
                        Icons.refresh,
                        'Spontaneous returns',
                        spontaneousReturns,
                      ),
                      const SizedBox(height: 6),
                      _buildMetricRow(
                        Icons.sentiment_satisfied_alt,
                        'Positive emotions',
                        positiveEmotions,
                      ),
                    ],
                  ),
                ),
              ],
            ),
            const SizedBox(height: 20),
            WaveAmplitudeGraph(
              dataPoints: trendData,
              width: double.infinity,
              height: 80,
            ),
            const SizedBox(height: 8),
            Text(
              'Last 7 days',
              textAlign: TextAlign.center,
              style: TextStyle(
                fontSize: 11,
                color: Colors.grey[500],
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildTrendIndicator() {
    IconData icon;
    Color color;

    switch (trendDirection) {
      case 'up':
        icon = Icons.trending_up;
        color = const Color(0xFF7ED321);
        break;
      case 'down':
        icon = Icons.trending_down;
        color = const Color(0xFFF5A623);
        break;
      default:
        icon = Icons.trending_flat;
        color = const Color(0xFF4A90E2);
    }

    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        children: [
          Icon(icon, size: 16, color: color),
          const SizedBox(width: 4),
          Text(
            trendDirection.toUpperCase(),
            style: TextStyle(
              fontSize: 11,
              fontWeight: FontWeight.bold,
              color: color,
              letterSpacing: 0.5,
            ),
          ),
        ],
      ),
    );
  }

  Widget _buildMetricRow(IconData icon, String label, int value) {
    return Row(
      children: [
        Icon(icon, size: 14, color: Colors.grey[600]),
        const SizedBox(width: 6),
        Text(
          '$value ',
          style: const TextStyle(
            fontSize: 13,
            fontWeight: FontWeight.bold,
            color: Colors.black87,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontSize: 12,
            color: Colors.grey[600],
          ),
        ),
      ],
    );
  }
}

/// Mini sparkline for compact trend visualization
class Sparkline extends StatelessWidget {
  final List<double> data;
  final double width;
  final double height;
  final Color color;

  const Sparkline({
    super.key,
    required this.data,
    this.width = 60,
    this.height = 20,
    this.color = const Color(0xFF7ED321),
  });

  @override
  Widget build(BuildContext context) {
    return CustomPaint(
      painter: SparklinePainter(data: data, color: color),
      size: Size(width, height),
    );
  }
}

class SparklinePainter extends CustomPainter {
  final List<double> data;
  final Color color;

  SparklinePainter({required this.data, required this.color});

  @override
  void paint(Canvas canvas, Size size) {
    if (data.isEmpty) return;

    final paint = Paint()
      ..color = color
      ..style = PaintingStyle.stroke
      ..strokeWidth = 2.0;

    final maxValue = data.reduce(math.max);
    final path = Path();
    final segmentWidth = size.width / (data.length - 1);

    path.moveTo(0, size.height - (data[0] / maxValue) * size.height);

    for (int i = 1; i < data.length; i++) {
      final x = i * segmentWidth;
      final y = size.height - (data[i] / maxValue) * size.height;
      path.lineTo(x, y);
    }

    canvas.drawPath(path, paint);
  }

  @override
  bool shouldRepaint(SparklinePainter oldDelegate) => true;
}
