import 'package:flutter/cupertino.dart';

class ObjectTitle {
  final String text;
  final bool is_editable;

  const ObjectTitle(
    this.text, {
    this.is_editable = false,
  });
}

class Object extends StatelessWidget {
  final ObjectTitle title;

  const Object({super.key, required this.title});

  @override
  Widget build(BuildContext context) {
    return const Placeholder();
  }
}
