use std::f32::consts::{FRAC_PI_2, TAU};

use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Orbital {
    speed: f32,
}

#[derive(Component)]
pub struct Ground;

#[derive(Component)]
pub struct Round;

pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, orbit_cube);
    }
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Circle::new(4.)),
            material: materials.add(Color::WHITE),
            transform: Transform::from_rotation(Quat::from_rotation_x(-FRAC_PI_2)),
            ..default()
        },
        Round,
    ));

    commands.spawn((PbrBundle {
        mesh: meshes.add(Plane3d::default().mesh().size(20., 20.)),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3)),
        transform: Transform::from_xyz(0., -1., 0.),
        ..default()
    }, Ground));

    commands.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(1., 1., 1.)),
        material: materials.add(Color::rgb_u8(124, 144, 255)),
        transform: Transform::from_xyz(0., 0.5, 0.),
        ..default()
    });

    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500000.,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });

    // let pl = PointLight::default();
    // println!("{:?}", pl);

    commands.spawn((
        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.4, 0.4, 0.4)),
            material: materials.add(Color::rgb_u8(124, 144, 255)),
            transform: Transform::from_xyz(2., 0.2, 0.),
            ..default()
        },
        Orbital { speed: 0.3 },
        Name("orbit cube".into()),
    ));
}

fn orbit_cube(mut orbit_cubes: Query<(&mut Transform, &Orbital)>, timer: Res<Time>) {
    for (mut transform, orbital) in &mut orbit_cubes {
        let angle = orbital.speed * TAU * timer.delta_seconds();
        let move_length = 0.5 * timer.delta_seconds();

        transform.rotate_y(angle);
        transform.translate_around(Vec3::ZERO, Quat::from_rotation_y(angle))
    }
}