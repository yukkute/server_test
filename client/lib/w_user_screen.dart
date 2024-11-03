import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";

import "user.dart";
import "user_registry.dart";
import "w_add_user.dart";
import "w_mobx_counter.dart";
import "w_users_registry.dart";

class WUserScreen extends StatelessWidget {
  final UserRegistry registry;

  const WUserScreen({required this.registry, super.key});

  Widget activeUser(BuildContext context) {
    return Observer(builder: (_) {
      late final User user;
      try {
        user = registry.users[0];
      } catch (_) {
        return SizedBox();
      }

      final emoji = Text(
        user.emoji ?? emojis.first,
        style: TextStyle(fontSize: 30),
      );

      final title = Row(children: [
        Expanded(
          child: Text(
            user.username,
            style: TextStyle(
              fontWeight: FontWeight.w600,
              fontSize: 20,
              overflow: TextOverflow.ellipsis,
            ),
            maxLines: 1,
          ),
        ),
        WMobxCounter(counter: MobxCounter()),
      ]);

      return Padding(
        padding: const EdgeInsets.all(8.0),
        child: Card.filled(
          elevation: 6,
          child: ListTile(
            leading: emoji,
            title: title,
          ),
        ),
      );
    });
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(
        title: Text("Users"),
        leading: TextButton(
          onPressed: () => Navigator.of(context).pop(),
          child: Icon(Icons.arrow_back),
        ),
      ),
      body: Column(
        children: [
          activeUser(context),
          SizedBox(height: 10),
          Expanded(child: WUsersRegistry(registry: registry)),
          WAddUser(registry: registry),
        ],
      ),
    );
  }
}
