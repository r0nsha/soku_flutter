// import 'dart:ffi';

import 'package:device_sim/device_sim.dart';
import 'package:flutter/foundation.dart';
import 'package:flutter/material.dart';
import 'package:soku/shared.dart';
import 'ffi.dart' if (dart.library.html) 'ffi_web.dart';

const bool debugEnableDeviceSimulator = kDebugMode;

void main() => runApp(const MyApp());

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  @override
  Widget build(BuildContext context) {
    return DeviceSim(
        isEnabled: debugEnableDeviceSimulator,
        devices: const [googlePixel5, iphone13, iphone13ProMax, ipad129Gen5],
        builder: (context) {
          return MaterialApp(
              useInheritedMediaQuery: debugEnableDeviceSimulator,
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
        });
  }
}
