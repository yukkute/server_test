import "dart:async";

import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";
import "package:mobx/mobx.dart";

part "generated/mobx/w_mobx_counter.g.dart";

class MobxCounter = _MobxCounter with _$MobxCounter;

class WMobxCounter extends StatelessWidget {
  final MobxCounter counter;

  const WMobxCounter({required this.counter, super.key});

  @override
  Widget build(BuildContext context) {
    return Observer(
      builder: (_) {
        return Center(
          child: Padding(
            padding: const EdgeInsets.all(8.0),
            child: Text(counter.count.toString().padLeft(2, "0"),
                style: TextStyle(
                  fontSize: 16,
                  fontFamily: "monospace",
                )),
          ),
        );
      },
    );
  }
}

abstract class _MobxCounter with Store {
  @observable
  int count = 0;

  Timer? _timer;

  _MobxCounter() {
    _startTimer();
  }

  @action
  void dispose() {
    _timer?.cancel();
  }

  void _startTimer() {
    _timer = Timer.periodic(Duration(milliseconds: 100), (timer) {
      count = (count + 1) % 100;
    });
  }
}
