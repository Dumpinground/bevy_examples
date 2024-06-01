mod camera;
mod env;
mod scene;

use bevy::prelude::*;
use camera::CameraPlugin;
use scene::ScenePlugin;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, ScenePlugin))
        .run();
}
