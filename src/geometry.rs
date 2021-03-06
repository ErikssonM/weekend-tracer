use nalgebra::base::Vector3;
use rand::{
    distributions::uniform::{UniformFloat, UniformSampler},
    random, thread_rng,
};
use std::f64::consts::PI;

use crate::{
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub type V3 = Vector3<f64>;

pub type Point = V3;

pub struct Ray {
    pub orig: Point,
    pub dir: V3,
}

pub struct Sphere {
    pub center: Point,
    pub radius: f64,
    pub material: Box<dyn Material>,
}

pub fn v3(x: f64, y: f64, z: f64) -> V3 {
    V3::new(x, y, z)
}

pub fn rand_vec() -> V3 {
    v3(random(), random(), random())
}

pub fn unit(v: &V3) -> V3 {
    v.normalize()
}

pub fn near_zero(v: &V3) -> bool {
    let eps = 1e-8;
    v.norm() < eps
}

pub fn rand_in(min: f64, max: f64) -> f64 {
    random::<f64>() * (max - min) + min
}

pub fn rand_in_unit_disk() -> V3 {
    v3(rand_in(-1., 1.), rand_in(-1., 1.), 0.).normalize() * random::<f64>()
}

pub fn rand_vec_bounded(min: f64, max: f64) -> V3 {
    let mut rng = thread_rng();
    let range: UniformFloat<f64> = UniformSampler::new(min, max);
    v3(
        range.sample(&mut rng),
        range.sample(&mut rng),
        range.sample(&mut rng),
    )
}

pub fn deg_to_rad(deg: f64) -> f64 {
    (deg * PI / 2.) / 180.
}

pub fn random_in_unit_sphere() -> V3 {
    rand_vec_bounded(-1., 1.).normalize() * random::<f64>()
}

pub fn random_unit_vec() -> V3 {
    rand_vec().normalize()
}

pub fn reflect(v: &V3, n: &V3) -> V3 {
    v - 2.0 * v.dot(n) * n
}

pub fn refract(uv: &V3, n: &V3, etai_over_etat: f64) -> V3 {
    let cos_theta = -uv.dot(n).min(-1.);
    let r_out_perp = etai_over_etat * (uv + cos_theta * n);
    let r_out_par = -(1. - r_out_perp.dot(&r_out_perp)).abs().sqrt() * n;

    r_out_perp + r_out_par
}

impl Ray {
    pub fn at(&self, t: f64) -> V3 {
        self.orig + t * self.dir
    }

    pub fn direction(&self) -> V3 {
        self.dir
    }

    pub fn origin(&self) -> Point {
        self.orig
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().norm_squared();
        let half_b = oc.dot(&ray.direction());
        let c = oc.norm_squared() - self.radius * self.radius;

        let discriminant = half_b.powf(2.) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let mut root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || t_max < root {
            root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let t = root;
        let point = ray.at(root);
        let outward_normal = (point - self.center) / self.radius;

        Some(HitRecord::new(
            &ray,
            &outward_normal,
            point,
            self.material.as_ref(),
            t,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Color, Metal};

    #[test]
    fn test_hit_sphere() {
        let mat = Metal {
            albedo: Color(v3(1., 1., 1.)),
            fuzz: 0.1,
        };

        let sphere = Sphere {
            center: v3(0., 0., 0.),
            radius: 1.,
            material: Arc::new(mat),
        };

        let ray = Ray {
            orig: v3(-2., 0., 0.),
            dir: v3(1., 0., 0.),
        };

        // Test hit
        let res = sphere.hit(&ray, 0., 100.);

        // A Hit should be detected
        match res {
            // recorded hit site should be -1, 0, 0
            Some(rec) => assert_eq!((rec.point - v3(-1., 0., 0.)).norm() < 0.01, true),
            None => panic!("Expected a hit to be recorded"),
        }
    }

    #[test]
    fn test_hit_sphere_no_intersection() {
        let mat = Metal {
            albedo: Color(v3(1., 1., 1.)),
            fuzz: 0.1,
        };

        let sphere = Sphere {
            center: v3(0., 0., 0.),
            radius: 1.,
            material: Arc::new(mat),
        };

        let ray = Ray {
            orig: v3(-2., 0., 0.),
            dir: v3(0., 1., 0.),
        };

        // Test hit
        let res = sphere.hit(&ray, 0., 100.);

        // No hit should be detected
        match res {
            Some(_) => panic!("No hit should be detected"),
            None => (),
        }
    }

    #[test]
    fn test_hit_sphere_ray_starts_inside() {
        let mat = Metal {
            albedo: Color(v3(1., 1., 1.)),
            fuzz: 0.1,
        };

        let sphere = Sphere {
            center: v3(0., 0., 0.),
            radius: 1.,
            material: Arc::new(mat),
        };

        let ray = Ray {
            orig: v3(0., 0., 0.),
            dir: v3(1., 0., 0.),
        };

        // Test hit
        let res = sphere.hit(&ray, 0., 100.);

        // Hit should be detected
        match res {
            // Hit should occur at 1, 0, 0
            Some(rec) => assert_eq!((rec.point - v3(1., 0., 0.)).norm() < 0.01, true),
            None => panic!("Expected a hit to be recorded"),
        };
    }

    #[test]
    fn test_hit_sphere_glancing() {
        let mat = Metal {
            albedo: Color(v3(1., 1., 1.)),
            fuzz: 0.1,
        };

        let sphere = Sphere {
            center: v3(0., 0., 0.),
            radius: 1.,
            material: Arc::new(mat),
        };

        let ray = Ray {
            orig: v3(-2., 1., 0.),
            dir: v3(1., 0., 0.),
        };

        // Test hit
        let res = sphere.hit(&ray, 0., 100.);

        // Hit should be detected
        match res {
            // Hit should occur at 0, 1, 0
            Some(rec) => assert_eq!((rec.point - v3(0., 1., 0.)).norm() < 0.01, true),
            None => panic!("Expected a hit to be recorded"),
        };
    }
}
