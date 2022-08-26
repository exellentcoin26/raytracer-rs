#![allow(dead_code)]

use crate::lib::{
    hittables::HittableList,
    hittables::Sphere,
    materials::{Lambertian, Metal},
    traits::Hittable,
    utils, Camera, Color, Point3, Ray, Vec3,
};
use std::rc::Rc;

mod lib;

fn main() {
    // ===================
    //        Image
    // ===================

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400_usize;
    let image_height = (image_width as f64 / aspect_ratio).round() as usize;
    let samples_per_pixel = 100_usize;
    let max_ray_depth = 50_usize;

    // ===================
    //       World
    // ===================

    let mut world = HittableList::default();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.1, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.2, 0.1, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.2, 0.1, -1.0),
        0.5,
        material_right.clone(),
    )));

    // ===================
    //       Camera
    // ===================

    let cam = Camera::default();

    // ===================
    //       Render
    // ===================

    // print file to console (https://en.wikipedia.org/wiki/Netpbm#PPM_example)
    // file header
    println!("P3\n{} {}\n255", image_width, image_height);

    // example image
    for j in (0..image_height).rev() {
        // print progress to `stderr`
        // ascii escape sequence that rerenders said line
        eprintln!("                                                 \x1B[1A"); // temporary fix for trailing
        eprintln!("Scanline: {} / {}\x1B[1A", image_height - j, image_height);

        for i in 0..image_width {
            // use `Vec3` because the `Color` invariants cannot be guaranteed before rescaling
            let mut color = Vec3::default();

            for _ in 0..samples_per_pixel {
                let u = (i as f64 + utils::random_double()) / (image_width - 1) as f64;
                let v = (j as f64 + utils::random_double()) / (image_height - 1) as f64;

                let r = cam.get_ray(u, v);
                color += ray_color(&r, &world, max_ray_depth).into(); // because `color` is a `Vec3` for now
            }

            let color: Color = (color / samples_per_pixel as f64)
                .try_into()
                .expect("could not convert `Vec3` to `Color`");
            println!("{}", color.write_color());
        }
    }

    eprintln!("\nDone");
}

fn ray_color<T: Hittable>(r: &Ray, world: &T, depth: usize) -> Color {
    // check if depth limit is reached
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // fix the `shadow acne` problem by ignoring bounces that bounce from themselves
    if let Some(rec) = world.hit(r, 0.001, f64::INFINITY) {
        if let Some((attenuation, scattered)) = rec.mat().scatter(r, &rec) {
            return attenuation * ray_color(&scattered, world, depth - 1);
        }
        return Color::default();
    }

    let unit_dir = r.direction().unit_vector();
    let t = 0.5 * (unit_dir.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc: Vec3 = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let half_b = r.direction().dot(oc);
    let c = oc.length_squared() - radius.powi(2);
    let discriminant = half_b.powi(2) - a * c;

    if discriminant < 0.0 {
        None
    } else {
        Some((-half_b - discriminant.sqrt()) / a)
    }
}
