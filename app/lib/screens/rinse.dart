import 'package:flutter/material.dart';

class RinseScreen extends StatelessWidget {
  const RinseScreen({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: const Text('Evening Rinse'),
      ),
      body: Center(
        child: Padding(
          padding: const EdgeInsets.all(24.0),
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              const Icon(
                Icons.nightlight_round,
                size: 64,
                color: Colors.indigo,
              ),
              const SizedBox(height: 24),
              Text(
                'Rinse Screen',
                style: Theme.of(context).textTheme.headlineMedium,
              ),
              const SizedBox(height: 16),
              const Text(
                'Evening reflection: phrase of the day, one improvement, one success.\nThis will be implemented in Milestone C.',
                textAlign: TextAlign.center,
                style: TextStyle(color: Colors.grey),
              ),
            ],
          ),
        ),
      ),
    );
  }
}
