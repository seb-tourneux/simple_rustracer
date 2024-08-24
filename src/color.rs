
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
        *self = Rgb([to_byte(pixel_color.x()), to_byte(pixel_color.y()), to_byte(pixel_color.z())]);
    }
}

impl WriteColor for &mut [u8] {
    fn write_color(self, pixel_color: Color) {
        self[0] = to_byte(pixel_color.x());
        self[1] = to_byte(pixel_color.y());
        self[2] = to_byte(pixel_color.z());
    }
}
