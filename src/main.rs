mod settings;

use std::path::Path;
use image::{ImageBuffer, RgbImage, Rgb};

use std::time::Instant;
use rayon::prelude::*;

fn to_byte(f: f64) -> u8
{
    return (255.0 * f) as u8;
}
fn main() {
    let chrono_total = Instant::now();

    let settings: settings::Settings = Default::default();

    let output_filename="output/render_last.png";
    let mut img: RgbImage = ImageBuffer::new(settings.image_width, settings.image_height);
    //let mut file = File::create(output_filename)?;

    settings.dump();

    let chrono_render_loop = Instant::now();
    if !settings.parallel {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (x as f64) / ((settings.image_width-1) as f64);
            let g = (y as f64) / ((settings.image_height-1) as f64);
            let b: f64 = 0.0;
            *pixel = Rgb([to_byte(r), to_byte(g), to_byte(b)]);
        }
    }
    else {
        img.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            let x = (i % settings.image_width as usize) as u8;
            let y = (i / settings.image_width as usize) as u8;
            
            let r = (x as f64) / ((settings.image_width-1) as f64);
            let g = (y as f64) / ((settings.image_height-1) as f64);
            let b: f64 = 0.0;

            pixel[0] = to_byte(r);
            pixel[1] = to_byte(g);
            pixel[2] = to_byte(b);
        });
    }
    println!("== Elapsed render {:?}", chrono_render_loop.elapsed());

    let chrono_save = Instant::now();
    img.save(Path::new(output_filename)).unwrap();
    println!("== Elapsed save {:?}", chrono_save.elapsed());

    println!("= Elapsed total {:?}", chrono_total.elapsed());
}
