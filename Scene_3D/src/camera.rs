use bevy::prelude::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup,setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_camera(query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    let transform = query.get_single();

    if keyboard.pressed(KeyCode::A) {
        
    }
}