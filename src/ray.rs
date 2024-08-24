use crate::vec3::*;

#[derive(Default)]
pub struct Ray {
    orig: Point3,
    dir: Vec3,
}

impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Ray {
        Ray{
            orig: origin,
            dir: direction,
        }
    }

    pub fn origin(&self) -> Point3 {
        self.orig
    }

    pub fn direction(&self) -> Point3 {
        self.dir
    }

    pub fn at(&self, t:Scalar) -> Point3 {
        self.orig + t * self.dir
    }
}