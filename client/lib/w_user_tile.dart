import "package:flutter/gestures.dart";
import "package:flutter/material.dart";
import "package:flutter_slidable/flutter_slidable.dart";

import "user.dart";
import "user_registry.dart";
import "w_mobx_counter.dart";

class WUserTile extends StatefulWidget {
  final UserRegistry registry;

  final BuildContext context;
  final int index;
  const WUserTile({
    required this.registry,
    required this.context,
    required this.index,
    super.key,
  });

  @override
  _WUserTileState createState() => _WUserTileState();
}

class _WUserTileState extends State<WUserTile> with TickerProviderStateMixin {
  late final AnimationController _circleController;
  late final Animation<double> _circleAnimation;
  late final SlidableController _slideController;

  @override
  Widget build(BuildContext context) {
    late final User user;
    try {
      user = widget.registry.users[widget.index + 1];
    } catch (_) {
      return const SizedBox();
    }

    final title = Row(children: [
      Expanded(
        child: Text(
          user.username,
          style: const TextStyle(overflow: TextOverflow.ellipsis),
          maxLines: 1,
        ),
      ),
      WMobxCounter(counter: MobxCounter()),
    ]);

    final emoji = SizedBox(
      child: Stack(
        alignment: Alignment.center,
        children: [
          Text(
            user.emoji ?? emojis.first,
            style: const TextStyle(fontSize: 24),
          ),
          CircularProgressIndicator(
            value: _circleAnimation.value,
            valueColor: AlwaysStoppedAnimation<Color>(
              Theme.of(context).highlightColor,
            ),
          ),
        ],
      ),
    );

    return Slidable(
      controller: _slideController,
      dragStartBehavior: DragStartBehavior.start,
      startActionPane: ActionPane(
        openThreshold: 0.28,
        closeThreshold: 0.28,
        extentRatio: 0.3,
        motion: const ScrollMotion(),
        children: [
          SlidableAction(
            onPressed: (_) => widget.registry.removeUser(user.username),
            backgroundColor: const Color.fromARGB(255, 249, 73, 73),
            foregroundColor: Colors.white,
            icon: Icons.delete_outlined,
            label: "Delete",
          ),
        ],
      ),
      child: GestureDetector(
        onTapDown: (_) => _startLongPress(user),
        onTapUp: (_) => _endLongPress(),
        child: ListTile(
          leading: emoji,
          title: title,
        ),
      ),
    );
  }

  @override
  void dispose() {
    _circleController.dispose();
    _slideController.dispose();
    super.dispose();
  }

  @override
  void initState() {
    super.initState();
    _slideController = SlidableController(this);

    _circleController = AnimationController(
        vsync: this, duration: const Duration(milliseconds: 600));

    _circleAnimation = CurvedAnimation(
      parent: _circleController,
      curve: Curves.easeInOut, // Change the curve here
    );

    _circleController.addListener(
      () => setState(() {
        if (_circleAnimation.isAnimating) _slideController.ratio = 0;
      }),
    );
  }

  void _endLongPress() {
    setState(() {
      _circleController.reverse();
    });
  }

  void _startLongPress(User user) {
    if (_slideController.ratio != 0) return;
    setState(() {
      _circleController.forward();
    });

    _circleController.addStatusListener((status) {
      if (status == AnimationStatus.completed) {
        widget.registry.setCurrentUser(user);
        _circleController.reset();
      }
    });
  }
}
