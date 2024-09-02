mod vec3;
mod settings;
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
use camera::Camera;
use common::degrees_to_radians;
use image::{ImageBuffer, RgbImage};
use settings::Settings;
use vec3::*;
use common::*;

use std::time::Instant;
use chrono::prelude::*;

fn launch_render(settings: &Settings) -> std::io::Result<()> {

    let chrono_total = Instant::now();

    let mut img: RgbImage = ImageBuffer::new(settings.image_width, settings.image_height);
    settings.dump();

    println!("Rendering...");
    let chrono_render_loop = Instant::now();
    render(&settings, &mut img);
    println!("== Elapsed render {:?}", chrono_render_loop.elapsed());

    
    let chrono_save = Instant::now();
    let output_filename_last="output/render_last.png";
    img.save(Path::new(output_filename_last)).unwrap();
    println!("== Elapsed save {:?}", chrono_save.elapsed());

    let local: DateTime<Local> = Local::now();
    let output_filename=format!("output/render_{}.png", local.format("%Y-%m-%d_%H_%M_%S_%3f").to_string());
    println!("Saving {}...", output_filename);
    fs::copy(output_filename_last, output_filename)?;

    println!("= Elapsed render+save {:?}", chrono_total.elapsed());

    Ok(())
}
fn main() -> std::io::Result<()> {

    let single = true;
    if single {
        let mut settings: settings::Settings = Default::default();

        let fov = 50.0;
        let dist = 5.0;
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let t = 2.0 * PI *(1.0 / 8.0);
        let look_from = look_at + 
            Point3::new(dist * Scalar::cos(t), 
                        0.8, 
                        dist * Scalar::sin(t) );

        settings.camera = Camera::new(
            look_from,
            look_at,
            Vec3::new(0.0, 1.0, 0.0),
            fov, );
            
        launch_render(&settings)?;   
    }
    else
    {
        let nb_frames = 16;
        for i in 0..nb_frames {
            let mut settings: settings::Settings = Default::default();

            let anim_tournette = false;
            if anim_tournette {
                let fov = 30.0;
                let dist = 5.0;
                let look_at = Point3::new(0.0, 0.0, -1.0);
                let t = 2.0 * PI *(i as Scalar) / (nb_frames as Scalar);
                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );

                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, );
                launch_render(&settings)?;
            }
            else {
                // anim fov
                let fov = linear_step((i as Scalar) / ((nb_frames-1) as Scalar), 10.0, 90.0);
                let dist = 5.0;
                let t = 2.0 * PI * (1.0 / 8.0);

                let look_at = Point3::new(0.0, 0.0, -1.0);

                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );

                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, );
                launch_render(&settings)?;
            }
        }

    }


    
    Ok(())
}
