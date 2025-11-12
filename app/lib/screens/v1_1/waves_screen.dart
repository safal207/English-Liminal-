import 'package:flutter/material.dart';
import 'dart:math' as math;

/// Waves Screen - Social Resonance Visualization
///
/// Displays anonymous ResonanceTrace messages from other users.
/// Users can "reflect" on traces by tapping them.
class WavesScreen extends StatefulWidget {
  const WavesScreen({super.key});

  @override
  State<WavesScreen> createState() => _WavesScreenState();
}

class _WavesScreenState extends State<WavesScreen>
    with TickerProviderStateMixin {
  // TODO: Load from FFI API: get_recent_traces_json(None, 20)
  final List<ResonanceTraceModel> _traces = [
    ResonanceTraceModel(
      id: '1',
      roleId: 'qa_engineer_abroad',
      roleName: 'QA Engineer',
      message: 'I passed the interview! 🎉',
      reflections: 12,
      timestamp: DateTime.now().subtract(const Duration(hours: 2)),
    ),
    ResonanceTraceModel(
      id: '2',
      roleId: 'visa_journey',
      roleName: 'Visa Journey',
      message: 'Got my visa approved today. Dreams do come true.',
      reflections: 8,
      timestamp: DateTime.now().subtract(const Duration(hours: 5)),
    ),
    ResonanceTraceModel(
      id: '3',
      roleId: 'global_citizen',
      roleName: 'Global Citizen',
      message: 'First time ordering coffee without panic. Small wins.',
      reflections: 15,
      timestamp: DateTime.now().subtract(const Duration(days: 1)),
    ),
    ResonanceTraceModel(
      id: '4',
      roleId: 'family_abroad',
      roleName: 'Family Abroad',
      message: 'My kid made their first friend at kindergarten today ❤️',
      reflections: 23,
      timestamp: DateTime.now().subtract(const Duration(days: 2)),
    ),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: Container(
        decoration: const BoxDecoration(
          gradient: LinearGradient(
            begin: Alignment.topLeft,
            end: Alignment.bottomRight,
            colors: [
              Color(0xFF4A90E2), // Blue
              Color(0xFF7ED321), // Green
            ],
            stops: [0.0, 1.0],
          ),
        ),
        child: SafeArea(
          child: Column(
            crossAxisAlignment: CrossAxisAlignment.start,
            children: [
              _buildHeader(),
              Expanded(
                child: ListView.builder(
                  padding: const EdgeInsets.symmetric(horizontal: 16),
                  itemCount: _traces.length,
                  itemBuilder: (context, index) {
                    return WaveCard(
                      trace: _traces[index],
                      onReflect: () => _showReflectionDialog(_traces[index]),
                    );
                  },
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _buildHeader() {
    return Padding(
      padding: const EdgeInsets.all(24.0),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          const Text(
            'Waves',
            style: TextStyle(
              fontSize: 36,
              fontWeight: FontWeight.bold,
              color: Colors.white,
              letterSpacing: 1.2,
            ),
          ),
          const SizedBox(height: 8),
          Text(
            'Anonymous echoes from fellow travelers',
            style: TextStyle(
              fontSize: 16,
              color: Colors.white.withOpacity(0.8),
              letterSpacing: 0.5,
            ),
          ),
        ],
      ),
    );
  }

  void _showReflectionDialog(ResonanceTraceModel trace) {
    final controller = TextEditingController();

    showDialog(
      context: context,
      builder: (context) => AlertDialog(
        title: const Text('Reflect on this wave'),
        content: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(
              '"${trace.message}"',
              style: const TextStyle(
                fontStyle: FontStyle.italic,
                color: Colors.grey,
              ),
            ),
            const SizedBox(height: 16),
            TextField(
              controller: controller,
              decoration: const InputDecoration(
                hintText: 'Your reflection...',
                border: OutlineInputBorder(),
              ),
              maxLines: 3,
            ),
          ],
        ),
        actions: [
          TextButton(
            onPressed: () => Navigator.pop(context),
            child: const Text('Cancel'),
          ),
          ElevatedButton(
            onPressed: () {
              if (controller.text.isNotEmpty) {
                // TODO: Call FFI API: add_reflection_to_trace(trace.id, message)
                Navigator.pop(context);
                ScaffoldMessenger.of(context).showSnackBar(
                  const SnackBar(
                    content: Text('Your reflection was sent 🌊'),
                    duration: Duration(seconds: 2),
                  ),
                );
              }
            },
            child: const Text('Send Reflection'),
          ),
        ],
      ),
    );
  }
}

/// Wave Card - Animated card for each ResonanceTrace
class WaveCard extends StatefulWidget {
  final ResonanceTraceModel trace;
  final VoidCallback onReflect;

  const WaveCard({
    super.key,
    required this.trace,
    required this.onReflect,
  });

  @override
  State<WaveCard> createState() => _WaveCardState();
}

class _WaveCardState extends State<WaveCard>
    with SingleTickerProviderStateMixin {
  late AnimationController _controller;
  late Animation<double> _scaleAnimation;

  @override
  void initState() {
    super.initState();
    _controller = AnimationController(
      vsync: this,
      duration: const Duration(milliseconds: 2000),
    )..repeat(reverse: true);

    _scaleAnimation = Tween<double>(begin: 1.0, end: 1.05).animate(
      CurvedAnimation(parent: _controller, curve: Curves.easeInOut),
    );
  }

  @override
  void dispose() {
    _controller.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return ScaleTransition(
      scale: _scaleAnimation,
      child: Card(
        margin: const EdgeInsets.only(bottom: 16),
        shape: RoundedRectangleBorder(
          borderRadius: BorderRadius.circular(16),
        ),
        elevation: 4,
        child: InkWell(
          onTap: widget.onReflect,
          borderRadius: BorderRadius.circular(16),
          child: Padding(
            padding: const EdgeInsets.all(20.0),
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              children: [
                Row(
                  children: [
                    _buildRoleIcon(widget.trace.roleId),
                    const SizedBox(width: 12),
                    Expanded(
                      child: Column(
                        crossAxisAlignment: CrossAxisAlignment.start,
                        children: [
                          Text(
                            widget.trace.roleName,
                            style: const TextStyle(
                              fontSize: 14,
                              fontWeight: FontWeight.w600,
                              color: Color(0xFF4A90E2),
                            ),
                          ),
                          Text(
                            _formatTimestamp(widget.trace.timestamp),
                            style: TextStyle(
                              fontSize: 12,
                              color: Colors.grey[600],
                            ),
                          ),
                        ],
                      ),
                    ),
                    _buildReflectionsBadge(widget.trace.reflections),
                  ],
                ),
                const SizedBox(height: 12),
                Text(
                  widget.trace.message,
                  style: const TextStyle(
                    fontSize: 16,
                    height: 1.4,
                  ),
                ),
                const SizedBox(height: 12),
                Row(
                  children: [
                    Icon(
                      Icons.waves,
                      size: 16,
                      color: Colors.grey[600],
                    ),
                    const SizedBox(width: 4),
                    Text(
                      'Tap to reflect',
                      style: TextStyle(
                        fontSize: 12,
                        color: Colors.grey[600],
                        fontStyle: FontStyle.italic,
                      ),
                    ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }

  Widget _buildRoleIcon(String roleId) {
    IconData icon;
    Color color;

    switch (roleId) {
      case 'qa_engineer_abroad':
        icon = Icons.code;
        color = const Color(0xFF4A90E2);
        break;
      case 'visa_journey':
        icon = Icons.flight_takeoff;
        color = const Color(0xFFF5A623);
        break;
      case 'global_citizen':
        icon = Icons.public;
        color = const Color(0xFF7ED321);
        break;
      case 'family_abroad':
        icon = Icons.family_restroom;
        color = const Color(0xFFE91E63);
        break;
      default:
        icon = Icons.stars;
        color = const Color(0xFF4A90E2);
    }

    return Container(
      padding: const EdgeInsets.all(8),
      decoration: BoxDecoration(
        color: color.withOpacity(0.1),
        borderRadius: BorderRadius.circular(8),
      ),
      child: Icon(icon, size: 24, color: color),
    );
  }

  Widget _buildReflectionsBadge(int count) {
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 12, vertical: 6),
      decoration: BoxDecoration(
        gradient: const LinearGradient(
          colors: [Color(0xFF4A90E2), Color(0xFF7ED321)],
        ),
        borderRadius: BorderRadius.circular(12),
      ),
      child: Row(
        children: [
          const Icon(Icons.waves, size: 14, color: Colors.white),
          const SizedBox(width: 4),
          Text(
            '$count',
            style: const TextStyle(
              color: Colors.white,
              fontWeight: FontWeight.bold,
              fontSize: 12,
            ),
          ),
        ],
      ),
    );
  }

  String _formatTimestamp(DateTime timestamp) {
    final diff = DateTime.now().difference(timestamp);
    if (diff.inHours < 1) {
      return '${diff.inMinutes}m ago';
    } else if (diff.inHours < 24) {
      return '${diff.inHours}h ago';
    } else {
      return '${diff.inDays}d ago';
    }
  }
}

/// Model for ResonanceTrace data
class ResonanceTraceModel {
  final String id;
  final String roleId;
  final String roleName;
  final String message;
  final int reflections;
  final DateTime timestamp;

  ResonanceTraceModel({
    required this.id,
    required this.roleId,
    required this.roleName,
    required this.message,
    required this.reflections,
    required this.timestamp,
  });

  // TODO: Add fromJson factory for FFI integration
  factory ResonanceTraceModel.fromJson(Map<String, dynamic> json) {
    return ResonanceTraceModel(
      id: json['id'],
      roleId: json['role_id'],
      roleName: _roleIdToName(json['role_id']),
      message: json['message'],
      reflections: json['reflections']?.length ?? 0,
      timestamp: DateTime.parse(json['created_at']),
    );
  }

  static String _roleIdToName(String roleId) {
    switch (roleId) {
      case 'qa_engineer_abroad':
        return 'QA Engineer';
      case 'visa_journey':
        return 'Visa Journey';
      case 'global_citizen':
        return 'Global Citizen';
      case 'family_abroad':
        return 'Family Abroad';
      default:
        return 'Unknown Role';
    }
  }
}
