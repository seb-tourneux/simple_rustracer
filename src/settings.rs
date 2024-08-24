use crate::camera::Camera;

pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub parallel: bool,

    pub camera: Camera,
}

impl Settings {
    pub fn dump(&self) {
        println!("= Settings");
        println!("=== Execution {}", if self.parallel {"parallel"} else {"sequential"});
        println!("=== {}x{}", self.image_width, self.image_height);
        println!("========================================================");
    }
}

impl Default for Settings {
    fn default() -> Self {
        let camera = Camera::new();
        let image_width = 400;
        let image_height = ((image_width as f64) / camera.aspect_ratio) as u32;
        Self {
            image_width: image_width,
            image_height: image_height,
            parallel: true,
            camera: camera,
        }
    }
}