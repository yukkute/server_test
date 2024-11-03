import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";

import "user_registry.dart";
import "w_user_tile.dart";

class WUsersRegistry extends StatelessWidget {
  final UserRegistry registry;

  const WUsersRegistry({required this.registry, super.key});

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (context) {
        return ListView.builder(
          itemCount: registry.users.length - 1,
          itemBuilder: (context, index) => WUserTile(
            registry: registry,
            context: context,
            index: index,
          ),
        );
      },
    );
  }
}
