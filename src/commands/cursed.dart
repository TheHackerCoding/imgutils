import 'package:args/command_runner.dart';
import 'package:image/image.dart';
import 'package:path/path.dart';
import '../utils.dart';
import 'dart:math';
import 'dart:io';



class CursedCommand extends Command {
  @override
  final name = "cursed";
  @override
  final description = "makes cursed images";
  @override
  final aliases = ["c"];
  CursedCommand() {
    argParser.addOption('limit', abbr: 'l', defaultsTo: '256');
  }

  @override
  void run() {
    var files = argResults?.rest;
    Random randomizer;
    var seed = globalResults?['randomizerSeed'];
    if (seed == 'none') {
      randomizer = Random();
    } else {
      randomizer = Random(int.parse(seed));
    }

    if (files!.isEmpty) {
      print("No files given. Exiting...");
      exit(1);
    }

    int average(int x, int y) => ((x + y) ~/ 2).toInt();

    int limit = int.parse(argResults?['limit']);

    void process(String _file) async {
      var _img = processImg(_file);
      var img = _img.img;
      var file = _img.file;

      for (var w = 0; w < img.width; w++) {
        for (var h = 0; h < img.height; h++) {
          var pixel = img.getPixel(w, h);
          // max is 0xFFFFFFFF
          var crystalPixel = average(pixel, randomizer.nextInt(limit));
          // print("x: $w, y: $h, color: $crystalPixel");
          img.setPixel(w, h, crystalPixel);
        }
      }

      var filename = basenameWithoutExtension(file.path);
      await File('${filename}_cursed.png').writeAsBytes(encodePng(img));
    }

    for (var element in files) {
      process(element);
    }
  }
}
