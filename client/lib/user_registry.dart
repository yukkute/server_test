// ignore_for_file: prefer_final_fields

import "dart:convert";

import "package:collection/collection.dart";
import "package:mobx/mobx.dart";
import "package:shared_preferences/shared_preferences.dart";

import "user.dart";

part "generated/mobx/user_registry.g.dart";

Future<UserRegistry> loadUserRegistry() async {
  try {
    final prefs = await SharedPreferences.getInstance();
    final jsonString = prefs.getString("user_registry")!;
    return _UserRegistry.fromJson(jsonDecode(jsonString));
  } catch (e) {
    return UserRegistry();
  }
}

class UserRegistry = _UserRegistry with _$UserRegistry;

abstract class _UserRegistry with Store {
  final ObservableList<User> _users = ObservableList();

  @observable
  User? currentUser;

  @computed
  (User, int)? get currentUserN {
    final u = _users.firstWhereOrNull((u) => u == currentUser);
    if (u == null) return null;

    final i = _users.indexOf(u);
    return (u, i);
  }

  @computed
  List<User> get users => _users;

  @action
  Future<void> addUser(User user) async {
    if (hasUser(user.username)) return;
    _users.add(user);
    await setCurrentUser(user);
    //print(toJson());
  }

  @action
  bool hasUser(String username) {
    final i = _users.indexWhere((u) => u.username == username);
    return i != -1;
  }

  @action
  Future<void> removeUser(String username) async {
    _users.removeWhere((u) => u.username == username);
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
  Future<void> setCurrentUser(User? user) async {
    currentUser = _users.firstWhereOrNull((u) => u == user);
    currentUser ??= _users.firstOrNull;

    // Move current user to the front
    if (currentUser != null && _users.first != currentUser) {
      _users.remove(currentUser);
      _users.insert(0, currentUser!);
    }
    await save();

    //print("current user: ${currentUser?.username}");
  }

  Map<String, dynamic> toJson() => {
        "users": _users.map((user) => user.toJson()).toList(),
        "last_user": currentUser?.username,
      };

  static UserRegistry fromJson(Map<String, dynamic> json) {
    final registry = UserRegistry();
    if (json["users"] != null) {
      for (final userJson in json["users"]) {
        registry.addUser(userFromJson(userJson));
      }
    }
    registry.setCurrentUser(registry.users
        .firstWhereOrNull((u) => u.username == json["last_user"]));
    return registry;
  }
}
