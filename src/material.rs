use glam::*;

#[derive(Clone, Copy, Debug)]
pub enum Material {
    Lambertian { albedo: Vec3A },
    Metal { albedo: Vec3A, fuzz: f32 },
    Dielectric { refraction_index: f32 },
}

pub fn lambertian(r: f32, g: f32, b: f32) -> Material {
    Material::Lambertian {
        albedo: vec3a(r, g, b),
    }
}
pub fn metal(r: f32, g: f32, b: f32, fuzz: f32) -> Material {
    Material::Metal {
        albedo: vec3a(r, g, b),
        fuzz,
    }
}
pub fn dielectric(refraction_index: f32) -> Material {
    Material::Dielectric { refraction_index }
}
