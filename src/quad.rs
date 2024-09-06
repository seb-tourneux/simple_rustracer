use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::common::SP;
use crate::ray::Ray;
use crate::{unit_vector, dot, cross};
use crate::vec3::{self, Vec3, Point3, Scalar};

pub struct Quad {
    origin: Point3,
    u: Vec3,
    v: Vec3,
    normal: Vec3,
    w: Vec3,
    d: Scalar,
    mat: SP<dyn Material>,
}

impl Quad {
    pub fn new(origin: Point3, u: Vec3, v: Vec3, mat: SP<dyn Material>) -> Quad {
        let n = cross(u, v);
        let normal = unit_vector(n);
        let d = vec3::dot(normal, origin);
        let w = n / n.length_squared();
        Quad{
            origin,
            u,
            v,
            normal,
            w,
            d,
            mat,
        }
    }
}

impl Hittable for Quad {
    fn hit(&self, ray: &Ray, t_min: Scalar, t_max: Scalar, rec: &mut HitRecord) -> bool {

        const EPSILON : Scalar = 1e-8;
        let dot_nd = dot(self.normal, ray.direction());
        if dot_nd.abs() < EPSILON {
            return false;
        }
        let dot_no = dot(self.normal, ray.origin());

        let t = (self.d - dot_no) / dot_nd;

        if t <= t_min || t_max <= t {
            return false;
        }

        rec.t = t;
        rec.p = ray.at(t);
        let o_p: Vec3 = rec.p - self.origin;

        // inside quad ?
        let alpha = dot(self.w, cross(o_p, self.v));
        if alpha < 0.0 || alpha > 1.0 {
            return false;
        }
        let beta = dot(self.w, cross(self.u, o_p));
        if beta < 0.0 || beta > 1.0 {
            return false;
        }

        let outward_normal = if dot_nd < 0.0 { -self.normal} else { self.normal };
        rec.set_face_normal(ray, outward_normal);
        rec.mat = Some(self.mat.clone());

        rec.uv = Vec3::new(alpha, beta, 0.0);

        true
    }
}