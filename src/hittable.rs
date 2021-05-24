use std::rc::Rc;

use crate::{
    geometry::{v3, Point, Ray, V3},
    material::Material,
};

#[derive(Clone)]
pub struct HittableList {
    pub list: Vec<Rc<dyn Hittable>>,
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool;
}
#[derive(Clone)]
pub struct HitRecord {
    pub point: Point,
    pub normal: V3,
    pub material: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList { list: Vec::new() }
    }

    pub fn clear(&mut self) {
        self.list = Vec::new()
    }

    pub fn add(&mut self, item: Rc<dyn Hittable>) {
        self.list.push(item);
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut any_hit = false;
        let mut closest_so_far = t_max;

        for hittable in self.list.iter() {
            let mut tmp_record = HitRecord::new();
            if hittable.hit(ray, t_min, closest_so_far, &mut tmp_record) {
                any_hit = true;
                closest_so_far = tmp_record.t;
                *record = tmp_record;
            }
        }

        any_hit
    }
}

impl HitRecord {
    pub fn new() -> Self {
        HitRecord {
            point: v3(0.0, 0.0, 0.0),
            normal: v3(0.0, 0.0, 0.0),
            material: None,
            t: 0.0,
            front_face: true,
        }
    }

    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &V3) {
        self.front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -outward_normal
        }
    }
}
