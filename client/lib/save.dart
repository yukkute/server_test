import "dart:math";

import "package:mobx/mobx.dart";

part "generated/mobx/save.g.dart";

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

String generateKey() {
  const characters =
      "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
  final random = Random();
  return List.generate(
    64,
    (_) => characters[random.nextInt(characters.length)],
  ).join("");
}

Save saveFromJson(Map<String, dynamic> json) {
  return Save(
    name: json["name"]!,
    emoji: json["emoji"],
    key: json["key"],
  );
}

class Save = _Save with _$Save;

abstract class _Save with Store {
  final String name;
  String? emoji;
  final String _key;

  _Save({required this.name, String? key, this.emoji})
      : _key = key ??
            generateKey(); // TODO: make key required and get it from server

  Map<String, String?> toJson() => {
        "username": name,
        "emoji": emoji,
        "key": _key,
      };
}
