mod camera;
mod env;
mod scene;

use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use camera::CameraPlugin;
use scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ScenePlugin))
        .run();
}
