use crate::camera::Camera;
use crate::material::Material;
use crate::scene::{self, Renderer, Scene};
use crate::vec_util;
use glam::*;
use obvhs::ray::{self, Ray, RayHit};
use obvhs::{BvhBuildParams, cwbvh::CwBvh, cwbvh::builder::build_cwbvh};

pub struct DefaultRendererConfig {
    pub samples_per_pixel: u32,
    pub max_depth: u32,
    pub bvh_params: BvhBuildParams,
}

pub struct DefaultRenderer;
impl Renderer for DefaultRenderer {
    type Config = DefaultRendererConfig;
    fn render(scene: Scene, camera: Camera, config: Self::Config) {}
}
struct Scattered {
    attenuation: Vec3A,
    scattered: Ray,
}
pub struct HitRecord {
    t: f32,
    hit_pos: Vec3A,
    front_face: bool,
    material_id: usize,
    normal: Vec3A,
}
impl DefaultRenderer {
    fn ray_color(scene: &Scene, bvh: &CwBvh, ray: Ray, depth: i32) -> Vec3A {
        if depth < 0 {
            return Vec3A::ZERO;
        }

        if let Some(hit_record) = Self::hit(scene, bvh, ray) {
            if let Some(scattered) = Self::scatter(
                ray,
                hit_record.t,
                hit_record.hit_pos,
                hit_record.front_face,
                scene.materials[hit_record.material_id],
                hit_record.normal,
            ) {
                return scattered.attenuation
                    * Self::ray_color(scene, bvh, scattered.scattered, depth - 1);
            }
            return Vec3A::ZERO;
        }

        let a = (ray.direction.normalize().y + 1.0) * 0.5;
        Vec3A::new(1.0, 1.0, 1.0) * (1.0 - a) + Vec3A::new(0.5, 0.7, 1.0) * a
    }
    fn hit(scene: &Scene, bvh: &CwBvh, ray_in: Ray) -> Option<HitRecord> {
        let mut ray_hit = RayHit::none();
        let mut normal = Vec3A::ZERO;
        let mut obj_id: usize = 0;
        if bvh.ray_traverse(ray_in, &mut ray_hit, |ray, id| {
            obj_id = bvh.primitive_indices[id] as usize;
            scene.objects[obj_id].intersect_and_normal(ray, &mut normal)
        }) {
            let front_face = ray_in.direction.dot(normal) < 0.0;
            let normal = if front_face { normal } else { -normal };
            Some(HitRecord {
                t: ray_hit.t,
                hit_pos: ray_in.origin + ray_in.direction * ray_hit.t,
                front_face: front_face,
                material_id: scene.objects[obj_id].material_id,
                normal,
            })
        } else {
            None
        }
    }
    fn scatter(
        ray_in: Ray,
        t: f32,
        hit_pos: Vec3A,
        front_face: bool,
        material: Material,
        normal: Vec3A,
    ) -> Option<Scattered> {
        match material {
            Material::Lambertian { albedo } => {
                let direction = normal + vec_util::random_unit_vector();
                Some(Scattered {
                    attenuation: albedo,
                    scattered: new_ray(hit_pos, direction),
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray_in.direction.reflect(normal);
                let reflected = reflected.normalize() + fuzz * vec_util::random_unit_vector();
                if reflected.dot(normal) > 0.0 {
                    Some(Scattered {
                        attenuation: albedo,
                        scattered: new_ray(hit_pos, reflected),
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
                let sin_theta = (1.0 - cos_theta * cos_theta).sqrt(); // todo: remove this sprt
                let direction =
                    if ri * sin_theta > 1.0 || reflectance(cos_theta, ri) > fastrand::f32() {
                        unit_dir.reflect(normal)
                    } else {
                        unit_dir.refract(normal, ri)
                    };
                Some(Scattered {
                    attenuation: Vec3A::ONE,
                    scattered: new_ray(hit_pos, direction),
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
