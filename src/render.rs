use crate::quad::Quad;
use crate::{material::*, to_spherical};
use crate::{color, ray::*, vec3};
use crate::camera::Camera;

use crate::settings::*;
use crate::hittable::*;
use crate::hittable_list::*;
use crate::common::{self, random_double, SP, sigmoid};

use crate::color::{Color, WriteColor};
use crate::vec3::{Point3, Vec3};
use crate::vec3::Scalar;
use crate::vec3::*;
use crate::sphere::Sphere;

use rayon::prelude::*;
use image::RgbImage;
use indicatif::{ProgressBar, ProgressStyle, ProgressState};
use std::fmt::Write;

use exr::prelude::Vec2;

fn get_image_pixel(settings: &Settings, spherical: Vec3) -> Color {
    let image = settings.env_map.as_ref().unwrap();
    let size = image.layer_data.size;
    let pos_px = Vec2(  (spherical.y() * (size.0 as Scalar)) as usize,
                       (spherical.z() * (size.1 as Scalar)) as usize);
    let pixel = image.layer_data.channel_data.pixels.get_pixel(pos_px);
    Color::new(pixel.0.to_f32().into(), pixel.1.to_f32().into(), pixel.2.to_f32().into())
}


fn sky_color_hdri(ray: &Ray, settings: &Settings) -> Color {
    let mut unit_direction = vec3::unit_vector(ray.direction());
    unit_direction.e[1] = unit_direction.y();
    let mut sph = to_spherical(unit_direction);
    
    sph.e[1] %= 2.0 * common::PI;

    sph.e[2] += 2.0*common::PI;
    sph.e[2] %= 2.0 * common::PI;

    get_image_pixel(settings, sph  / (2.0 * common::PI) )
}

fn sky_color(ray: &Ray, settings: &Settings) -> Color {
    let unit_direction = vec3::unit_vector(ray.direction());
    let mut t = 0.5 * (unit_direction.y() + 1.0);
    const COLOR1: Color = color::white();
    const COLOR2: Color = Color::new(0.5, 0.7, 1.0);
    //t = t * t * t;
    //t = Scalar::powf(t, 0.5);
    t*=1.5;
    //t = sigmoid(t+0.5, 2.0);
    
    (1.0 - t) * COLOR1 + t * COLOR2
}

fn ray_color(ray: &Ray, world: &dyn Hittable, settings: &Settings, depth: u32) -> Color {

    if depth <= 0 {
        return color::black();
    }

    let mut rec = HitRecord::new();
    const EPSILON: Scalar = 0.0001;

    if world.hit(ray, EPSILON, common::INFINITY, &mut rec) {
        let mut attenuation = Color::default();
        let mut scattered = Ray::default();

        //return rec.uv; // todo : pattern matching to switch pass ?
        //return vec3::fit01(vec3::unit_vector(rec.p)); // todo : pattern matching to switch pass ?
        //return vec3::fit01(vec3::unit_vector(rec.normal)); // todo : pattern matching to switch pass ?
        if rec.mat.as_ref().unwrap().
            scatter(ray, &rec, &mut attenuation, &mut scattered)
        {
            return attenuation * ray_color(&scattered, world, settings, depth - 1);
        }   
        return color::black();   
    }
    sky_color(ray, settings)
}

fn compute_color(settings: &Settings, u: Scalar, v: Scalar, world: &dyn Hittable) -> Color {
    let r = settings.camera.get_ray(u, v);
    ray_color(&r, world, settings, settings.max_depth)

}

