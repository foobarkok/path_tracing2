mod default_renderer;
mod material;
mod mesh;
mod scene;
mod vec_util;

use obvhs::{BvhBuildParams, cwbvh::builder::build_cwbvh};
use scene::Scene;
use std::time::Duration;

fn main() {
    let mut core_build_time = Duration::default();
    let scene = Scene::new();
    let bvh = build_cwbvh(
        &scene.objects,
        BvhBuildParams::medium_build(),
        &mut core_build_time,
    );
}
