import 'package:flutter/cupertino.dart';
import 'package:flutter/services.dart';

void main() {
  runApp(const MyApp());
}

class MyApp extends StatelessWidget {
  const MyApp({super.key});

  // This widget is the root of your application.
  @override
  Widget build(BuildContext context) {
    return const CupertinoApp(
      title: 'Flutter Demo',
      theme: CupertinoThemeData(primaryColor: CupertinoColors.activeBlue),
      home: MyHomePage(title: 'Flutter Demo Home Page'),
    );
  }
}

class MyHomePage extends StatefulWidget {
  const MyHomePage({super.key, required this.title});

  // This widget is the home page of your application. It is stateful, meaning
  // that it has a State object (defined below) that contains fields that affect
  // how it looks.

  // This class is the configuration for the state. It holds the values (in this
  // case the title) provided by the parent (in this case the App widget) and
  // used by the build method of the State. Fields in a Widget subclass are
  // always marked "final".

  final String title;

  @override
  State<MyHomePage> createState() => _MyHomePageState();
}

class _MyHomePageState extends State<MyHomePage> {
  int _counter = 0;

  void _incrementCounter() {
    setState(() {
      // This call to setState tells the Flutter framework that something has
      // changed in this State, which causes it to rerun the build method below
      // so that the display can reflect the updated values. If we changed
      // _counter without calling setState(), then the build method would not be
      // called again, and so nothing would appear to happen.
      _counter++;
    });
  }

  @override
  Widget build(BuildContext context) {
    // This method is rerun every time setState is called, for instance as done
    // by the _incrementCounter method above.
    //
    // The Flutter framework has been optimized to make rerunning build methods
    // fast, so that you can just rebuild anything that needs updating rather
    // than having to individually change instances of widgets.
    // return CupertinoTabScaffold(
    //     tabBar: CupertinoTabBar(items: const [
    //       BottomNavigationBarItem(icon: Icon(CupertinoIcons.add)),
    //       BottomNavigationBarItem(icon: Icon(CupertinoIcons.airplane))
    //     ]),
    //     tabBuilder: (context, tab) {
    //       return Placeholder();
    //     });
    return CupertinoPageScaffold(
      child: CupertinoTabScaffold(
          tabBar: CupertinoTabBar(items: const [
            BottomNavigationBarItem(icon: Icon(CupertinoIcons.add)),
            BottomNavigationBarItem(icon: Icon(CupertinoIcons.airplane))
          ]),
          tabBuilder: (context, tab) {
            return Center(
              // child: ,
              child: CupertinoContextMenu(
                actions: const [
                  CupertinoContextMenuAction(child: Text("Hi")),
                  CupertinoContextMenuAction(child: Text("Bye"))
                ],
                enableHapticFeedback: true,
                child: const Padding(padding: EdgeInsets.all(50), child: Text("Click me")),
              ),
            );
          }),
    );
  }
}
