use crate::hittable::{Hittable, HitRecord};
use crate::ray::Ray;
use crate::color::Color;
use crate::vec3::Vec3;

pub struct Scene {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }

    pub fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, t_min, closest_so_far) {
                closest_so_far = temp_rec.t;
                hit_record = Some(temp_rec);
            }
        }

        hit_record
    }

    pub fn ray_color(&self, r: &Ray, depth: i32) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        // Check if the ray hits any object in the scene
        if let Some(rec) = self.hit(r, 0.001, f64::INFINITY) {
            // If the material scatters the ray, continue tracing
            if let Some((attenuation, scattered)) = rec.material.scatter(r, &rec) {
                return attenuation * self.ray_color(&scattered, depth - 1);
            }
            // If the material absorbs the ray, return black
            return Color::new(0.0, 0.0, 0.0);
        }

        // If the ray doesn't hit anything, return the background color (sky)
        let unit_direction = r.direction.unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
