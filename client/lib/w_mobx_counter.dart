import "dart:async";
import "package:mobx/mobx.dart";
import "package:flutter/material.dart";
import "package:flutter_mobx/flutter_mobx.dart";

part "generated/mobx/w_mobx_counter.g.dart";

class MobxCounter = _MobxCounter with _$MobxCounter;

abstract class _MobxCounter with Store {
  @observable
  int count = 0;

  Timer? _timer;

  _MobxCounter() {
    _startTimer();
  }

  void _startTimer() {
    _timer = Timer.periodic(Duration(milliseconds: 100), (timer) {
      count = (count + 1) % 100;
    });
  }

  @action
  void dispose() {
    _timer?.cancel();
  }
}

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
                  fontSize: 24,
                  fontFamily: "monospace",
                )),
          ),
        );
      },
    );
  }
}
