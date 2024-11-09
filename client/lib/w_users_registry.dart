import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";

import "saves.dart";
import "w_user_tile.dart";

class WUsersRegistry extends StatelessWidget {
  final Saves registry;

  const WUsersRegistry({required this.registry, super.key});

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) {
        return ListView.builder(
          itemCount: registry.users.length - 1,
          itemBuilder: (context, index) => WSaveSlot(
            registry: registry,
            context: context,
            index: index,
          ),
        );
      },
    );
  }
}
