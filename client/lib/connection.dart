import 'dart:async';
import 'dart:io';

import 'package:grpc/grpc.dart';
import 'package:network_info_plus/network_info_plus.dart';

import 'generated/protobuf/data.pbgrpc.dart';

bool connected = false;
MoreOnigiriServicesClient? stub;
ClientChannel? _channel;

final info = NetworkInfo();
InternetAddress? localIp;

int? localPort;

typedef Server = (InternetAddress, int);

Future<Set<Server>> listenActiveServers() async {
  final ip = InternetAddress('233.252.0.0');
  const port = 4445;

  final socket = await RawDatagramSocket.bind(InternetAddress.anyIPv4, port);
  final activeServers = <(InternetAddress, int)>{};

  socket.joinMulticast(ip);

  socket.listen((RawSocketEvent event) {
    Datagram? datagram = socket.receive();
    if (datagram == null) return;

    //print("datagram catched: $datagram\n");

    String message = String.fromCharCodes(datagram.data).trim();

    //print("message: $message");

    const header = 'mo_';

    if (!message.startsWith(header)) return;

    int? portNumber = int.tryParse(message.substring(header.length));
    if (portNumber == null) return;
    if (portNumber < 0 || portNumber > 65535) return;

    //print("address: ${datagram.address}, port: $portNumber");

    activeServers.add((datagram.address, portNumber));
  });

  await Future.delayed(const Duration(seconds: 1));

  socket.close();

  return activeServers;
}

Future<Server?> findLocalServer() async {
  localIp ??= await () async {
    final s = await info.getWifiIP();
    return s == null ? null : InternetAddress(s);
  }();
  if (localIp == null) return null;

  final servers = await listenActiveServers();
  if (servers.isEmpty) return null;

  servers.retainWhere((s) => (s.$1 == localIp));

  if (servers.isEmpty) return null;

  if (servers.length > 1) {
    print(
        'Multiple local servers found. Which one exactly used is unspecified');
  }

  return servers.first;
}

Future<void> disconnect() async {
  connected = false;
  stub = null;
  await _channel?.terminate();
  _channel = null;
}

Future<void> tryToConnect() async {
  await disconnect();

  try {
    //final server = (await findLocalServer())!;

    final Server server = (InternetAddress.loopbackIPv4, localPort!);

    _channel = ClientChannel(
      server.$1,
      port: server.$2,
      options: const ChannelOptions(credentials: ChannelCredentials.insecure()),
    );

    stub = MoreOnigiriServicesClient(_channel!, options: CallOptions());
  } catch (e) {
    await disconnect();
  }
}

void stayConnected() {
  Timer.periodic(const Duration(seconds: 2), (Timer t) async {
    if (stub == null) {
      connected = false;
    } else {
      connected = (await stub?.sendPing(Empty()).catchError((_) {
        disconnect();
        return Pong(port: '');
      }))!
          .port
          .isNotEmpty;
    }

    if (!connected) {
      await tryToConnect();
    } else {
      print("connected");
    }
  });
}
