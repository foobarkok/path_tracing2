use crate::mesh::Mesh;
use glam::*;
use obvhs::{Boundable, ray::Ray};

#[derive(Clone, Copy, Debug)]
pub struct Object {
    pub mesh: Mesh,
    pub material_id: usize,
}
impl Boundable for Object {
    fn aabb(&self) -> obvhs::aabb::Aabb {
        self.mesh.aabb()
    }
}
impl Object {
    pub fn intersect_and_normal(&self, ray: &Ray, normal: &mut Vec3A) -> f32 {
        self.mesh.intersect_and_normal(ray, normal)
    }
}
