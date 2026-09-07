mod camera;
mod default_renderer;
mod material;
mod mesh;
mod object;
mod renderer;
mod scene;
mod vec_util;

use glam::vec3a;
use material::Material;
use mesh::Mesh;
use obvhs::{BvhBuildParams, cwbvh::builder::build_cwbvh};
use renderer::Renderer;
use scene::Scene;
use std::time::Duration;

use crate::{
    camera::Camera,
    default_renderer::{DefaultRenderer, DefaultRendererConfig},
};

fn main() {
    let mut scene = Scene::new();

    let material_ground = scene.register_material(Material::Lambertian {
        albedo: vec3a(0.8, 0.8, 0.0),
    });
    let material_center = scene.register_material(Material::Lambertian {
        albedo: vec3a(0.1, 0.2, 0.5),
    });
    let material_left = scene.register_material(Material::Dielectric {
        refraction_index: 1.50,
    });
    let material_bubble = scene.register_material(Material::Dielectric {
        refraction_index: 1.0 / 1.50,
    });
    let material_right = scene.register_material(Material::Metal {
        albedo: vec3a(0.8, 0.6, 0.2),
        fuzz: 1.0,
    });

    scene.add_object(
        Mesh::Sphere {
            center: vec3a(0.0, -100.5, -1.0),
            r: 100.0,
        },
        material_ground,
    );
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(0.0, 0.0, -1.2),
            r: 0.5,
        },
        material_center,
    );
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(-1.0, 0.0, -1.0),
            r: 0.5,
        },
        material_left,
    );
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(-1.0, 0.0, -1.0),
            r: 0.4,
        },
        material_bubble,
    );
    scene.add_object(
        Mesh::Sphere {
            center: vec3a(1.0, 0.0, -1.0),
            r: 0.5,
        },
        material_right,
    );

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        20.0,
        vec3a(-2.0, 2.0, 1.0),
        vec3a(0.0, 0.0, -1.0),
        vec3a(0.0, 1.0, 0.0),
        10.0,
        3.4,
    );

    <DefaultRenderer as Renderer>::render(
        scene,
        cam,
        DefaultRendererConfig {
            samples_per_pixel: 100,
            max_depth: 50,
            bvh_params: BvhBuildParams::medium_build(),
        },
    );
}
