import 'package:flutter/material.dart';
import 'package:grpc/grpc.dart';

import 'generated/data.pbgrpc.dart';

MoreOnigiriServicesClient? stub;

void main() async {
  ClientChannel? channel;
  try {
    channel = ClientChannel('localhost',
        port: 8000,
        options:
            const ChannelOptions(credentials: ChannelCredentials.insecure()));

    stub = MoreOnigiriServicesClient(channel,
        options: CallOptions(timeout: const Duration(seconds: 1000)));

    runApp(const MyApp());
  } catch (e) {
    // ignore: avoid_print
    print(e);
  }
}

class CounterWidget extends StatefulWidget {
  const CounterWidget({super.key});

  @override
  CounterWidgetState createState() => CounterWidgetState();
}

class CounterWidgetState extends State<CounterWidget> {
  late final Stream<DataResponse> _dataStream;

  @override
  void initState() {
    super.initState();
    final request = DataRequest()..version = 0;
    _dataStream = stub!.getData(request);
  }

  @override
  Widget build(BuildContext context) {
    return StreamBuilder<DataResponse>(
      stream: _dataStream,
      builder: (context, snapshot) {
        if (snapshot.hasError) {
          return Text('Error: ${snapshot.error}');
        } else {
          return Text('Counter: ${snapshot.data?.counter}');
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
            Text(
              'Server response:',
            ),
            CounterWidget(),
          ],
        ),
      ),
    );
  }
}
