use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::common::SP;
use crate::ray::Ray;
use crate::vec3::{self, Point3, Scalar};

pub struct Sphere {
    center: Point3,
    radius: Scalar,
    mat: SP<dyn Material>,
}

impl Sphere {
    pub fn new(center: Point3, radius: Scalar, mat: SP<dyn Material>) -> Sphere {
        Sphere{
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar, rec: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = vec3::dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return false;
        }
 
        let sqrt_d = f64::sqrt(discriminant);
 
        // Find the nearest root that lies in the acceptable range
        let mut root = (-half_b - sqrt_d) / a;
        if root <= t_min || t_max <= root {
            root = (-half_b + sqrt_d) / a;
            if root <= t_min || t_max <= root {
                return false;
            }
        }
 
        rec.t = root;
        rec.p = ray.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());

        true
    }
}