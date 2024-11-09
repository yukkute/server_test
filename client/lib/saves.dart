// ignore_for_file: prefer_final_fields

import "dart:convert";

import "package:collection/collection.dart";
import "package:mobx/mobx.dart";
import "package:shared_preferences/shared_preferences.dart";

import "save.dart";

part "generated/mobx/saves.g.dart";

Future<Saves> initSaves() async {
  try {
    final prefs = await SharedPreferences.getInstance();
    final jsonString = prefs.getString("saves")!;
    return _Saves.fromJson(jsonDecode(jsonString));
  } catch (e) {
    return Saves();
  }
}

class Saves = _Saves with _$Saves;

abstract class _Saves with Store {
  final ObservableList<Save> _users = ObservableList();

  @observable
  Save? currentSave;

  @computed
  (Save, int)? get currentSaveN {
    final u = _users.firstWhereOrNull((u) => u == currentSave);
    if (u == null) return null;

    final i = _users.indexOf(u);
    return (u, i);
  }

  @computed
  List<Save> get users => _users;

  @action
  Future<void> addUser(Save user) async {
    if (hasUser(user.name)) return;
    _users.add(user);
    await setCurrentUser(user);
    //print(toJson());
  }

  @action
  bool hasUser(String username) {
    final i = _users.indexWhere((u) => u.name == username);
    return i != -1;
  }

  @action
  Future<void> removeUser(String username) async {
    _users.removeWhere((u) => u.name == username);
    await setCurrentUser(null);
    //print(toJson());
  }

  @action
  Future<void> save() async {
    final prefs = await SharedPreferences.getInstance();
    final String jsonString = jsonEncode(toJson());
    await prefs.setString("user_registry", jsonString);
  }

  @action
  Future<void> setCurrentUser(Save? user) async {
    currentSave = _users.firstWhereOrNull((u) => u == user);
    currentSave ??= _users.firstOrNull;

    // Move current user to the front
    if (currentSave != null && _users.first != currentSave) {
      _users.remove(currentSave);
      _users.insert(0, currentSave!);
    }
    await save();

    //print("current user: ${currentUser?.username}");
  }

  Map<String, dynamic> toJson() => {
        "users": _users.map((user) => user.toJson()).toList(),
        "last_user": currentSave?.name,
      };

  static Saves fromJson(Map<String, dynamic> json) {
    final registry = Saves();
    if (json["users"] != null) {
      for (final userJson in json["users"]) {
        registry.addUser(saveFromJson(userJson));
      }
    }
    registry.setCurrentUser(
        registry.users.firstWhereOrNull((u) => u.name == json["last_user"]));
    return registry;
  }
}
