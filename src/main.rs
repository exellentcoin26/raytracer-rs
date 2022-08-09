#![allow(dead_code)]
use crate::lib::{color::Color, ray::Ray, vec3::Point3, vec3::Vec3};

mod lib;

fn main() {
    // ===================
    //        Image
    // ===================

    let aspect_ratio = 16f64 / 9f64;
    let image_width = 256;
    let image_height = (image_width as f64 / aspect_ratio).round() as usize;

    // ===================
    //       Camera
    // ===================

    let viewport_height = 2f64;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1f64;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

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
        eprintln!("Scanlines remaining: {}\x1B[1A", j);

        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;

            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let color = ray_color(&r);
            println!("{}", color.write_color());
        }
    }

    eprintln!();
    eprintln!("Done");
}

fn ray_color(r: &Ray) -> Color {
    // spawn a single sphere for testing
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_dir = r.direction().unit_vector();
    let t = (1.0f64 + unit_dir.y()) * 0.5;
    (1f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc: Vec3 = r.origin() - *center;
    let a = r.direction().dot(r.direction());
    let b = 2.0 * r.direction().dot(oc);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4.0 * a * c;
    discriminant > 0.0
}
