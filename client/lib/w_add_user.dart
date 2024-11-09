import "package:flutter/material.dart";

import "save.dart";
import "saves.dart";
import "w_emoji_selector.dart";

class WAddUser extends StatefulWidget {
  final Saves registry;

  const WAddUser({required this.registry, super.key});

  @override
  _WAddUserState createState() => _WAddUserState();
}

class _WAddUserState extends State<WAddUser> {
  final GlobalKey<FormState> _formKey = GlobalKey<FormState>();
  final TextEditingController _usernameController = TextEditingController();

  String? _selectedEmoji;

  late final WEmojiSelector emojiWidget;

  @override
  Widget build(BuildContext context) {
    final usernameField = Expanded(
      child: TextFormField(
        autovalidateMode: AutovalidateMode.onUserInteraction,
        autocorrect: false,
        controller: _usernameController,
        decoration: InputDecoration(
          labelText: "Name",
          border: InputBorder.none,
        ),
        validator: _validateUsername,
      ),
    );

    final buttonActive = _usernameController.text.isNotEmpty &&
        _validateUsername(_usernameController.text) == null;

    final addUserButton = Center(
      child: TextButton(
        onPressed: buttonActive ? _registerUser : null,
        child: Center(
          child: Icon(Icons.add),
        ),
      ),
    );

    return Padding(
      padding: const EdgeInsets.all(16.0),
      child: Card.outlined(
        elevation: 6.0,
        child: Form(
          key: _formKey,
          child: Row(
            mainAxisAlignment: MainAxisAlignment.center,
            crossAxisAlignment: CrossAxisAlignment.center,
            children: [
              emojiWidget,
              SizedBox(width: 16.0),
              usernameField,
              SizedBox(width: 16.0),
              addUserButton,
            ],
          ),
        ),
      ),
    );
  }

  @override
  void dispose() {
    _usernameController.removeListener(_updateButtonState);
    _usernameController.dispose();
    super.dispose();
  }

  @override
  void initState() {
    super.initState();
    emojiWidget = WEmojiSelector(callback: (s) => _selectedEmoji = s);
    _usernameController.addListener(_updateButtonState);
  }

  void _registerUser() {
    if (_formKey.currentState?.validate() ?? false) {
      final name = _usernameController.text;

      final user = Save(
        name: name,
        emoji: _selectedEmoji,
      );
      widget.registry.addUser(user);

      _usernameController.clear();
      setState(() {});
    }
  }

  void _updateButtonState() {
    setState(() {});
  }

  String? _validateUsername(String? value) {
    if (value == null || value.isEmpty) {
      return null;
    }
    if (widget.registry.hasUser(value)) {
      return "Username is already taken";
    }
    return null;
  }
}
