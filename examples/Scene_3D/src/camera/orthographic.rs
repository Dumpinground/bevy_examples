use bevy::{prelude::*, render::camera::ScalingMode};

pub struct OrthographicCameraPlugin;

impl Plugin for OrthographicCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(6.),
            ..default()
        }.into(),
        transform: Transform::from_xyz(5., 5., 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}