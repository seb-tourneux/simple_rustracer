use std::iter::Scan;

use crate::color::{self, Color};
use crate::noise::Perlin;
use crate::{checkerboard, common, to_spherical};
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{self, Scalar};


pub trait Material: Send + Sync {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}


pub struct Lambertian {
    albedo: Color,
    checker: Option<Scalar>,

    perlin: Perlin,
}

impl Lambertian {
    pub fn new(albedo: Color, checker: Option<Scalar>) -> Lambertian {
        Lambertian{
            albedo,
            checker,
            perlin: Perlin::new(),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {

        *attenuation = self.albedo;
        if self.checker.is_some() {
            // if checkerboard(rec.uv.x(), self.checker.unwrap()) 
            //     ^ checkerboard(rec.uv.y(), self.checker.unwrap()) 
            // {
            //     *attenuation = 0.5 * self.albedo;
            // }
            *attenuation = self.perlin.cell_noise(rec.p) * self.albedo;
        }

        let mut scatter_direction = rec.normal + vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(rec.p, scatter_direction);

        true
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: Scalar
}

impl Metal {
    pub fn new(albedo: Color, fuzz: Scalar) -> Metal {
        Metal{
            albedo,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = self.albedo;

        let mut reflected_direction = vec3::reflect(vec3::unit_vector(r_in.direction()), rec.normal);
        reflected_direction = reflected_direction + self.fuzz * vec3::random_in_unit_sphere();
        *scattered = Ray::new(rec.p, reflected_direction);

        vec3::dot(scattered.direction(), rec.normal) > 0.0
    }
}

pub struct Dielectric {
    ir: Scalar,
}

impl Dielectric {
    pub fn new(ir: Scalar) -> Dielectric {
        Dielectric{
            ir,
        }
    }

    fn reflectance(cosine:Scalar, ref_idx: Scalar) -> Scalar {
        // Schlick
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * Scalar::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = vec3::unit_vector(r_in.direction());

        let cos_theta = Scalar::min(vec3::dot(-unit_direction, rec.normal), 1.0);
        let sin_theta = Scalar::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction = if cannot_refract
            || Self::reflectance(cos_theta, refraction_ratio) > common::random_double()
        {
            vec3::reflect(unit_direction, rec.normal)
        }
        else {
            vec3::refract(unit_direction, rec.normal, refraction_ratio)
        };

        *attenuation = color::white();
        *scattered = Ray::new(rec.p, direction);

        true
    }
}