fn generate_world_generic() -> HittableList
{
    let mut world = HittableList::new();

    let lambert_blue = SP::new(Lambertian::new(Color::new(0.1, 0.2, 0.8), None));
    let lambert_red = SP::new(Lambertian::new(Color::new(0.8, 0.3, 0.2), Some(0.005)));
    let lambert_red_checker = SP::new(Lambertian::new(Color::new(0.8, 0.3, 0.2), Some(0.005)));
    let lambert_dark = SP::new(Lambertian::new(Color::new(0.06, 0.05, 0.05), Some(0.1)));
    let metal_red = SP::new(Metal::new(Color::new(0.8, 0.5, 0.3), 0.9));
    let metal_green = SP::new(Metal::new(Color::new(0.6, 0.8, 0.65), 0.5));
    let metal_white_fuzz = SP::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.));
    let metal_white_reflect = SP::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.001));
    let glass = SP::new(Dielectric::new(1.5));
    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, lambert_blue.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-1.0, 1.5, -3.0), 0.6, lambert_blue)));
    world.add(Box::new(Sphere::new(Point3::new(4.5, 1.7, -4.0), 1.0, metal_red.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-1.2, 0.05, -1.0), 0.5, glass.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.35, -0.4, -0.7), 0.1, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.55, -0.4, -0.8), 0.1, metal_green.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-0.55, -0.4, -0.75), 0.1, metal_green.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-0.40, -0.47, -0.7), 0.03, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-0.38, -0.47, -0.75), 0.03, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-0.33, -0.47, -0.72), 0.03, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.5, 0.8, -1.3), 0.35, metal_green)));
    world.add(Box::new(Sphere::new(Point3::new(-0.85, 0.5, -1.2), -0.25, glass)));
    world.add(Box::new(Sphere::new(Point3::new(1.55, 1.2, -1.9), 0.4, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(1.55, 1.2, -1.9), 0.4, metal_white_reflect)));
    world.add(Box::new(Sphere::new(Point3::new(0.95, 0.4, -0.15), 0.15, metal_white_fuzz.clone())));
    world.add(Box::new(Sphere::new(Point3::new(1.15, 0.2, -0.12), 0.08, metal_white_fuzz.clone())));
    world.add(Box::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, lambert_red_checker.clone())));
    //world.add(Box::new(Sphere::new(Point3::new(-15.0, 27.0, -5.0), 25.0, lambert_dark)));


    return world;
}

fn generate_world_planes() -> HittableList
{
    
    let mut world = HittableList::new();

    let lambert_blue = SP::new(Lambertian::new(Color::new(0.1, 0.2, 0.8), None));
    let lambert_green = SP::new(Lambertian::new(Color::new(0.5, 0.8, 0.6), None));
    let lambert_red_plane = SP::new(Lambertian::new(Color::new(0.8, 0.3, 0.2), Some(0.2)));
    let lambert_red_checker = SP::new(Lambertian::new(Color::new(0.8, 0.3, 0.2), Some(0.5)));
    let lambert_dark = SP::new(Lambertian::new(Color::new(0.06, 0.05, 0.05), Some(0.1)));
    let lambert_light = SP::new(Lambertian::new(Color::new(0.95, 0.95, 0.95), Some(0.1)));
    let metal_red = SP::new(Metal::new(Color::new(0.8, 0.5, 0.3), 0.9));
    let metal_green = SP::new(Metal::new(Color::new(0.6, 0.8, 0.65), 0.5));
    let metal_white_little_fuzz = SP::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.05));
    let metal_white_reflect = SP::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.001));
    let glass = SP::new(Dielectric::new(1.5));
    let bubble = SP::new(Dielectric::new(-1.5));

    world.add(Box::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, metal_white_reflect.clone())));
    world.add(Box::new(Sphere::new(Point3::new(-1.2, 0.05, -1.0), 0.5, lambert_blue.clone())));
    world.add(Box::new(Sphere::new(Point3::new(1.0, -0.4, -1.0), 0.1, lambert_green.clone())));
    let plane_scale = 3.5;
    world.add(Box::new(Quad::new(Point3::new(-0.5*plane_scale,-0.5, -0.5*plane_scale -1.0),
    Vec3::new(0.0, 0.0, plane_scale),
                                                Vec3::new(plane_scale, 0.0, 0.0),
                                                lambert_red_plane.clone())));
    
    world.add(Box::new(Quad::new(Point3::new(2.0, -1.0, -1.5),
                                                Vec3::new(0.0,  3.0, 0.0),
                                                Vec3::new(-2.0, 0.0, -2.0),
                                                lambert_light.clone())));

    let angle = common::degrees_to_radians(45.0);
    world.add(Box::new(Quad::new(Point3::new(0.6, 0.0, -1.0),
                                    0.5*Vec3::new(1.0,  0.0, 0.0),
                                    0.5*Vec3::new(0.0, Scalar::cos(angle), Scalar::sin(angle)),
                                    metal_white_reflect.clone())));

                                    let angle3 = common::degrees_to_radians(-45.0);
    world.add(Box::new(Quad::new(Point3::new(0.5, 0.75, -0.5),
                                    0.5*Vec3::new(0.0,  0.0, 1.0),
                                    0.5*Vec3::new(Scalar::cos(angle3), Scalar::sin(angle3), 0.0),
                                    metal_white_reflect.clone())));
                                          
    return world;
}

