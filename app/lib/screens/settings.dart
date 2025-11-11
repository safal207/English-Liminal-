import 'package:flutter/material.dart';

class SettingsScreen extends StatelessWidget {
  const SettingsScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Settings'),
      ),
      body: ListView(
        children: [
          const _SectionHeader(title: 'Notifications'),
          SwitchListTile(
            title: const Text('Morning Warmup'),
            subtitle: const Text('08:00 AM'),
            value: true,
            onChanged: (value) {},
          ),
          SwitchListTile(
            title: const Text('Afternoon Ping'),
            subtitle: const Text('12:30 PM'),
            value: true,
            onChanged: (value) {},
          ),
          SwitchListTile(
            title: const Text('Evening Rinse'),
            subtitle: const Text('08:00 PM'),
            value: true,
            onChanged: (value) {},
          ),
          const Divider(),
          const _SectionHeader(title: 'Learning'),
          ListTile(
            title: const Text('Decay Rate'),
            subtitle: const Text('0.82 (default)'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {},
          ),
          ListTile(
            title: const Text('Ping Frequency'),
            subtitle: const Text('Adaptive'),
            trailing: const Icon(Icons.chevron_right),
            onTap: () {},
          ),
          const Divider(),
          const _SectionHeader(title: 'Data'),
          ListTile(
            title: const Text('Export Data'),
            subtitle: const Text('Export your progress and events'),
            trailing: const Icon(Icons.download),
            onTap: () {
              // TODO: Call native.exportData()
            },
          ),
          ListTile(
            title: const Text('Clear History'),
            subtitle: const Text('Reset all data'),
            trailing: const Icon(Icons.delete),
            onTap: () {},
          ),
          const Divider(),
          const _SectionHeader(title: 'About'),
          ListTile(
            title: const Text('Version'),
            subtitle: const Text('0.1.0'),
          ),
          ListTile(
            title: const Text('Rust Core'),
            subtitle: const Text('liminal_english_core 0.1.0'),
          ),
        ],
      ),
    );
  }
}

class _SectionHeader extends StatelessWidget {
  final String title;

  const _SectionHeader({required this.title});

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.fromLTRB(16, 16, 16, 8),
      child: Text(
        title.toUpperCase(),
        style: Theme.of(context).textTheme.labelSmall?.copyWith(
              color: Theme.of(context).colorScheme.primary,
              fontWeight: FontWeight.bold,
            ),
      ),
    );
  }
}
