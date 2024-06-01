mod camera;
mod debug;
mod movement;
mod spaceship;

use bevy::prelude::*;
use camera::CameraPlugin;
use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

#[derive(Component, Debug)]
struct Velocity {
    pub value: Vec3,
}

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            brightness: 700.,
            ..default()
        })
        .add_plugins((
            DefaultPlugins,
            SpaceshipPlugin,
            MovementPlugin,
            DebugPlugin,
            CameraPlugin,
        ))
        .run();
}