fn generate_world_debug() -> HittableList
{
    let mut world = HittableList::new();

    let r = Scalar::cos(common::PI / 4.0);

    let material_left = SP::new(Lambertian::new(Color::new(0.0, 0.0, 1.0), None));
    let material_right = SP::new(Lambertian::new(Color::new(1.0, 0.0, 0.0), None));

    world.add(Box::new(Sphere::new(
        Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    return world;
}

fn render_sequential(   settings: &Settings, 
                        world: &HittableList, 
                        progress_bar: &ProgressBar,
                        img: &mut RgbImage)
{
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let mut pixel_color = color::black();
        for _ in 0..settings.sample_per_pixel {
            let u = ((x as Scalar) + random_double()) / ((settings.image_width-1) as f64);
            let v = (((settings.image_height-y) as Scalar) + random_double()) / ((settings.image_height-1) as f64);

            pixel_color += compute_color(&settings, u, v, world);
        }

        pixel.write_color(pixel_color / (settings.sample_per_pixel as Scalar));
        progress_bar.inc(1);
    }
}

fn render_parallel( settings: &Settings, 
                    world: &HittableList, 
                    progress_bar: &ProgressBar,
                    img: &mut RgbImage)
{
    for j in (0..settings.image_height).rev() {
        let pixel_colors: Vec<_> = (0..settings.image_width)
        .into_par_iter()
        .map(|i| {
            let mut pixel_color = color::black();
            for _ in 0..settings.sample_per_pixel {
                let u = ((i as Scalar) + random_double()) / ((settings.image_width-1) as f64);
                let v = (((settings.image_height-j) as Scalar) + random_double()) / ((settings.image_height-1) as f64);
    
                pixel_color += compute_color(&settings, u, v, world);
            }
            pixel_color / (settings.sample_per_pixel as Scalar)
        })
        .collect();

        for (i, pixel_color) in pixel_colors.iter().enumerate() {
            img.put_pixel(i.try_into().unwrap(), j, color::to_rgb(*pixel_color));
        }
        progress_bar.inc(settings.image_width.into());

    }
}

pub fn render(settings: &Settings, img: &mut RgbImage)
{
    let total_nb_pixels = settings.image_width * settings.image_height; 
    let progress_bar = ProgressBar::new(total_nb_pixels.into());
    progress_bar.set_message("Render");
    progress_bar.set_style(ProgressStyle::with_template("{spinner:.green} [{elapsed_precise}] [{bar:.cyan/blue}] {percent}% ({eta})")
        .unwrap()
        .with_key("eta", |state: &ProgressState, w: &mut dyn Write| write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()));


    //let world = generate_world_generic();
    //let world = generate_world_generic();
    let world = generate_world_planes();

    if !settings.parallel {
        render_sequential(settings, &world, &progress_bar, img);
    }
    else {
        render_parallel(settings, &world, &progress_bar, img);
    }

    progress_bar.finish();

}