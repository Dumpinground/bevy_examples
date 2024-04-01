mod components;
mod input;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

use crate::{components::*, input::*};

type Config = GgrsConfig<u8, PeerId>;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
        ))
        .rollback_component_with_clone::<Transform>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .add_systems(Startup, (setup, spawn_players, start_matchbox_socket))
        .add_systems(Update, (wait_for_players, camera_follow))
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(GgrsSchedule, move_players)
        .run();
}

fn setup(mut commands: Commands) {
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.projection.scaling_mode = ScalingMode::FixedVertical(10.);
    commands.spawn(camera_bundle);
}

fn camera_follow(local_players: Res<LocalPlayers>, players: Query<(&Player, &Transform)>, mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>) {
    for (player, player_transform) in &players {
        if !local_players.0.contains(&player.handle) {
            continue;
        }

        let pos = player_transform.translation;

        for mut transform in &mut camera {
            transform.translation.x = pos.x;
            transform.translation.y = pos.y;
        }
    }
}

fn spawn_players(mut commands: Commands) {
    // Player 1
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(-2., 0., 0.)),
                sprite: Sprite {
                    color: Color::rgb(0., 0.47, 1.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            Player { handle: 0 },
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(2., 0., 0.)),
                sprite: Sprite {
                    color: Color::rgb(0., 0.4, 0.),
                    custom_size: Some(Vec2::new(1., 1.)),
                    ..default()
                },
                ..default()
            },
            Player { handle: 1 },
        ))
        .add_rollback();
}

fn move_players(
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut Transform, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let direction = direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_seconds();

        transform.translation += move_delta.extend(0.);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://localhost:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_ggrs(room_url))
}

fn wait_for_players(mut commands: Commands, mut socket: ResMut<MatchboxSocket<SingleChannel>>) {
    if socket.get_channel(0).is_err() {
        return;
    }

    socket.update_peers();
    let players = socket.players();

    let num_players = 2;
    if players.len() < num_players {
        return;
    }

    info!("All peers have joined, going in-game");

    let mut session_builder = ggrs::SessionBuilder::<Config>::new()
        .with_num_players(num_players)
        .with_input_delay(2);

    for (i, player) in players.into_iter().enumerate() {
        session_builder = session_builder
            .add_player(player, i)
            .expect("failed to add player");
    }

    let channel = socket.take_channel(0).unwrap();

    let ggrs_session = session_builder
        .start_p2p_session(channel)
        .expect("failed to start session");

    commands.insert_resource(Session::P2P(ggrs_session));
}
