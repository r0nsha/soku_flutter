// import 'dart:ffi';

import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:soku/shared.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    // api.sudoku().then((sudoku) {
    //   logger.v("$sudoku.field0");
    // });

    return MaterialApp(
        title: 'Soku',
        theme: ThemeData(
          primarySwatch: Colors.blue,
        ),
        // home: const CalibrationScreen(),
        home: Scaffold(
          appBar: AppBar(
            title: const Text('DeviceSimulator Demo'),
          ),
          body: const Center(
            // child: Text('Hello multiple resolutions!'),
            child: Text('Hello multiple resolutions!'),
          ),
        ));
  }
}
