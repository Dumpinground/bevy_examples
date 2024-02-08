use bevy::{input::mouse::{MouseScrollUnit, MouseWheel}, prelude::*};

pub struct MyCameraPlugin;

impl Plugin for MyCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup,setup)
        .add_systems(Update, (move_camera, orbit_camera, lift_camera));
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2.5, 4.5, 9.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn move_camera(mut query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
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

        if direction.length() > 0. {
            direction = direction.normalize();
        }

        transform.translation += direction * speed * time.delta_seconds();
    }
}

fn orbit_camera(mut query: Query<&mut Transform, With<Camera>>, keyboard: Res<Input<KeyCode>>, time: Res<Time>) {
    
    if let Ok(mut transform) = query.get_single_mut() {

        // (x, z) = (z, -x);
        let translation = transform.translation;
        let distance_2 = translation.x.powi(2) + translation.y.powi(2);

        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::E) {
            direction += Vec3::new(translation.z, 0., -translation.x);
        }

        if keyboard.pressed(KeyCode::Q) {
            direction += Vec3::new(-translation.z, 0., translation.x);
        }

        if direction.length() > 0. {
            direction = direction.normalize();
            // println!("({}, {}, {})", direction.x, direction.z, direction.x.powi(2) + direction.z.powi(2));
        }

        let mut new_translation =  translation + direction * time.delta_seconds();
        let new_distance_2 = new_translation.x.powi(2) + new_translation.y.powi(2);
        let ratio = (distance_2 / new_distance_2).sqrt();
        (new_translation.x, new_translation.z) = (new_translation.x * ratio, new_translation.z * ratio);

        transform.translation = new_translation;
        transform.look_at(Vec3::ZERO, Vec3::Y);
        println!("{}", transform.translation);
    }
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