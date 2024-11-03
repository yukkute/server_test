import "dart:math";

import "package:mobx/mobx.dart";

part "generated/mobx/user.g.dart";

const List<String> emojis = <String>[
  "😀", "😂", "😍", "😎", "😢", //
  "😡", "😱", "🎉", "❤️", "👍", //
  "🥳", "🤔", "😇", "😜", "😏", //
  "😋", "😴", "😬", "🤗", "👻", //
  "🧙‍♂️", "🦸‍♀️", "👑", "🤖", "🐉", //
  "🧛‍♂️", "👺", "🧝‍♀️", "🧜‍♂️", "🧟‍♂️", //
  "🐻", "🐼", "🦄", "🌈", "🚀", //
  "🪐"
];

String generateRandomString(int length) {
  const characters =
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  final random = Random();
  return List.generate(
      length, (_) => characters[random.nextInt(characters.length)]).join("");
}

User userFromJson(Map<String, dynamic> json) {
  return User(
      username: json["username"]!,
      emoji: json["emoji"],
      password: json["password"],
      key: json["passwordSub"]);
}

class User = _User with _$User;

abstract class _User with Store {
  final String username;
  String? emoji;
  String? _maybePassword;
  final String _key;

  _User({required this.username, String? key, this.emoji, String? password})
      : _maybePassword = password,
        _key = key ?? generateRandomString(64);

  String get password => _maybePassword ?? _key;
  bool get hasPassword => _maybePassword != null;

  set password(String? p) => _maybePassword = p;

  Map<String, String?> toJson() => {
        "username": username,
        "emoji": emoji,
        "password": _maybePassword,
        "key": _key
      };
}
