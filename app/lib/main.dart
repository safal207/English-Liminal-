import 'package:flutter/material.dart';
import 'package:path_provider/path_provider.dart';
import 'package:provider/provider.dart';
import 'dart:io';

// Screens
import 'screens/home.dart';
import 'screens/warmup.dart';
import 'screens/ping.dart';
import 'screens/rinse.dart';
import 'screens/settings.dart';

// Services (will be implemented with FFI bridge)
// import 'bridge/bridge.generated.dart' as native;

void main() async {
  WidgetsFlutterBinding.ensureInitialized();

  // Initialize storage and load scripts
  // TODO: Uncomment when FFI bridge is generated
  /*
  final dbPath = await _getDbPath();
  await native.initStorage(dbPath: dbPath);

  // Load YAML scripts from assets
  final scriptsPath = 'assets/scripts';
  final count = await native.loadScriptsFromDir(dir: scriptsPath);
  print('Loaded $count scripts');
  */

  runApp(
    MultiProvider(
      providers: [
        // Add providers here (ScriptProvider, StatsProvider, etc.)
      ],
      child: const LiminalApp(),
    ),
  );
}

Future<String> _getDbPath() async {
  final dir = await getApplicationDocumentsDirectory();
  return '${dir.path}/liminal.db';
}

class LiminalApp extends StatelessWidget {
  const LiminalApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'English Liminal',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MainNavigator(),
      debugShowCheckedModeBanner: false,
    );
  }
}

class MainNavigator extends StatefulWidget {
  const MainNavigator({super.key});

  @override
  State<MainNavigator> createState() => _MainNavigatorState();
}

class _MainNavigatorState extends State<MainNavigator> {
  int _currentIndex = 0;

  final List<Widget> _screens = const [
    HomeScreen(),
    WarmupScreen(),
    PingScreen(),
    RinseScreen(),
    SettingsScreen(),
  ];

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      body: IndexedStack(
        index: _currentIndex,
        children: _screens,
      ),
      bottomNavigationBar: BottomNavigationBar(
        currentIndex: _currentIndex,
        onTap: (index) => setState(() => _currentIndex = index),
        type: BottomNavigationBarType.fixed,
        selectedItemColor: Theme.of(context).colorScheme.primary,
        unselectedItemColor: Colors.grey,
        items: const [
          BottomNavigationBarItem(
            icon: Icon(Icons.home),
            label: 'Home',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.wb_sunny),
            label: 'Warmup',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.notifications_active),
            label: 'Ping',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.nightlight_round),
            label: 'Rinse',
          ),
          BottomNavigationBarItem(
            icon: Icon(Icons.settings),
            label: 'Settings',
          ),
        ],
      ),
    );
  }
}
