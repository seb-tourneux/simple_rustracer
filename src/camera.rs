use crate::vec3::*;


pub struct Camera
{
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
/*         vfov: f64, // Vertical field-of-view in degrees
        aspect_ratio: f64, */
    ) -> Camera {
        let aspect_ratio = 16.0 / 9.0;
        let focal_length = 1.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            aspect_ratio: aspect_ratio,
            // viewport_height: viewport_height,
            // viewport_width: viewport_width,
            // focal_length: focal_length,
            origin: origin,
            horizontal: horizontal,
            vertical: vertical,
            lower_left_corner: lower_left_corner,
        }
    }
}