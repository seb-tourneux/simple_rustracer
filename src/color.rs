
use crate::vec3::{Vec3, Scalar};
use image::{Rgb};

// Type alias
pub type Color = Vec3;
pub type PixelType = u8;


fn to_byte(f: Scalar) -> PixelType
{
    return (255.999 * f) as PixelType;
}

pub trait WriteColor {
    fn write_color(self, _: Color);
}

impl WriteColor for &mut Rgb<PixelType> {
    fn write_color(self, pixel_color: Color) {
        *self = to_rgb(pixel_color)
    }
}

impl WriteColor for &mut [u8] {
    fn write_color(self, pixel_color: Color) {
        self[0] = to_byte(pixel_color.x());
        self[1] = to_byte(pixel_color.y());
        self[2] = to_byte(pixel_color.z());
    }
}

pub fn to_rgb(pixel_color: Color) -> Rgb<PixelType> {
    Rgb([to_byte(pixel_color.x()), to_byte(pixel_color.y()), to_byte(pixel_color.z())])
}

pub const fn red() -> Color{
    Color::new(1.0, 0.0, 0.0)
}

pub const fn green() -> Color{
    Color::new(0.0, 1.0, 0.0)
}

pub const fn blue() -> Color{
    Color::new(0.0, 0.0, 1.0)
}

pub const fn white() -> Color{
    Color::new(1.0, 1.0, 1.0)
}

pub const fn black() -> Color{
    Color::new(0.0, 0.0, 0.0)
}

pub const fn yellow() -> Color{
    Color::new(1.0, 1.0, 0.0)
}

pub const fn cyan() -> Color{
    Color::new(0.0, 1.0, 1.0)
}

pub const fn magenta() -> Color{
    Color::new(1.0, 0.0, 1.0)
}