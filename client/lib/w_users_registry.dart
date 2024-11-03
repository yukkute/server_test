import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";

import "user.dart";
import "user_registry.dart";
import "w_mobx_counter.dart";

class WUsersRegistry extends StatelessWidget {
  const WUsersRegistry({required this.registry, super.key});

  final UserRegistry registry;

  @override
  Widget build(BuildContext context) {
    return Observer(
      warnWhenNoObservables: true,
      builder: (_) {
        return ListView.builder(
          itemCount: registry.users.length,
          itemBuilder: (context, index) {
            final user = registry.users[index];

            return SizedBox(
              height: 60,
              child: Row(
                children: [
                  WMobxCounter(counter: MobxCounter()),
                  Text(user.emoji ?? emojis.first),
                  Text(user.username),
                  Spacer(),
                  ElevatedButton(
                    onPressed: () => registry.setCurrentUser(user.username),
                    child: Icon(Icons.compare_arrows),
                  ),
                  ElevatedButton(
                    onPressed: () => registry.removeUser(user.username),
                    child: Icon(Icons.delete),
                  ),
                ],
              ),
            );
          },
        );
      },
    );
  }
}
