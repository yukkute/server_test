// ignore_for_file: prefer_final_fields

import "dart:convert";

import "package:mobx/mobx.dart";
import "package:shared_preferences/shared_preferences.dart";

import "user.dart";

part "generated/mobx/user_registry.g.dart";

class UserRegistry = _UserRegistry with _$UserRegistry;

Future<UserRegistry> loadUserRegistry() async {
  try {
    final prefs = await SharedPreferences.getInstance();
    final jsonString = prefs.getString("user_registry")!;
    return _UserRegistry.fromJson(jsonDecode(jsonString));
  } catch (e) {
    return UserRegistry();
  }
}

abstract class _UserRegistry with Store {
  final ObservableList<User> _users = ObservableList();

  @computed
  List<User> get users => _users;

  @observable
  int? _currentUser;

  @computed
  User? get currentUser => users.isEmpty
      ? null
      : _currentUser == null
          ? null
          : _users[_currentUser!];

  @action
  bool hasUser(String username) {
    final i = _users.indexWhere((u) => u.username == username);
    return i != -1;
  }

  @action
  void addUser(User user) {
    if (hasUser(user.username)) return;
    _users.add(user);
    print(toJson());
  }

  @action
  void removeUser(String username) {
    _users.removeWhere((u) => u.username == username);
    print(toJson());
  }

  @action
  void setCurrentUser(String username) {
    final i = _users.indexWhere((u) => username == u.username);
    _currentUser = i < 0 ? null : i;
    print("current user: $_currentUser");
  }

  Map<String, dynamic> toJson() => {
        "users": _users.map((user) => user.toJson()).toList(),
      };

  static UserRegistry fromJson(Map<String, dynamic> json) {
    final registry = UserRegistry();
    if (json["users"] != null) {
      for (final userJson in json["users"]) {
        registry.addUser(userFromJson(userJson));
      }
    }
    return registry;
  }

  @action
  Future<void> save() async {
    final prefs = await SharedPreferences.getInstance();
    final String jsonString = jsonEncode(toJson());
    await prefs.setString("user_registry", jsonString);
  }
}
