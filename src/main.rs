mod settings;
mod vec3;
mod color;
mod ray;
mod camera;
mod hittable;
mod hittable_list;
mod common;
mod material;

mod render;
pub mod sphere;
use crate::render::render;

use std::path::Path;
use std::fs;
use image::{ImageBuffer, RgbImage};

use std::time::Instant;
use chrono::prelude::*;


fn main() -> std::io::Result<()> {

    let chrono_total = Instant::now();

    let settings: settings::Settings = Default::default();

    let mut img: RgbImage = ImageBuffer::new(settings.image_width, settings.image_height);
    settings.dump();

    println!("Rendering...");
    let chrono_render_loop = Instant::now();
    render(&settings, &mut img);
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
