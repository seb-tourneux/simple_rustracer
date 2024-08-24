
use crate::{ray::*, vec3};
use crate::camera::Camera;

use crate::settings::*;

use crate::color::{Color, WriteColor};
use crate::vec3::Point3;
use crate::vec3::Scalar;
use rayon::prelude::*;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;



fn sky_color(ray: &Ray) -> Color {
    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    const COLOR1: Color = Color::new(1.0, 1.0, 1.0);
    const COLOR2: Color = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * COLOR1 + t * COLOR2
}

fn ray_color(ray: &Ray) -> Color {
    sky_color(ray)
}

fn compute_color(cam: &Camera, u: Scalar, v: Scalar) -> Color {
    let dir = cam.lower_left_corner + u * cam.horizontal + v * cam.vertical - cam.origin;
    let r = Ray::new(cam.origin, dir);

    ray_color(&r)
}

pub fn render(settings: &Settings, img: &mut RgbImage)
{
    let total_nb_pixels = settings.image_width * settings.image_height; 
    let progress_bar = ProgressBar::new(total_nb_pixels.into());
    progress_bar.set_message("Render");
    progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));


    if !settings.parallel {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let u = (x as f64) / ((settings.image_width-1) as f64);
            let v = (y as f64) / ((settings.image_height-1) as f64);

            let pixel_color = compute_color(&settings.camera, u, v);
            pixel.write_color(pixel_color);
            progress_bar.inc(1);
        }
    }
    else {
        img.par_chunks_mut(3).enumerate().for_each(|(i, pixel)| {
            let x = (i % settings.image_width as usize) as u32;
            let y = settings.image_height - ((i / settings.image_width as usize) as u32);
            
            let u = (x as f64) / ((settings.image_width-1) as f64);
            let v = (y as f64) / ((settings.image_height-1) as f64);

            let pixel_color = compute_color(&settings.camera, u, v);

            pixel.write_color(pixel_color);
            progress_bar.inc(1);
        });
    }

    progress_bar.finish();

}