import 'dart:async';

import 'package:client/connection.dart';
import 'package:client/generated/rust/mo_server/local_server.dart';
import 'package:flutter/material.dart';

import 'generated/rust/frb_generated.dart';

void main() async {
  // Initialize rust lib
  await RustLib.init();

  try {
    // Start local server
    localPort = await startLocalServer();
  } catch (e) {
    print(e);
  }

  localPort!;
  stayConnected();

  runApp(const MyApp());
}

class CounterWidget extends StatefulWidget {
  const CounterWidget({super.key});

  @override
  CounterWidgetState createState() => CounterWidgetState();
}

class CounterWidgetState extends State<CounterWidget> {
  Stream<DataResponse>? _dataStream;
  Timer? _timer;

  @override
  void initState() {
    stayConnected();
    super.initState();
  }

  @override
  void dispose() {
    _timer?.cancel();
    super.dispose();
  }

  void stayConnected() {
    _timer = Timer.periodic(const Duration(milliseconds: 10), (Timer t) async {
      if (!connected) {
        _dataStream = null;
        return;
      }
      if (_dataStream == null) {
        final request = DataRequest()..version = 0;
        try {
          _dataStream = stub?.getData(request);
          setState(() {});
        } catch (e) {
          _dataStream = null;
        }
      }
    });
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<DataResponse>(
      stream: _dataStream,
      builder: (context, snapshot) {
        if (snapshot.hasError || snapshot.data == null) {
          _dataStream = null;
          return const CircularProgressIndicator();
        } else {
          return Text(
            'Data: ${snapshot.data?.counter}',
            style: const TextStyle(
              fontFamily: "monospace",
            ),
          );
        }
      },
    );
  }
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Flutter Demo',
      theme: ThemeData(
        colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
        useMaterial3: true,
      ),
      home: const MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: const Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            CounterWidget(),
          ],
        ),
      ),
    );
  }
}
