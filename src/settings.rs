use crate::camera::Camera;
use crate::vec3::{Point3, Vec3};
use crate::common;

use exr::prelude::*;
use exr::image::pixel_vec::*; // import predefined pixel storage

type DynamicRgbaPixel = (Sample, Sample, Sample, Sample); // `Sample` is an enum containing the original data type (f16,f32, or u32)
pub type PixelImg = PixelImage<PixelVec<DynamicRgbaPixel>, RgbaChannels> ;

pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub parallel: bool,

    pub sample_per_pixel: u32,
    pub max_depth: u32,

    pub camera: Camera,

    pub env_map: Option<common::SP<PixelImg>>,
}

impl Settings {
    pub fn dump(&self) {
        println!("= Settings");
        println!("=== Execution {}", if self.parallel {"parallel"} else {"sequential"});
        println!("=== vfov {} degrees", self.camera.vfov);
        println!("========================================================");
    }
}

fn load_exr(path: &str) -> PixelImg {

    // load an rgba image
    // this specific example discards all but the first valid rgb layers and converts all pixels to f32 values
    // TODO optional alpha channel!
    let image: PixelImg = read_first_rgba_layer_from_file(
        path,
        PixelVec::<DynamicRgbaPixel>::constructor,

        // use this predefined rgba pixel container from the exr crate, requesting any type of pixels with 3 or 4 values
        PixelVec::set_pixel
    ).expect("Cannot find file ? ");

    //println!("w {:?} ", );
    println!("pixel {:?} ", image.layer_data.channel_data.pixels.get_pixel(Vec2(1, 0)));
    return image;
}

impl Default for Settings {
    fn default() -> Self {

        let lookfrom = Point3::new(-2.0, 2.0, 1.0);
        let lookat = Point3::new(0.0, 0.0, -1.0);

        let dist_to_focus = (lookfrom - lookat).length();
        let aperture = 2.0;

        let camera = Camera::new(
            lookfrom,
            lookat,
            Vec3::new(0.0, 1.0, 0.0),
            90.0, 
            aperture,
            dist_to_focus);
        let image_width = 400;
        let image_height = ((image_width as f64) / camera.aspect_ratio) as u32;
        let env_map_filename = "ressources\\hdri\\symmetrical_garden_02_4k.exr";
        //let env_map_filename = "./ressources/hdri/kloofendal_48d_partly_cloudy_puresky_4k.exr";
        //let env_map_filename = "./ressources/hdri/studio_small_03_4k.exr";
        
        Self {
            image_width: image_width,
            image_height: image_height,
            parallel: true,
            camera: camera,
            sample_per_pixel: 16,
            max_depth: 10,
            //env_map: common::SP::new(load_exr(&env_map_filename)),
            env_map: None,
        }
    }
}