use glam::*;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3A },
    Metal { albedo: Vec3A, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}
