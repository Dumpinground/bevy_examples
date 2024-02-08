use std::f32::consts::TAU;

use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,setup)
        .add_systems(Update, (pan_camera, orbit_camera, lift_camera));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn pan_camera(mut query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    let speed = 2.;

    let move_length = speed * time.delta_seconds();

    if let Ok(mut transform) = query.get_single_mut() {
        
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::A) {
            direction += -Vec3::X;
        }
        if keyboard.pressed(KeyCode::D) {
            direction += Vec3::X;
        }
        if keyboard.pressed(KeyCode::W) {
            direction += Vec3::Z;
        }
        if keyboard.pressed(KeyCode::S) {
            direction += -Vec3::Z;
        }

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_seconds();
    }
}

fn orbit_camera(mut query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    
    if let Ok(mut transform) = query.get_single_mut() {

        let angle = 0.3 * TAU * time.delta_seconds();

        if keyboard.pressed(KeyCode::E) {
            orbit(&mut transform, angle);
        }

        if keyboard.pressed(KeyCode::Q) {
            orbit(&mut transform, -angle);
        }
    }
}

fn orbit(transform: &mut Mut<'_, Transform>, angle: f32) {
    
    transform.rotate_around(Vec3::ZERO, Quat::from_rotation_y(angle));
    transform.look_at(Vec3::ZERO, Vec3::Y);
}

fn lift_camera(mut query: Query<&mut Transform, With<Camera>>, mut wheel_event: EventReader<MouseWheel>, time: Res<Time>) {
    let speed = 4.;

    if let Ok(mut transform) = query.get_single_mut() {
        let mut direction = Vec3::ZERO;

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