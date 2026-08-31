use crate::material::Material;
use crate::mesh::Mesh;
use glam::*;
use obvhs::{Boundable, ray::Ray};

#[derive(Clone, Copy, Debug)]
pub struct Object {
    mesh: Mesh,
    material_id: usize,
}
impl Boundable for Object {
    fn aabb(&self) -> obvhs::aabb::Aabb {
        self.mesh.aabb()
    }
}
impl Object {
    fn intersect_and_normal(&self, ray: &Ray, normal: &mut Vec3A) -> f32 {
        self.mesh.intersect_and_normal(ray, normal)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Camera {
    pub image_width: u32,
    pub image_height: u32,
    pub center: Vec3A,
    pub pixel00_loc: Vec3A,
    pub pixel_delta_u: Vec3A,
    pub pixel_delta_v: Vec3A,
    pub focus_dist: f32,
    pub defocus_disk_u: Vec3A,
    pub defocus_disk_v: Vec3A,
}
impl Camera {
    pub fn new(
        aspect_ratio: f32,
        image_width: u32,
        vfov: f32,
        lookfrom: Vec3A,
        lookat: Vec3A,
        vup: Vec3A,
        defocus_angle: f32,
        focus_dist: f32,
    ) {
    }
}

#[derive(Debug)]
pub struct Scene {
    pub materials: Vec<Material>,
    pub objects: Vec<Object>,
}
impl Scene {
    pub fn new() -> Self {
        Self {
            materials: Vec::new(),
            objects: Vec::new(),
        }
    }
}

pub trait Renderer {
    fn render(scene: Scene);
}
