use std::time::Instant;
use rand::Rng;

mod vec3;
mod ray;
mod camera;
mod color;
mod scene;
mod hittable;
mod sphere;
mod material;

use vec3::Vec3;
use color::Color;
use camera::Camera;
use scene::Scene;
use sphere::Sphere;
use material::{Lambertian, Metal};

fn main() {
    // Image settings
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // Create camera
    let lookfrom = Vec3::new(3.0, 3.0, 2.0);
    let lookat = Vec3::new(0.0, 0.0, -1.0);
    let vup = Vec3::new(0.0, 1.0, 0.0);
    let vfov = 20.0;
    let camera = Camera::new(lookfrom, lookat, vup, vfov, aspect_ratio);

    // Create scene
    let mut scene = Scene::new();
    
    // Add objects to the scene
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, 0.0, -1.0),
        0.5,
        Box::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)))
    )));
    scene.add(Box::new(Sphere::new(
        Vec3::new(0.0, -100.5, -1.0),
        100.0,
        Box::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)))
    )));
    scene.add(Box::new(Sphere::new(
        Vec3::new(1.0, 0.0, -1.0),
        0.5,
        Box::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.3))
    )));

    // Render the scene
    println!("P3\n{} {}\n255", image_width, image_height);

    let start_time = Instant::now();
    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rng.gen::<f64>()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += scene.ray_color(&r, max_depth);
            }
            pixel_color.write_color(samples_per_pixel);
        }
    }

    let duration = start_time.elapsed();
    eprintln!("\nDone. Render time: {:?}", duration);
}
