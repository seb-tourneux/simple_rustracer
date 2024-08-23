pub struct Settings {
    pub image_width: u32,
    pub image_height: u32,
    pub parallel: bool,

}

impl Settings
{
    pub fn dump(&self) {
        println!("= Settings");
        println!("=== Execution {}", if self.parallel {"parallel"} else {"sequential"});
        println!("=== {}x{}", self.image_width, self.image_height);
        println!("========================================================");
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            image_width: 256,
            image_height: 256,
            parallel: true,
        }
    }
}