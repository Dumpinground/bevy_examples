use bevy::prelude::*;

#[derive(Component, Debug)]
struct Velocity {
    pub value: Vec3,
}

pub struct SpaceshipPlugin;

impl Plugin for SpaceshipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_spaceship);
    }
}

pub struct MovementPlugin;

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_position);
    }
}

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, print_position);
    }
}

fn main() {
    App::new().add_plugins(DefaultPlugins).run();
}

fn spawn_spaceship(mut commands: Commands) {
    commands.spawn((
        SpatialBundle::default(),
        Velocity {
            value: Vec3::default(),
        },
    ));
}

fn update_position(mut query: Query<(&Velocity, &mut Transform)>) {
    for (velocity, mut transform) in query.iter_mut() {
        transform.translation.x += velocity.value.x;
        transform.translation.y += velocity.value.y;
        transform.translation.z += velocity.value.z;
    }
}

fn print_position(query: Query<(Entity, &Transform)>) {
    // log the entity ID and position of each entitu with a `Position` component.
    for (entity, transform) in query.iter() {
        info!("Entity {:?} is at position {:?}.", entity, transform);
    }
}
