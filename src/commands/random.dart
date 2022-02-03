import 'dart:io';
import 'dart:math';
import 'package:args/command_runner.dart';
import 'package:image/image.dart';

class RandomService {
  final int thing;
  final Image img;
  RandomService(this.thing, this.img);
}

class RandomCommand extends Command {
  @override
  final name = "random";
  @override
  final description = "makes random images";
  @override
  final aliases = ["r"];
  RandomCommand() {
    argParser.addOption('number', abbr: 'n', defaultsTo: '1');
    argParser.addOption('width', abbr: 'w', defaultsTo: '500');
    argParser.addOption('height', abbr: 'e', defaultsTo: '500');
    argParser.addOption('offset', abbr: 'o', defaultsTo: '0');
    argParser.addOption('name', defaultsTo: 'random_');
  }

  @override
  void run() {
    Random randomizer;
    var seed = globalResults?['randomizerSeed'];
    if (seed == 'none') {
      randomizer = Random();
    } else {
      randomizer = Random(int.parse(seed));
    }

    int random() => randomizer.nextInt(256 - int.parse(argResults?['offset']));

    int randomColor() => Color.fromRgb(random(), random(), random());

    void process(RandomService config) async {
      var width = config.img.width;
      var height = config.img.height;
      for (var i = 0; i < width; i++) {
        for (var j = 0; j < height; j++) {
          drawPixel(config.img, i, j, randomColor());
        }
      }
      await File("${argResults?['name']}${config.thing}.png")
          .writeAsBytes(encodePng(config.img));
    }

    void create() async {
      var width = argResults?['width'];
      var height = argResults?['height'];
      var _width = int.parse(width);
      var _height = int.parse(height);
      for (var i = 1; i < int.parse(argResults?['number']) + 1; i++) {
        // await Isolate.spawn(process, RandomService(i, Image(_width, _height)));
        process(RandomService(i, Image(_width, _height)));
      }
    }

    create();
  }
}
