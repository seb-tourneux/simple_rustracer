use crate::common;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::*;

pub struct Camera
{
    pub vfov: Scalar,
    pub aspect_ratio: Scalar,
    // pub viewport_height: Scalar,
    // pub viewport_width: Scalar,

    // pub focal_length: Scalar,

    pub origin: Point3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub lower_left_corner: Point3,
}

impl Camera {
    pub fn new(
        look_from: Vec3,
        look_at: Vec3,
        vup: Vec3,
        vfov: Scalar, // vertical fov in degrees
    ) -> Camera {
        let theta = common::degrees_to_radians(vfov);
        let h = Scalar::tan(theta / 2.0);
        let aspect_ratio = 16.0 / 9.0;
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = vec3::unit_vector(look_from - look_at);
        let u = vec3::unit_vector(vec3::cross(vup, w));
        let v = vec3::unit_vector(vec3::cross(w, u));

        let origin = look_from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - w;



        Camera {
            vfov,
            aspect_ratio,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
        }
    }

    pub fn get_ray(&self, u: Scalar, v: Scalar) -> Ray {
        let dir = self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin;
        Ray::new(self.origin, dir)
    }
}