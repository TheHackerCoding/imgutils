import 'package:args/command_runner.dart';
// commands
import 'commands/cursed.dart';
import 'commands/random.dart';

void main(List<String> args) {
  var cli = CommandRunner("imgutils", "random image stuff")
    ..addCommand(RandomCommand())
    ..addCommand(CursedCommand())
    ..argParser.addOption('randomizerSeed',
        abbr: 's', aliases: ['seed', 'random'], defaultsTo: 'none')
    ..run(args);
  // print('Hello world!');
}
