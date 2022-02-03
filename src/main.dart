import 'package:args/command_runner.dart';
// commands
import 'commands/cursed.dart';
import 'commands/pixelated.dart';
import 'commands/random.dart';
import 'dart:io';

void main(List<String> args) {
  var cli = CommandRunner("imgutils", "random image stuff")
    ..addCommand(RandomCommand())
    ..addCommand(CursedCommand())
    ..addCommand(PixelatedCommand())
    ..argParser.addOption('randomizerSeed',
        abbr: 's', aliases: ['seed', 'random'], defaultsTo: 'none')
    ..run(args).catchError((err) {
      if (err is! UsageException) throw err;
      print(err);
      exit(64); // Exit code 64 indicates a usage error.
    });
  // print('Hello world!');
}
