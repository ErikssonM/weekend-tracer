use crate::{
    geometry::{Point, Ray, V3},
    material::Material,
};

pub struct HittableList {
    pub list: Vec<Box<dyn Hittable>>,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
}
#[derive(Clone)]
pub struct HitRecord<'mat> {
    pub point: Point,
    pub normal: V3,
    pub material: &'mat dyn Material,
    pub t: f64,
    pub front_face: bool,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn add(&mut self, item: Box<dyn Hittable + Send + Sync>) {
        self.list.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut any_hit: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for hittable in self.list.iter() {
            if let Some(rec) = hittable.hit(ray, t_min, closest_so_far) {
                closest_so_far = rec.t;
                any_hit = Some(rec);
            }
        }
        any_hit
    }
}

impl<'mat> HitRecord<'mat> {
    pub fn new(
        ray: &Ray,
        outward_normal: &V3,
        point: Point,
        material: &'mat dyn Material,
        t: f64,
    ) -> Self {
        let (normal, front_face) = Self::get_face_normal_from_ray(ray, outward_normal);
        HitRecord {
            point,
            normal,
            material,
            t,
            front_face,
        }
    }

    fn get_face_normal_from_ray(ray: &Ray, outward_normal: &V3) -> (V3, bool) {
        let front_face = ray.direction().dot(outward_normal) < 0.0;
        let normal = if front_face {
            *outward_normal
        } else {
            -outward_normal
        };
        (normal, front_face)
    }
}
