import "package:flutter/material.dart";
import "user.dart";
import "user_registry.dart";

class WUserRegistration extends StatefulWidget {
  const WUserRegistration({required this.registry, super.key});

  final UserRegistry registry;

  @override
  _WUserRegistrationState createState() => _WUserRegistrationState();
}

class _WUserRegistrationState extends State<WUserRegistration> {
  final TextEditingController _usernameController = TextEditingController();
  final TextEditingController _passwordController = TextEditingController();

  String _selectedEmoji = emojis.first;

  void _registerUser() {
    final username = _usernameController.text;
    final password = _passwordController.text;

    if (username.isEmpty) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text("Username cannot be empty.")),
      );
      return;
    }

    if (widget.registry.hasUser(username)) {
      ScaffoldMessenger.of(context).showSnackBar(
        SnackBar(content: Text("Username is already taken.")),
      );
      return;
    }

    final user = User(
      username: username,
      emoji: _selectedEmoji,
      password: password,
    );
    widget.registry.addUser(user);
    widget.registry.save();

    _usernameController.clear();
    _passwordController.clear();
    setState(() {
      _selectedEmoji = emojis.first;
    });
    ScaffoldMessenger.of(context).showSnackBar(
      SnackBar(content: Text("User registered successfully!")),
    );
  }

  void _showEmojiSelector() {
    showModalBottomSheet<dynamic>(
      context: context,
      isScrollControlled: true,
      constraints: BoxConstraints(
        maxWidth: MediaQuery.of(context).size.width * 0.9,
        maxHeight: MediaQuery.of(context).size.height * 0.7,
      ),
      builder: (BuildContext context) {
        return GridView.builder(
          gridDelegate: SliverGridDelegateWithFixedCrossAxisCount(
            crossAxisCount: 5,
          ),
          itemCount: emojis.length,
          itemBuilder: (context, index) {
            return IntrinsicHeight(
              child: GestureDetector(
                onTap: () {
                  setState(() {
                    _selectedEmoji = emojis[index];
                  });
                  Navigator.pop(context);
                },
                child: SizedBox.square(
                  child: Container(
                    padding: EdgeInsets.all(4.0),
                    child: Center(
                        child: Text(emojis[index],
                            style: TextStyle(fontSize: 32))),
                  ),
                ),
              ),
            );
          },
        );
      },
    );
  }

  @override
  Widget build(BuildContext context) {
    final emojiSelector = IntrinsicHeight(
      child: GestureDetector(
        onTap: _showEmojiSelector,
        child: SizedBox.square(
          child: Container(
            padding: EdgeInsets.all(2.0),
            decoration: BoxDecoration(
              border: Border.all(color: Colors.grey),
              borderRadius: BorderRadius.circular(8.0),
            ),
            child: Center(
              child: Text(
                _selectedEmoji,
                style: TextStyle(fontSize: 30),
              ),
            ),
          ),
        ),
      ),
    );

    final usernameField = Expanded(
      child: TextField(
        controller: _usernameController,
        decoration: InputDecoration(labelText: "Username"),
      ),
    );

    final passwordField = Expanded(
      child: TextField(
        controller: _passwordController,
        decoration: InputDecoration(labelText: "Password (optional)"),
        obscureText: true,
      ),
    );

    final padding = Padding(
      padding: const EdgeInsets.all(16.0),
      child: Row(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.center,
        children: [
          emojiSelector,
          SizedBox(width: 16.0),
          usernameField,
          SizedBox(width: 16.0),
          passwordField,
          Expanded(
            child: SizedBox.square(
              child: ElevatedButton(
                onPressed: _registerUser,
                style: ElevatedButton.styleFrom(
                  shape: RoundedRectangleBorder(
                      borderRadius: BorderRadius.circular(8)),
                ),
                child: Center(
                  child: Icon(Icons.add),
                ),
              ),
            ),
          ),
        ],
      ),
    );

    return padding;
  }
}
