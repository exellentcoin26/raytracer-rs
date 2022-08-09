#![allow(dead_code)]
use crate::lib::{color::Color, ray::Ray, vec3::Vec3};

mod lib;

fn main() {
    // ===================
    //        Image
    // ===================

    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    let image_width: usize = 256;
    let image_height: usize = (image_width as f64 / ASPECT_RATIO).round() as usize;

    // ===================
    //       Camera
    // ===================

    let viewport_height = 2f64;
    let viewport_width = ASPECT_RATIO * viewport_height;

    // print file to console (https://en.wikipedia.org/wiki/Netpbm#PPM_example)
    // file header
    println!("P3\n{} {}\n255", image_width, image_height);

    // example image
    for j in (0..image_height).rev() {
        // print progress to `stderr`
        // ascii escape sequence that rerenders said line
        eprintln!("Scanlines remaining: {}\x1B[1A", j);

        for i in 0..image_width {
            let color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25,
            );

            println!("{}", color.write_color());
        }
    }

    eprintln!();
    eprintln!("Done");
}

fn ray_color(r: &Ray) -> Color {
    let unit_dir = r.direction().unit_vector();
    let t = (1.0f64 + unit_dir.y()) * 0.5;
    (1f64 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
