use std::f32::consts::TAU;

use bevy::prelude::*;

#[derive(Component)]
struct Name(String);

#[derive(Component)]
struct Orbital {
    speed: f32,
}

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
            mesh: meshes.add(Mesh::from(shape::Cube { size: 0.4 })),
            material: materials.add(Color::rgb_u8(124, 144, 255).into()),
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
