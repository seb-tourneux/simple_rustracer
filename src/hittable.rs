use crate::ray::Ray;
use crate::vec3::*;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: Scalar,
    pub front_face: bool,
}

impl HitRecord {
    pub fn new() -> HitRecord {
        Default::default()
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = dot(r.direction(), outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar, rec: &mut HitRecord) -> bool;
}