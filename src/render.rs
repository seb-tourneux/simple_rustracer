

use crate::{color, ray::*, vec3};
use crate::camera::Camera;

use crate::settings::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::common::{self, random_double};

use crate::color::{Color, WriteColor};
use crate::vec3::Point3;
use crate::vec3::Scalar;
use crate::sphere::Sphere;

use rayon::prelude::*;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;



fn sky_color(ray: &Ray) -> Color {
    let unit_direction = vec3::unit_vector(ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    const COLOR1: Color = color::white();
    const COLOR2: Color = Color::new(0.5, 0.7, 1.0);

    (1.0 - t) * COLOR1 + t * COLOR2
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, 0.0, common::INFINITY, &mut rec) {
        return vec3::fit01(rec.normal);
    }
    sky_color(ray)
}

fn compute_color(settings: &Settings, u: Scalar, v: Scalar, world: &dyn Hittable) -> Color {
    let r = settings.camera.get_ray(u, v);
    ray_color(&r, world)

}

pub fn render(settings: &Settings, img: &mut RgbImage)
{
    let total_nb_pixels = settings.image_width * settings.image_height; 
    let progress_bar = ProgressBar::new(total_nb_pixels.into());
    progress_bar.set_message("Render");
    progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));

    let mut world = HittableList::new();
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 1.5, -3.0), 0.6)));
    world.add(Box::new(Sphere::new(Point3::new(4.5, 1.7, -4.0), 1.0)));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    //if !settings.parallel {
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            let mut pixel_color = color::black();
            for _ in 0..settings.sample_per_pixel {
                let u = ((x as Scalar) + random_double()) / ((settings.image_width-1) as f64);
                let v = (((settings.image_height-y) as Scalar) + random_double()) / ((settings.image_height-1) as f64);
    
                pixel_color += compute_color(&settings, u, v, &world);
            }

            pixel.write_color(pixel_color / (settings.sample_per_pixel as Scalar));
            progress_bar.inc(1);
        }
    //}
    // else {

    //     for j in (0..settings.image_height).rev() {
    //         let pixel_colors: Vec<_> = (0..settings.image_width)
    //             .into_par_iter()
    //             .map(|i| {
    //                 let u = (i as f64) / ((settings.image_width-1) as f64);
    //                 let v = (j as f64) / ((settings.image_height-1) as f64);

    //                 compute_color(&settings.camera, u, v, &world)
    //             })
    //         .collect();

    //         for (i, &pixel_color) in pixel_colors.iter().enumerate() {
    //             img.put_pixel(i as u32, j as u32, color::to_rgb(pixel_color))
    //         }
    //         progress_bar.inc(settings.image_width as u64);
    //     }
    // }

    progress_bar.finish();

}