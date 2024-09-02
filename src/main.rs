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

enum Anim {
    NoAnim,
    Tournette,
    Fov,
    Aperture,
    DistFocus,
}

fn main() -> std::io::Result<()> {

    let anim = Anim::DistFocus;
    let nb_frames = 16;

    match anim {
        Anim::NoAnim => {
            let mut settings: settings::Settings = Default::default();

            let fov = 50.0;
            let dist = 2.0;
            let look_at = Point3::new(0.0, 0.0, -1.0);
            let t = 2.0 * PI *(1.0 / 6.0);
            let look_from = look_at + 
                Point3::new(dist * Scalar::cos(t), 
                            0.8, 
                            dist * Scalar::sin(t) );
            let dist_to_focus = (look_from - look_at).length();
            let aperture = 0.4;
                    
            settings.camera = Camera::new(
                look_from,
                look_at,
                Vec3::new(0.0, 1.0, 0.0),
                fov, 
                aperture,
            dist_to_focus);
                
            launch_render(&settings)?;   
        }
        Anim::Tournette => {
            for i in 0..nb_frames {
                let mut settings: settings::Settings = Default::default();

                let fov = 30.0;
                let dist = 5.0;
                let look_at = Point3::new(0.0, 0.0, -1.0);
                let t = 2.0 * PI *(i as Scalar) / (nb_frames as Scalar);
                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );
                let dist_to_focus = (look_from - look_at).length();
                let aperture = 2.0;

                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, 
                    aperture,
                    dist_to_focus);
                launch_render(&settings)?;
            }
        }
        Anim::Fov => {
            for i in 0..nb_frames {
                let mut settings: settings::Settings = Default::default();

                let fov = linear_step((i as Scalar) / ((nb_frames-1) as Scalar), 10.0, 90.0);
                let dist = 5.0;
                let t = 2.0 * PI * (1.0 / 8.0);

                let look_at = Point3::new(0.0, 0.0, -1.0);

                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );
                let dist_to_focus = (look_from - look_at).length();
                let aperture = 2.0;
                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, 
                    aperture,
                    dist_to_focus);
                launch_render(&settings)?;
            }
        }
        Anim::Aperture => {
            for i in 0..nb_frames {
                let mut settings: settings::Settings = Default::default();

                let fov = 50.0;
                let dist = 2.0;
                let look_at = Point3::new(0.0, 0.0, -1.0);
                let t = 2.0 * PI *(1.0 / 6.0);
                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );
                let dist_to_focus = (look_from - look_at).length();

                let t = (i as Scalar) / ((nb_frames-1) as Scalar);

                let aperture = linear_step(t, 0.0, 1.0);
                        
                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, 
                    aperture,
                dist_to_focus);
                    
                launch_render(&settings)?; 
            }
        }
        Anim::DistFocus => {
            for i in 0..nb_frames {
                let mut settings: settings::Settings = Default::default();

                let fov = 50.0;
                let dist = 2.0;
                let look_at = Point3::new(0.0, 0.0, -1.0);
                let t = 2.0 * PI *(1.0 / 6.0);
                let look_from = look_at + 
                    Point3::new(dist * Scalar::cos(t), 
                                0.8, 
                                dist * Scalar::sin(t) );
                                
                let aperture = 0.35;
                let t = (i as Scalar) / ((nb_frames-1) as Scalar);
                let dist_center = (look_from - look_at).length();
                let dist_to_focus = linear_step(t, 0.8 * dist_center, 1.2*dist_center);

                        
                settings.camera = Camera::new(
                    look_from,
                    look_at,
                    Vec3::new(0.0, 1.0, 0.0),
                    fov, 
                    aperture,
                dist_to_focus);
                    
                launch_render(&settings)?; 
            }
        }
    }
    
    Ok(())
}
