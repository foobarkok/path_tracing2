mod camera;
mod default_renderer;
mod material;
mod mesh;
mod object;
mod renderer;
mod scene;
mod vec_util;

use glam::vec3a;
use material::*;
use mesh::Mesh;
use obvhs::{BvhBuildParams, triangle::Triangle};
use renderer::Renderer;
use scene::Scene;

use crate::{
    camera::Camera,
    default_renderer::{DefaultRenderer, DefaultRendererConfig},
};

fn main() {
    let mut scene = Scene::new();

    let ground_material = scene.register_material(lambertian(0.5, 0.5, 0.5));
    const GROUND_SIZE: f32 = 25.0;
    const GROUND: [Triangle; 2] = [
        Triangle {
            v0: vec3a(GROUND_SIZE, 0.0, GROUND_SIZE),
            v1: vec3a(-GROUND_SIZE, 0.0, -GROUND_SIZE),
            v2: vec3a(-GROUND_SIZE, 0.0, GROUND_SIZE),
        },
        Triangle {
            v0: vec3a(GROUND_SIZE, 0.0, GROUND_SIZE),
            v1: vec3a(GROUND_SIZE, 0.0, -GROUND_SIZE),
            v2: vec3a(-GROUND_SIZE, 0.0, -GROUND_SIZE),
        },
    ];
    scene.add_tris(GROUND.into_iter(), ground_material);

    let glass_material = scene.register_material(dielectric(1.5));

    for a in -11..11 {
        for b in -11..11 {
            let center = vec3a(
                a as f32 + 0.9 * fastrand::f32(),
                0.2,
                b as f32 + 0.9 * fastrand::f32(),
            );

            if (center - vec3a(4.0, 0.2, 0.0)).length_squared() > 0.9 * 0.9 {
                let choose_mat = fastrand::f32();
                let mat = if choose_mat < 0.8 {
                    let albedo = vec_util::random(0.0, 1.0) * vec_util::random(0.0, 1.0);
                    scene.register_material(Material::Lambertian { albedo })
                } else if choose_mat < 0.95 {
                    let albedo = vec_util::random(0.5, 1.0);
                    let fuzz = 0.5 * fastrand::f32();
                    scene.register_material(Material::Metal { albedo, fuzz })
                } else {
                    glass_material
                };
                scene.add_object(Mesh::Sphere { center, r: 0.2 }, mat);
            }
        }
    }

    scene.add_object(
        Mesh::Sphere {
            center: vec3a(0.0, 1.0, 0.0),
            r: 1.0,
        },
        glass_material,
    );
    let material2 = scene.register_material(lambertian(0.4, 0.2, 0.1));
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(-4.0, 1.0, 0.0),
            r: 1.0,
        },
        material2,
    );
    let material3 = scene.register_material(metal(0.7, 0.6, 0.5, 0.0));
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(4.0, 1.0, 0.0),
            r: 1.0,
        },
        material3,
    );

    let cam = Camera::new(
        16.0 / 9.0,
        1200,
        20.0,
        vec3a(13.0, 2.0, 3.0),
        vec3a(0.0, 0.0, 0.0),
        vec3a(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    <DefaultRenderer as Renderer>::render(
        scene,
        cam,
        DefaultRendererConfig {
            samples_per_pixel: 500,
            max_depth: 50,
            bvh_params: BvhBuildParams::medium_build(),
        },
    );
}
