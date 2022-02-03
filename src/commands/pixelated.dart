import 'package:args/command_runner.dart';
import 'package:image/image.dart';
import 'package:path/path.dart';
import '../utils.dart';
import 'dart:io';


class PixelatedCommand extends Command {
  @override
  final name = "pixelated";
  @override
  final description = "smol img";
  @override
  final aliases = ["pixel", "nft", "p"];
  
  PixelatedCommand() {
    argParser.addOption('size', abbr: 's', defaultsTo: '5');
  }

  @override
  void run() {
    var files = argResults?.rest;

    if (files!.isEmpty) {
      print("No files given. Exiting...");
      exit(1);
    }
    
    var size = int.parse(argResults?['size']);
    
    void process(String _file) async {
      var _img = processImg(_file);
      var img = _img.img;
      var file = _img.file;

      var pixelImg = pixelate(img, size);
      
      var filename = basenameWithoutExtension(file.path);
      await File('${filename}_pixels.png').writeAsBytes(encodePng(pixelImg));
    }

    for (var element in files) {
      process(element);
    }

  }
}
