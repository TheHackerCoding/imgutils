use image::{DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba};
use rand::{prelude::ThreadRng, Rng};
use std::sync::Arc;
use std::sync::Mutex;

pub struct Engine {
    rng: ThreadRng,
}

impl Engine {
    pub fn new() -> Engine {
        Engine {
            rng: rand::thread_rng(),
        }
    }

    fn random_color(&mut self, limit: Option<&str>) -> u32 {
        match limit {
            Some(x) => match x.parse::<u32>() {
                Ok(x) => {
                    //println!("nice number {}", x);
                    return self.rng.gen_range(0..=x);
                }
                Err(err) => {
                    println!("uh huh?");
                    return self.rng.gen_range(0..=255);
                }
            },
            None => return self.rng.gen_range(0..=255),
        }
    }

    fn average(x: u32, y: u32) -> u32 {
        // (x + y) / 2
        return x.wrapping_add(y).wrapping_div(2);
    }

    fn luminance_standard(color: u32) -> f64 {
        let red = (color & 0x000000FF) as u8;
        let green = ((color & 0x0000FF00) >> 8) as u8;
        let blue = ((color & 0x00FF0000) >> 16) as u8;

        //println!("Red: {}, Green: {}, Blue: {}", red, green, blue);

        // Calculate luminance using a common formula
        let luminance = 0.299 * (red as f64) + 0.587 * (green as f64) + 0.114 * (blue as f64);
        return luminance
    }

    pub fn darkness_check(color: u32) -> bool {
        let luminance = Self::luminance_standard(color);
        let is_dark = luminance < 100.0;
        return is_dark
    }

    pub fn lightness_check(color: u32) -> bool {
        let luminance = Self::luminance_standard(color);
        let is_light = luminance > 200.0;

        return is_light
    }
    pub fn curse(
        &mut self,
        img: DynamicImage,
        limit: Option<&str>,
        darkness: bool,
        lightness: bool,
    ) -> DynamicImage {
        println!("light {}", lightness);
        println!("dark {}", darkness);
        //println!("{:?}", img.color());
        // let __img = ImageBuffer::new(img.len)
        let mut _img = img.clone();
        for (x, y, pixel) in img.pixels() {
            let original = pixel.0;
            let rgba = u32::from_ne_bytes(original);
            let mut pixel;
            match lightness || darkness {
                true => {
                    //println!("rahh");
                    pixel = rgba;
                    if lightness {
                        if Engine::lightness_check(rgba) {
                            //println!("yes");
                            pixel = Engine::average(rgba, self.random_color(limit));
                        }
                    }
                    if darkness {
                        if Engine::darkness_check(rgba) {
                            //println!("yes");
                            pixel = Engine::average(rgba, self.random_color(limit));
                        }
                    }
                }
                false => {
                    pixel = Engine::average(rgba, self.random_color(limit));
                }
            }
            _img.put_pixel(x, y, Rgba(pixel.to_ne_bytes()));
        }
        return _img;
    }

    pub fn random(&mut self, _width: Option<i32>, _height: Option<i32>) -> DynamicImage {
        let width = _width.unwrap_or(500) as u32;
        let height = _height.unwrap_or(500) as u32;

        let img = Engine::random_gen(width, height, num_cpus::get());

        //img
        return DynamicImage::ImageRgba8(img);
    }

    fn random_gen(width: u32, height: u32, num_threads: usize) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let img = Arc::new(Mutex::new(ImageBuffer::new(width, height)));
        let mut handles = vec![];
        let rows_per_thread = height as usize / num_threads;

        for i in 0..num_threads {
            let img = Arc::clone(&img);
            let start_row = i * rows_per_thread;
            let end_row = if i == num_threads - 1 {
                height as usize
            } else {
                (i + 1) * rows_per_thread
            };

            let handle = std::thread::spawn(move || {
                let mut rng = rand::thread_rng();
                for y in start_row..end_row {
                    for x in 0..width {
                        let r = rng.gen_range(0..=255);
                        let g = rng.gen_range(0..=255);
                        let b = rng.gen_range(0..=255);
                        let a = rng.gen_range(0..=255); // Fully opaque
                        let pixel = Rgba([r, g, b, a]);
                        img.lock()
                            .unwrap()
                            .put_pixel(x, y.try_into().unwrap(), pixel);
                    }
                }
            });

            handles.push(handle);
        }

        for handle in handles {
            handle.join().expect("Thread panicked");
        }

        Arc::try_unwrap(img)
            .expect("Lock still has multiple owners")
            .into_inner()
            .unwrap()
    }
}
