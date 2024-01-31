use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_systems(Startup,setup)
        .add_systems(Update, move_camera);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_camera(mut query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, mut wheel_event: EventReader<MouseWheel>, time: Res<Time>) {
    let speed = 2.;

    if let Ok(mut transform) = query.get_single_mut() {
        
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::A) {
            direction += Vec3::new(-1., 0., 0.);
        }
        if keyboard.pressed(KeyCode::D) {
            direction += Vec3::new(1., 0., 0.);
        }
        if keyboard.pressed(KeyCode::W) {
            direction += Vec3::new(0., 0., 1.);
        }
        if keyboard.pressed(KeyCode::S) {
            direction += Vec3::new(0., 0., -1.);
        }

        for event in wheel_event.read() {
            match event.unit {
                MouseScrollUnit::Line | MouseScrollUnit::Pixel => direction += Vec3::new(0., event.y, 0.),
            }
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_seconds();
    }
}

fn print_wheel_scroll(mut wheel_event: EventReader<MouseWheel>) {
    for event in wheel_event.read() {
        match event.unit {
            MouseScrollUnit::Line => println!("line vertical: {}, horizontal: {}", event.y, event.x),
            MouseScrollUnit::Pixel => println!("pixel vertical: {}, horizontal: {}", event.y, event.x),
        }
    }
}