import 'package:image/image.dart';
import 'package:mime/mime.dart';
import 'dart:io';

class ImgData {
  final Image img;
  final File file;

  ImgData(this.img, this.file);
}

ImgData processImg(String file) {
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
  var img = decodeImage(_file.readAsBytesSync());
  var result = ImgData(img!, _file);
  return result;
}
