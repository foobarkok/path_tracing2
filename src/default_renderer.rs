use crate::material::Material;
use crate::scene::{Object, Renderer, Scene};
use crate::vec_util;
use glam::*;
use obvhs::ray::Ray;

pub struct DefaultRenderer;
impl Renderer for DefaultRenderer {
    fn render(scene: Scene) {}
}
struct Scattered {
    attenuation: Vec3A,
    scattered: Ray,
}
impl DefaultRenderer {
    fn scatter(
        ray_in: &Ray,
        t: f32,
        hit_pos: &Vec3A,
        front_face: bool,
        material: Material,
        normal: Vec3A,
    ) -> Option<Scattered> {
        match material {
            Material::Lambertian { albedo } => {
                let direction = normal + vec_util::random_unit_vector();
                Some(Scattered {
                    attenuation: albedo,
                    scattered: new_ray(*hit_pos, direction),
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction.reflect(normal);
                let reflected = reflected.normalize() + fuzz * vec_util::random_unit_vector();
                if reflected.dot(normal) > 0.0 {
                    Some(Scattered {
                        attenuation: albedo,
                        scattered: new_ray(*hit_pos, reflected),
                    })
                } else {
                    None
                }
            }
            Material::Dielectric { refraction_index } => {
                let ri = if front_face {
                    1.0 / refraction_index
                } else {
                    refraction_index
                };
                let unit_dir = ray_in.direction.normalize();
                let cos_theta = -unit_dir.dot(normal);
                let cos_theta = if cos_theta < 1.0 { cos_theta } else { 1.0 };
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
                let direction =
                    if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > fastrand::f32() {
                        unit_dir.reflect(normal)
                    } else {
                        unit_dir.refract(normal, ri)
                    };
                Some(Scattered {
                    attenuation: Vec3A::ONE,
                    scattered: new_ray(*hit_pos, direction),
                })
            }
        }
    }
}
fn new_ray(origin: Vec3A, direction: Vec3A) -> Ray {
    Ray::new(origin, direction, 1e-4, f32::INFINITY)
}
fn reflectance(cosine: f32, refraction_index: f32) -> f32 {
    let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    let r0 = r0 * r0;
    let x = 1.0 - cosine;
    let x2 = x * x;
    let x5 = x2 * x2 * x;
    r0 + (1.0 - r0) * x5
}
