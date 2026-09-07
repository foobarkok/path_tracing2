use crate::{camera::Camera, material::Material, object::Object};

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
}

pub trait Renderer {
    type Config;
    fn render(scene: Scene, camera: Camera, config: Self::Config);
}
