use material::Material;
use rand::prelude::*;
use std::time::Instant;
use std::{error::Error, fs::File, rc::Rc};

mod camera;
mod color;
mod geometry;
mod hittable;
mod image;
mod material;

use camera::Camera;
use color::Color;
use geometry::{rand_in, unit, v3, Ray, Sphere};
use hittable::{Hittable, HittableList};
use image::{merge_samples, Image};

use crate::{
    material::{Dielectric, Lambertian, Metal},
};

const INF: f64 = f64::INFINITY;

fn ray_color(ray: &Ray, world: &impl Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::black();
    }

    if let Some(rec) = world.hit(ray, 0.001, INF) {
        let col = match rec.material.scatter(&ray, &rec) {
            None => Color::black(),
            Some((att, sc_ray)) => att * ray_color(&sc_ray, world, depth - 1),
        };
        return col;
    }

    let unit_dir = unit(&ray.direction());
    let t = 0.5 * (unit_dir.y + 1.0);
    Color((1.0 - t) * v3(1.0, 1.0, 1.0) + t * v3(0.5, 0.7, 1.0))
}

fn render(
    camera: &Camera,
    world: &HittableList,
    width: usize,
    height: usize,
    samples: i32,
    max_depth: i32,
) -> Image {
    let mut image = Image::new(width, height);

    for j in 0..height {
        for i in 0..width {
            let mut color = Color::black();

            for _ in 0..samples {
                let u = (i as f64 + random::<f64>()) / (width - 1) as f64;
                let v = (j as f64 + random::<f64>()) / (height - 1) as f64;
                let ray = camera.get_ray(u, v);

                color = color + ray_color(&ray, world, max_depth);
            }

            image.img[(i, j)] = Color(color.0 / (samples as f64));
        }
    }

    image
}

fn make_world() -> HittableList {
    let mut world = HittableList::new();

    let ground_mat = Rc::new(Lambertian {
        albedo: Color(v3(0.5, 0.5, 0.5)),
    });
    let ground = Sphere {
        center: v3(0., -1000., 0.),
        radius: 1000.,
        material: ground_mat,
    };
    world.add(Rc::new(ground));

    for a in -3..3 {
        for b in -3..3 {
            let choose_mat: f64 = random();
            let cent = v3(
                a as f64 + 0.9 * random::<f64>(),
                0.2,
                b as f64 + 0.9 * random::<f64>(),
            );

            if (cent - v3(4., 0.2, 0.)).norm() > 0.9 {
                let mat: Rc<dyn Material> = if choose_mat < 0.8 {
                    Rc::new(Lambertian {
                        albedo: Color::random() * Color::random(),
                    })
                } else if choose_mat < 0.95 {
                    Rc::new(Metal {
                        albedo: Color::random_in(0.5, 1.),
                        fuzz: rand_in(0., 0.3),
                    })
                } else {
                    Rc::new(Dielectric { ir: 1.5 })
                };

                world.add(Rc::new(Sphere {
                    center: cent,
                    radius: 0.2,
                    material: mat,
                }));
            }
        }
    }

    let mat1 = Rc::new(Dielectric { ir: 1.5 });
    world.add(Rc::new(Sphere {
        center: v3(0., 1., 0.),
        radius: 1.,
        material: mat1,
    }));

    let mat2 = Rc::new(Lambertian {
        albedo: Color(v3(0.4, 0.2, 0.1)),
    });
    world.add(Rc::new(Sphere {
        center: v3(-4., 1., 0.),
        radius: 1.,
        material: mat2,
    }));

    let mat3 = Rc::new(Metal {
        albedo: Color(v3(0.7, 0.6, 0.5)),
        fuzz: 0.0,
    });
    world.add(Rc::new(Sphere {
        center: v3(0., 1., 0.),
        radius: 1.,
        material: mat3,
    }));

    world
}

fn main() -> Result<(), Box<dyn Error>> {
    let start = Instant::now();

    let ratio: f64 = 16.0 / 9.0;
    let width: usize = 400;
    let height: usize = (width as f64 / ratio) as usize;

    let sub_samples = 8;
    let super_samples = 8;
    let max_depth = 50;

    let world = Rc::new(make_world());

    let lookfrom = v3(13., 2., 3.);
    let lookat = v3(0., 0., 0.);
    let vup = v3(0., 1., 0.);

    let focus_dist = (lookfrom - lookat).norm();
    //let focus_dist = 10.0;

    // Camera
    let camera = Rc::new(Camera::new(
        lookfrom,
        lookat,
        vup,
        40.,
        16. / 9.,
        0.1,
        focus_dist,
    ));

    //let mut handles = Vec::with_capacity(super_samples);
    let mut images = Vec::with_capacity(super_samples);

    for sup in 0..super_samples {
        println!("Running {} of {} samples.", sup, super_samples);
        images.push(render(
            &camera,
            &world,
            width,
            height,
            sub_samples,
            max_depth,
        ))
    }

    // println!("Starting threads");
    // for _ in 0..super_samples {
    //     handles.push(thread::spawn(move ||
    //         render(&camera.clone(), &world.clone(), width, height, sub_samples, max_depth)
    //     ));
    // }

    // for handle in handles {
    //     images.push(handle.join().unwrap());
    // }

    // println!("Joined all threads");

    let final_image = merge_samples(images);

    let mut file = File::create("out.ppm")?;
    final_image.write_ppm(&mut file)?;

    println!("Wrote file!");

    let end = Instant::now();
    println!("Finished running in {:?}", end.duration_since(start));

    Ok(())
}
