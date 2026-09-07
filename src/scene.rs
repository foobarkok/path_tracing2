use crate::{material::Material, mesh::Mesh, object::Object};

#[derive(Debug, Clone)]
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
    pub fn register_material(&mut self, material: Material) -> usize {
        self.materials.push(material);
        self.materials.len() - 1
    }
    pub fn add_object(&mut self, mesh: Mesh, material_id: usize) {
        self.objects.push(Object { mesh, material_id });
    }
}
