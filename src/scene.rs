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
