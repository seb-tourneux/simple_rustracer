use crate::camera::Camera;
use crate::vec3::{Point3, Vec3};

pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub parallel: bool,

    pub sample_per_pixel: u32,
    pub max_depth: u32,

    pub camera: Camera,
}

impl Settings {
    pub fn dump(&self) {
        println!("= Settings");
        println!("=== Execution {}", if self.parallel {"parallel"} else {"sequential"});
        println!("=== vfov {} degrees", self.camera.vfov);
        println!("========================================================");
    }
}

impl Default for Settings {
    fn default() -> Self {
        let camera = Camera::new(
            Point3::new(-2.0, 2.0, 1.0),
            Point3::new(0.0, 0.0, -1.0),
            Vec3::new(0.0, 1.0, 0.0),
            90.0, );
        let image_width = 400;
        let image_height = ((image_width as f64) / camera.aspect_ratio) as u32;
        Self {
            image_width: image_width,
            image_height: image_height,
            parallel: true,
            camera: camera,
            sample_per_pixel: 2,
            max_depth: 10,
        }
    }
}