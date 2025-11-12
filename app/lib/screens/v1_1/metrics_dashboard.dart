import 'package:flutter/material.dart';
import '../../widgets/v1_1/emotion_feedback.dart';
import '../../widgets/v1_1/wave_amplitude_graph.dart';

/// Metrics Dashboard Screen
///
/// Central hub displaying all v1.1 metrics:
/// - RoleFlow: completion × consistency multiplier
/// - EmotionBalance: confident vs nervous ratio
/// - JoyWave: spontaneous returns + positive emotions
/// - SocialEcho: traces sent + reflections received
///
/// This screen gives users insight into their liminal journey.
class MetricsDashboard extends StatefulWidget {
  const MetricsDashboard({super.key});

  @override
  State<MetricsDashboard> createState() => _MetricsDashboardState();
}

class _MetricsDashboardState extends State<MetricsDashboard> {
  // TODO: Load from FFI API
  final _metricsData = MetricsModel(
    roleFlow: 0.72,
    completedScenes: 9,
    totalScenes: 13,
    consecutiveDays: 5,
    consistencyMultiplier: 1.2,
    emotionBalance: 0.68,
    confidentCount: 17,
    nervousCount: 8,
    joyWave: 42,
    spontaneousReturns: 12,
    positiveEmotions: 30,
    socialEcho: 28,
    tracesSent: 8,
    reflectionsReceived: 20,
    joyWaveTrend: [0.3, 0.45, 0.6, 0.55, 0.7, 0.75, 0.82],
  );

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      backgroundColor: Colors.grey[50],
      body: SafeArea(
        child: CustomScrollView(
          slivers: [
            _buildAppBar(),
            SliverPadding(
              padding: const EdgeInsets.all(16),
              sliver: SliverList(
                delegate: SliverChildListDelegate([
                  _buildRoleFlowCard(),
                  const SizedBox(height: 16),
                  Row(
                    children: [
                      Expanded(
                        child: _buildEmotionBalanceCard(),
                      ),
                      const SizedBox(width: 16),
                      Expanded(
                        child: _buildSocialEchoCard(),
                      ),
                    ],
                  ),
                  const SizedBox(height: 16),
                  JoyWaveCard(
                    joyWaveScore: _metricsData.joyWave,
                    spontaneousReturns: _metricsData.spontaneousReturns,
                    positiveEmotions: _metricsData.positiveEmotions,
                    trendData: _metricsData.joyWaveTrend,
                    trendDirection: _calculateTrend(_metricsData.joyWaveTrend),
                  ),
                  const SizedBox(height: 16),
                  _buildInsightsCard(),
                  const SizedBox(height: 32),
                ]),
              ),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildAppBar() {
    return SliverAppBar(
      expandedHeight: 120,
      pinned: true,
      flexibleSpace: FlexibleSpaceBar(
        title: const Text(
          'Your Journey',
          style: TextStyle(
            fontWeight: FontWeight.bold,
            letterSpacing: 0.5,
          ),
        ),
        background: Container(
          decoration: const BoxDecoration(
            gradient: LinearGradient(
              begin: Alignment.topLeft,
              end: Alignment.bottomRight,
              colors: [
                Color(0xFF4A90E2),
                Color(0xFF7ED321),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildRoleFlowCard() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Container(
        padding: const EdgeInsets.all(24),
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(16),
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              const Color(0xFF4A90E2).withOpacity(0.1),
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
                const Row(
                  children: [
                    Icon(
                      Icons.trending_up,
                      color: Color(0xFF4A90E2),
                      size: 28,
                    ),
                    SizedBox(width: 12),
                    Text(
                      'RoleFlow',
                      style: TextStyle(
                        fontSize: 22,
                        fontWeight: FontWeight.bold,
                        color: Colors.black87,
                      ),
                    ),
                  ],
                ),
                Container(
                  padding: const EdgeInsets.symmetric(horizontal: 16, vertical: 8),
                  decoration: BoxDecoration(
                    color: const Color(0xFF4A90E2).withOpacity(0.2),
                    borderRadius: BorderRadius.circular(12),
                  ),
                  child: Text(
                    '${(_metricsData.roleFlow * 100).toInt()}%',
                    style: const TextStyle(
                      fontSize: 20,
                      fontWeight: FontWeight.bold,
                      color: Color(0xFF4A90E2),
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 4),
            Text(
              'Consistency of your role progression',
              style: TextStyle(
                fontSize: 13,
                color: Colors.grey[600],
              ),
            ),
            const SizedBox(height: 20),
            ClipRRect(
              borderRadius: BorderRadius.circular(10),
              child: LinearProgressIndicator(
                value: _metricsData.roleFlow,
                minHeight: 12,
                backgroundColor: Colors.grey[200],
                valueColor: const AlwaysStoppedAnimation<Color>(
                  Color(0xFF4A90E2),
                ),
              ),
            ),
            const SizedBox(height: 20),
            Row(
              mainAxisAlignment: MainAxisAlignment.spaceBetween,
              children: [
                _buildStatChip(
                  icon: Icons.check_circle_outline,
                  label: 'Scenes completed',
                  value: '${_metricsData.completedScenes}/${_metricsData.totalScenes}',
                  color: const Color(0xFF7ED321),
                ),
                _buildStatChip(
                  icon: Icons.calendar_today,
                  label: 'Streak',
                  value: '${_metricsData.consecutiveDays} days',
                  color: const Color(0xFFF5A623),
                ),
                _buildStatChip(
                  icon: Icons.bolt,
                  label: 'Multiplier',
                  value: '${_metricsData.consistencyMultiplier}x',
                  color: const Color(0xFF4A90E2),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildEmotionBalanceCard() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Row(
              children: [
                Icon(
                  Icons.sentiment_satisfied_alt,
                  color: Color(0xFF7ED321),
                  size: 24,
                ),
                SizedBox(width: 8),
                Expanded(
                  child: Text(
                    'Emotion Balance',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: Colors.black87,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Center(
              child: Text(
                '${(_metricsData.emotionBalance * 100).toInt()}%',
                style: const TextStyle(
                  fontSize: 36,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFF7ED321),
                ),
              ),
            ),
            const SizedBox(height: 12),
            EmotionBalanceIndicator(
              emotionBalance: _metricsData.emotionBalance,
              confidentCount: _metricsData.confidentCount,
              nervousCount: _metricsData.nervousCount,
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildSocialEchoCard() {
    return Card(
      elevation: 4,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            const Row(
              children: [
                Icon(
                  Icons.groups,
                  color: Color(0xFFE91E63),
                  size: 24,
                ),
                SizedBox(width: 8),
                Expanded(
                  child: Text(
                    'Social Echo',
                    style: TextStyle(
                      fontSize: 16,
                      fontWeight: FontWeight.w600,
                      color: Colors.black87,
                    ),
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            Center(
              child: Text(
                _metricsData.socialEcho.toString(),
                style: const TextStyle(
                  fontSize: 36,
                  fontWeight: FontWeight.bold,
                  color: Color(0xFFE91E63),
                ),
              ),
            ),
            const SizedBox(height: 12),
            _buildMetricRow(
              Icons.send,
              'Traces sent',
              _metricsData.tracesSent,
              const Color(0xFF4A90E2),
            ),
            const SizedBox(height: 8),
            _buildMetricRow(
              Icons.waves,
              'Reflections',
              _metricsData.reflectionsReceived,
              const Color(0xFF7ED321),
            ),
          ],
        ),
      ),
    );
  }

  Widget _buildInsightsCard() {
    return Card(
      elevation: 2,
      shape: RoundedRectangleBorder(
        borderRadius: BorderRadius.circular(16),
      ),
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(
              children: [
                Icon(
                  Icons.lightbulb_outline,
                  color: Colors.amber[700],
                  size: 24,
                ),
                const SizedBox(width: 8),
                const Text(
                  'Insights',
                  style: TextStyle(
                    fontSize: 18,
                    fontWeight: FontWeight.w600,
                    color: Colors.black87,
                  ),
                ),
              ],
            ),
            const SizedBox(height: 16),
            ..._generateInsights().map((insight) {
              return Padding(
                padding: const EdgeInsets.only(bottom: 12),
                child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  children: [
                    Container(
                      margin: const EdgeInsets.only(top: 4),
                      width: 6,
                      height: 6,
                      decoration: BoxDecoration(
                        color: insight.color,
                        shape: BoxShape.circle,
                      ),
                    ),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Text(
                        insight.message,
                        style: TextStyle(
                          fontSize: 14,
                          color: Colors.grey[800],
                          height: 1.4,
                        ),
                      ),
                    ),
                  ],
                ),
              );
            }),
          ],
        ),
      ),
    );
  }

  Widget _buildStatChip({
    required IconData icon,
    required String label,
    required String value,
    required Color color,
  }) {
    return Column(
      children: [
        Icon(icon, size: 20, color: color),
        const SizedBox(height: 6),
        Text(
          value,
          style: TextStyle(
            fontSize: 16,
            fontWeight: FontWeight.bold,
            color: color,
          ),
        ),
        const SizedBox(height: 2),
        Text(
          label,
          textAlign: TextAlign.center,
          style: TextStyle(
            fontSize: 10,
            color: Colors.grey[600],
          ),
        ),
      ],
    );
  }

  Widget _buildMetricRow(IconData icon, String label, int value, Color color) {
    return Row(
      children: [
        Icon(icon, size: 16, color: color),
        const SizedBox(width: 8),
        Text(
          '$value ',
          style: const TextStyle(
            fontSize: 14,
            fontWeight: FontWeight.bold,
            color: Colors.black87,
          ),
        ),
        Text(
          label,
          style: TextStyle(
            fontSize: 13,
            color: Colors.grey[600],
          ),
        ),
      ],
    );
  }

  List<InsightModel> _generateInsights() {
    final insights = <InsightModel>[];

    if (_metricsData.consecutiveDays >= 7) {
      insights.add(InsightModel(
        message: 'Amazing! You\'ve maintained a 7-day streak. Your consistency multiplier is at ${_metricsData.consistencyMultiplier}x.',
        color: const Color(0xFF7ED321),
      ));
    }

    if (_metricsData.emotionBalance > 0.7) {
      insights.add(InsightModel(
        message: 'Your confidence is showing! ${(_metricsData.emotionBalance * 100).toInt()}% of your expressions are calm and confident.',
        color: const Color(0xFF7ED321),
      ));
    }

    if (_metricsData.joyWave > 30) {
      insights.add(InsightModel(
        message: 'High JoyWave detected! You\'re returning spontaneously and enjoying the journey.',
        color: const Color(0xFF4A90E2),
      ));
    }

    if (_metricsData.reflectionsReceived > 15) {
      insights.add(InsightModel(
        message: 'Your waves are resonating! ${_metricsData.reflectionsReceived} people reflected on your shared experiences.',
        color: const Color(0xFFE91E63),
      ));
    }

    if (insights.isEmpty) {
      insights.add(InsightModel(
        message: 'Keep going! Every scene brings you closer to your new self.',
        color: const Color(0xFF4A90E2),
      ));
    }

    return insights;
  }

  String _calculateTrend(List<double> data) {
    if (data.length < 2) return 'stable';
    final recent = data.sublist(data.length - 3);
    final avg = recent.reduce((a, b) => a + b) / recent.length;
    final prev = data[data.length - 4];

    if (avg > prev * 1.1) return 'up';
    if (avg < prev * 0.9) return 'down';
    return 'stable';
  }
}

/// Model for all v1.1 metrics
class MetricsModel {
  final double roleFlow;
  final int completedScenes;
  final int totalScenes;
  final int consecutiveDays;
  final double consistencyMultiplier;

  final double emotionBalance;
  final int confidentCount;
  final int nervousCount;

  final int joyWave;
  final int spontaneousReturns;
  final int positiveEmotions;

  final int socialEcho;
  final int tracesSent;
  final int reflectionsReceived;

  final List<double> joyWaveTrend;

  MetricsModel({
    required this.roleFlow,
    required this.completedScenes,
    required this.totalScenes,
    required this.consecutiveDays,
    required this.consistencyMultiplier,
    required this.emotionBalance,
    required this.confidentCount,
    required this.nervousCount,
    required this.joyWave,
    required this.spontaneousReturns,
    required this.positiveEmotions,
    required this.socialEcho,
    required this.tracesSent,
    required this.reflectionsReceived,
    required this.joyWaveTrend,
  });

  // TODO: Add fromJson for FFI integration
}

class InsightModel {
  final String message;
  final Color color;

  InsightModel({required this.message, required this.color});
}
