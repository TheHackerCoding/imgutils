use image::{GenericImageView, DynamicImage, GenericImage, Rgba};
use rand::{prelude::ThreadRng, Rng};
use recur_fn::{recur_fn, RecurFn};

pub struct Engine {
  rng: ThreadRng
}

impl Engine {
  pub fn new() -> Engine {
    Engine {
      rng: rand::thread_rng()
    }
  }
  
  fn general_in(x: [u8; 4]) -> u32 {
    let mut init: String = "".to_owned();
    for i in 0..3 {
      let mut str = x[i].to_string();
      let recursive = recur_fn(|lib, _n: u8| {
        if str.len() < 3 {
          str = "0".to_owned() + &str;
          lib(5);
        }
      });
      init = init + x[i].to_string().as_str();
      recursive.call(5);
    }

    // let conversion = format!("{:?}", &x);
    let result: u32 = init.parse().unwrap();
    result
  }

  fn random_color(&mut self) -> [u8; 4] {
    let mut result: [u8; 4] = [0; 4];
    for i in 0..3 {
      let number = self.rng.gen_range(0..=255);
      result[i] = number
    }
    result
  }

  fn average(x: [u8; 4], y: [u8; 4]) -> [u8; 4] {
    fn math(x: u8, y: u8) -> u16 {
      let x = x as u16;
      let y = y as u16;
      // println!("x: {}, y: {}", x, y);
      ((x+y)/2).into()
    }

    let mut result: [u8; 4] = [0; 4];
    result[0] = math(x[0], y[0]) as u8;
    result[1] = math(x[1], y[1]) as u8;
    result[2] = math(x[2], y[2]) as u8;
    result[3] = math(x[3], y[3]) as u8;
    result
  }

  pub fn curse(&mut self, img: DynamicImage) -> DynamicImage {
    println!("{:?}", img.color());
    // let __img = ImageBuffer::new(img.len)
    let mut _img = img.clone();
    for (x, y, mut pixel) in img.pixels() {
      
      let original = pixel.0;
      
      let mut _pixel = Engine::average(original, self.random_color());
      println!("{}", Engine::general_in(_pixel));
      // _pixel = Engine::average(_pixel, self.random_color());
      // println!("original: {:#?}, new: {:#?}", original, _pixel);
      pixel = Rgba(_pixel);
      _img.put_pixel(x, y, pixel);
      //I love lua!!!!!!     
    }
    return _img
  }
}