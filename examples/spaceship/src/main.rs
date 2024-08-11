mod asset_loader;
mod asteroids;
mod camera;
mod collision_detection;
mod debug;
mod movement;
mod spaceship;
mod despawn;

use asset_loader::AssetLoaderPlugin;
use asteroids::AsteroidPlugin;
use bevy::prelude::*;
use camera::CameraPlugin;
use collision_detection::CollisionsDetectionPlugin;
use despawn::DespawnPlugin;
// use debug::DebugPlugin;
use movement::MovementPlugin;
use spaceship::SpaceshipPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.1, 0., 0.15)))
        .insert_resource(AmbientLight {
            brightness: 700.,
            ..default()
        })
        .add_plugins((
            DefaultPlugins,
            AssetLoaderPlugin,
            SpaceshipPlugin,
            AsteroidPlugin,
            MovementPlugin,
            CollisionsDetectionPlugin,
            // DebugPlugin,
            CameraPlugin,
            DespawnPlugin,
        ))
        .run();
}
