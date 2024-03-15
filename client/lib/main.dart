import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';

import 'generated/data.pbgrpc.dart';

DataRequestClient? stub;

void main() async {
  ClientChannel? channel;
  try {
    channel = ClientChannel('localhost',
        port: 8000,
        options:
            const ChannelOptions(credentials: ChannelCredentials.insecure()));

    stub = DataRequestClient(channel,
        options: CallOptions(timeout: const Duration(milliseconds: 1000)));

    runApp(const MyApp());
  } catch (e) {
    print(e);
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

//int? serverResponse;

Future<int?> sendRequest() async {
  try {
    final request = CounterRequest()..version = 0;
    final response = await stub!.getCounter(request);
    return response.counter;
  } catch (e) {
    print("$e");
    return null;
  }
}

class _MyHomePageState extends State<MyHomePage> {
  String? _counter;

  void _setRequest() async {
    _counter = (await sendRequest())?.toString();
    setState(() {});
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        backgroundColor: Theme.of(context).colorScheme.inversePrimary,
        title: Text(widget.title),
      ),
      body: Center(
        child: Column(
          mainAxisAlignment: MainAxisAlignment.center,
          children: <Widget>[
            const Text(
              'Server response:',
            ),
            Text(
              '$_counter',
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
        ),
      ),
      floatingActionButton: FloatingActionButton(
        onPressed: _setRequest,
        tooltip: 'Increment',
        child: const Icon(Icons.add),
      ),
    );
  }
}
