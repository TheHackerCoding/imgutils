import 'package:args/command_runner.dart';
import 'package:image/image.dart';
import 'dart:math';
import 'dart:io';

import 'package:mime/mime.dart';
import 'package:path/path.dart';

class CursedCommand extends Command {
  final name = "cursed";
  final description = "makes cursed images";

  CursedCommand() {
    argParser.addOption('limit', abbr: 'l', defaultsTo: '256');
  }

  void run() {
    var files = argResults?.rest;
    Random randomizer;
    var seed = globalResults?['randomizerSeed'];
    if (seed == 'none') {
      randomizer = Random();
    } else {
      randomizer = Random(int.parse(seed));
    }

    if (files?.length == 0) {
      print("No files given. Exiting...");
      exit(1);
    }

    int average(int x, int y) => ((x + y) ~/ 2).toInt();

    int limit = int.parse(argResults?['limit']);

    void process(String file) async {
      var _file = File(file);
      if (_file.existsSync() == false) {
        print("File $file doesn't exit. Exiting...");
        exit(1);
      }

      var mime = lookupMimeType(file);
      var _mime = mime?.split("/");
      if (_mime?.first != 'image') {
        print('Image file not give. Exiting...');
        exit(1);
      }
      var img = decodeImage(await _file.readAsBytes());

      for (var w = 0; w < img!.width; w++) {
        for (var h = 0; h < img.height; h++) {
          var pixel = img.getPixel(w, h);
          // max is 0xFFFFFFFF
          var crystalPixel = average(pixel, randomizer.nextInt(limit));
          // print("x: $w, y: $h, color: $crystalPixel");
          img.setPixel(w, h, crystalPixel);
        }
      }

      var filename = basenameWithoutExtension(_file.path);
      await File('${filename}_crystalized.png').writeAsBytes(encodePng(img));
    }

    files?.forEach((element) {
      process(element);
    });
  }
}
