import "package:flutter/material.dart";

import "save.dart";

class WEmojiSelector extends StatefulWidget {
  final void Function(String)? callback;

  const WEmojiSelector({this.callback, super.key});

  @override
  State<WEmojiSelector> createState() => _WEmojiSelectorState();
}

class _WEmojiSelectorState extends State<WEmojiSelector> {
  static const _dimension = 60.0;

  String _selectedEmoji = emojis.first;

  @override
  Widget build(BuildContext context) {
    final b = TextButton(
      onPressed: _showEmojiSelector,
      child: Text(
        _selectedEmoji,
        style: TextStyle(fontSize: 30),
      ),
    );

    return b;
  }

  void _showEmojiSelector() {
    showModalBottomSheet<dynamic>(
      context: context,
      isScrollControlled: true,
      builder: (BuildContext context) {
        return LayoutBuilder(
          builder: (context, constraints) {
            final maxWidth = constraints.maxWidth * 0.6;
            final maxHeight = constraints.maxHeight * 0.7;

            return Padding(
              padding: const EdgeInsets.all(16.0),
              child: Container(
                constraints: BoxConstraints(
                  maxWidth: maxWidth,
                  maxHeight: maxHeight,
                ),
                child: GridView.builder(
                  shrinkWrap: true,
                  gridDelegate: SliverGridDelegateWithMaxCrossAxisExtent(
                    maxCrossAxisExtent: _dimension,
                    childAspectRatio: 1.0,
                  ),
                  itemCount: emojis.length,
                  itemBuilder: (context, index) {
                    return GestureDetector(
                      onTap: () {
                        setState(() {
                          _selectedEmoji = emojis[index];
                          if (widget.callback != null) {
                            widget.callback!(_selectedEmoji);
                          }
                        });
                        Navigator.pop(context);
                      },
                      child: SizedBox(
                        width: _dimension,
                        height: _dimension,
                        child: Center(
                          child: Text(emojis[index],
                              style: TextStyle(fontSize: 32)),
                        ),
                      ),
                    );
                  },
                ),
              ),
            );
          },
        );
      },
    );
  }
}
