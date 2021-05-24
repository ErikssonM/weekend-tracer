use rand::random;

use crate::{geometry::{near_zero, random_in_unit_sphere, random_unit_vec, reflect, refract, unit, v3}, hittable::HitRecord};
use crate::geometry::Ray;
use crate::color::Color;

pub type Scatter = (Color, Ray);

pub trait Material {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter>;
}

pub struct Lambertian {
    pub albedo: Color
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64
}

pub struct Dielectric {
    pub ir: f64
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let mut scatter_dir = rec.normal + random_unit_vec();

        if near_zero(&scatter_dir) {
            scatter_dir = rec.normal;
        }

        let scattered = Ray { orig: rec.point, dir: scatter_dir };
        let color = self.albedo.clone();
        Some((color, scattered))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let reflected = reflect(&unit(&ray.direction()), &rec.normal);
        let scattered = Ray { orig: rec.point, dir: reflected + self.fuzz * random_in_unit_sphere()};
        let color = self.albedo.clone();

        if scattered.direction().dot(&rec.normal) > 0. {
            Some((color, scattered))
        } else {
            None
        }
    }
}

impl Dielectric {
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powf(2.);
        r0 + (1.-r0) * (1.-cosine).powf(5.)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<Scatter> {
        let attenuation = Color(v3(1.0, 1.0, 1.0));
        let refraction_ratio = if rec.front_face {1.0/self.ir} else {self.ir};

        let unit_dir = unit(&ray.direction());
        
        let cos_theta = unit_dir.dot(&rec.normal).min(1.0);
        let sin_theta = (1. - cos_theta*cos_theta).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let should_reflect = Dielectric::reflectance(cos_theta, refraction_ratio) > random();

        let direction = if cannot_refract || should_reflect {
            reflect(&unit_dir, &rec.normal)
        } else {
            refract(&unit_dir, &rec.normal, refraction_ratio)
        };

        Some((attenuation, Ray { orig: rec.point, dir: direction }))
    }
}