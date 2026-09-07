use crate::{camera::Camera, scene::Scene};

pub trait Renderer {
    type Config;
    fn render(scene: Scene, camera: Camera, config: Self::Config);
}
