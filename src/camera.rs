use crate::geometry::{deg_to_rad, rand_in_unit_disk, unit, v3, Point, Ray, V3};

pub struct Camera {
    origin: Point,
    lower_left: Point,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    w: V3,
    lens_radius: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Point,
        lookat: Point,
        vup: V3,
        vfov: f64,
        aspect: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Self {
        let theta = deg_to_rad(vfov);
        let h = f64::tan(theta / 2.);

        let vp_height = 2.0 * h;
        let vp_width = aspect * vp_height;

        let w = unit(&(lookfrom - lookat));
        let u = unit(&vup.cross(&w));
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * vp_width * u;
        let vertical = focus_dist * vp_height * v;
        let lower_left = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.;

        Self {
            origin,
            lower_left,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * rand_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            orig: self.origin.clone() + offset,
            dir: self.lower_left + s * self.horizontal + t * self.vertical - self.origin - offset, //dir: self.lower_left + s*self.horizontal + t*self.vertical - self.origin
        }
    }
}
