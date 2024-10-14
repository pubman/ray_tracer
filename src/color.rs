use crate::vec3::Vec3;

pub type Color = Vec3;

impl Color {
    pub fn write_color(&self, samples_per_pixel: i32) {
        let scale = 1.0 / samples_per_pixel as f64;
        let r = (self.x * scale).sqrt();
        let g = (self.y * scale).sqrt();
        let b = (self.z * scale).sqrt();

        println!(
            "{} {} {}",
            (256.0 * r.clamp(0.0, 0.999)) as i32,
            (256.0 * g.clamp(0.0, 0.999)) as i32,
            (256.0 * b.clamp(0.0, 0.999)) as i32
        );
    }
}

