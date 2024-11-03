import "package:flutter/material.dart";
import "package:grpc/grpc.dart";

import "generated/protobuf/empty.pb.dart";
import "generated/protobuf/data.pbgrpc.dart";
import "generated/protobuf/authentication.pbgrpc.dart";

import "rust_ffi.dart";
import "user_registry.dart";
import "w_user_register.dart";
import "w_users_registry.dart";

void main() async {
  final rust = RustBindings();
  final int port = rust.startLocalServer();

  final registry = await loadUserRegistry();

  runApp(MyApp(
    registry: registry,
    port: port,
  ));

  await registry.save();
}

class MyApp extends StatelessWidget {
  final int port;

  const MyApp({required this.port, required this.registry, super.key});

  final UserRegistry registry;

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
        title: "Flutter Demo",
        theme: ThemeData(
          colorScheme: ColorScheme.fromSeed(seedColor: Colors.deepPurple),
          useMaterial3: true,
        ),
        home: Scaffold(
            body: Column(
          children: [
            MyHomePage(title: "Flutter Demo Home Page", port: port),
            Spacer(),
            Expanded(child: WUsersRegistry(registry: registry)),
            WUserRegistration(registry: registry),
          ],
        )));
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({required this.title, required this.port, super.key});
  final String title;
  final int port;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;
  late ClientChannel channel;
  late MoTalkingClient talkingStub;
  late MoAuthClient authStub;
  String _username = "";
  String _password = "";
  SessionCredentials? _sessionCredentials;
  bool _fetched = false;

  @override
  void initState() {
    super.initState();
    _initializeGrpc();
  }

  void _initializeGrpc() {
    channel = ClientChannel(
      "localhost",
      port: widget.port,
      options: const ChannelOptions(
        credentials: ChannelCredentials.insecure(),
      ),
    );
    talkingStub = MoTalkingClient(channel);
    authStub = MoAuthClient(channel);
  }

  Future<void> _register() async {
    final request = UserCredentials()
      ..username = _username
      ..password = _password;
    try {
      await authStub.register(request);
    } finally {}
  }

  Future<void> _authenticate() async {
    final request = UserCredentials()
      ..username = _username
      ..password = _password;

    try {
      final response = await authStub.authenticate(request);
      setState(() {
        _sessionCredentials = response;
      });
    } finally {}
  }

  Future<void> _getClock() async {
    try {
      final stream = talkingStub.requestServerClock(Empty());
      await for (final _ in stream) {
        _getData();
      }
    } finally {}
  }

  Future<void> _getData() async {
    if (_sessionCredentials == null) {
      await _authenticate();
    }

    final request = MoClientDatagram()..sessionId = _sessionCredentials!;

    try {
      final response = await talkingStub.getData(request);
      setState(() {
        _fetched = true;
        _counter = response.counter;
      });
    } catch (e) {
      _sessionCredentials = null;
      _fetched = false;
    }
  }

  @override
  void dispose() {
    channel.shutdown();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Center(
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        children: <Widget>[
          const Text("Enter your credentials:"),
          TextField(
            decoration: const InputDecoration(labelText: "Username"),
            onChanged: (value) {
              setState(() {
                _username = value;
              });
            },
          ),
          TextField(
            decoration: const InputDecoration(labelText: "Password"),
            obscureText: true,
            onChanged: (value) {
              setState(() {
                _password = value;
              });
            },
          ),
          Row(
            children: [
              ElevatedButton(
                onPressed: _register,
                child: const Text("Register"),
              ),
              ElevatedButton(
                onPressed: _authenticate,
                child: const Text("Authenticate"),
              ),
            ],
          ),
          if (_sessionCredentials != null) ...[
            const Text("Authenticated successfully!"),
            Text(
              "Current counter value: $_counter",
              style: Theme.of(context).textTheme.headlineMedium,
            ),
          ],
          if (_sessionCredentials != null && !_fetched) ...[
            ElevatedButton(
              onPressed: _getClock,
              child: const Text("Fetch Counter Value"),
            ),
          ]
        ],
      ),
    );
  }
}
