use bevy::prelude::*;
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};

pub struct MyPanOrbitCameraPlugin;

impl Plugin for MyPanOrbitCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PanOrbitCameraPlugin)
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((Camera3dBundle {
        transform: Transform::from_translation(Vec3::new(0., 1.5, 5.0)),
        ..default()
    }, PanOrbitCamera::default()));
}