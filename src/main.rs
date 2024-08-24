mod settings;
mod vec3;
mod color;
use color::{Color, WriteColor};

use std::path::Path;
use std::fs;
use image::{ImageBuffer, RgbImage};

use std::time::Instant;
use rayon::prelude::*;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;
use chrono::prelude::*;

fn main() -> std::io::Result<()> {
    let chrono_total = Instant::now();

    let settings: settings::Settings = Default::default();

    let mut img: RgbImage = ImageBuffer::new(settings.image_width, settings.image_height);
    settings.dump();

    let total_nb_pixels = settings.image_width * settings.image_height; 
    let progress_bar = ProgressBar::new(total_nb_pixels.into());
    progress_bar.set_message("Render");
    progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));

    println!("Rendering...");
    let chrono_render_loop = Instant::now();
    if !settings.parallel {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let r = (x as f64) / ((settings.image_width-1) as f64);
            let g = (y as f64) / ((settings.image_height-1) as f64);
            let b: f64 = 0.0;
            pixel.write_color(Color::new(r, g, b));
            progress_bar.inc(1);
        }
    }
    else {
        img.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            let x = (i % settings.image_width as usize) as u8;
            let y = (i / settings.image_width as usize) as u8;
            
            let r = (x as f64) / ((settings.image_width-1) as f64);
            let g = (y as f64) / ((settings.image_height-1) as f64);
            let b: f64 = 0.0;

            pixel.write_color(Color::new(r, g, b));
            progress_bar.inc(1);
        });
    }
    progress_bar.finish();
    println!("== Elapsed render {:?}", chrono_render_loop.elapsed());

    println!("Saving...");
    let chrono_save = Instant::now();
    let output_filename_last="output/render_last.png";
    img.save(Path::new(output_filename_last)).unwrap();
    println!("== Elapsed save {:?}", chrono_save.elapsed());

    let local: DateTime<Local> = Local::now();
    let output_filename=format!("output/render_{}.png", local.format("%Y-%m-%d_%H_%M_%S").to_string());
    fs::copy(output_filename_last, output_filename)?;

    println!("= Elapsed total {:?}", chrono_total.elapsed());
    
    Ok(())
}
