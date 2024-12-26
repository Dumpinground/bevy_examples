mod components;
mod input;

use bevy::{prelude::*, render::camera::ScalingMode};
use bevy_asset_loader::prelude::*;
use bevy_ggrs::*;
use bevy_matchbox::prelude::*;

use crate::{components::*, input::*};

type Config = GgrsConfig<u8, PeerId>;

const MAP_SIZE: u32 = 41;
const GRID_WIDTH: f32 = 0.05;

#[derive(AssetCollection, Resource)]
struct ImageAssets {
    #[asset(path = "bullet.png")]
    bullet: Handle<Image>,
}

#[derive(States, Clone, Eq, PartialEq, Debug, Hash, Default)]
enum GameState {
    #[default]
    AssetLoading,
    Matchmaking,
    InGame,
}

fn main() {
    App::new()
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::AssetLoading)
                .load_collection::<ImageAssets>()
                .continue_to_state(GameState::Matchmaking),
        )
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // fill the entire browser window
                    fit_canvas_to_parent: true,
                    // don't hijack keyboard shortcuts like F5, F6, F12, Ctrl+R etc.
                    prevent_default_event_handling: false,
                    ..default()
                }),
                ..default()
            }),
            GgrsPlugin::<Config>::default(),
        ))
        .rollback_component_with_clone::<Transform>()
        .rollback_component_with_copy::<BulletReady>()
        .insert_resource(ClearColor(Color::srgb(0.53, 0.53, 0.53)))
        .add_systems(
            OnEnter(GameState::Matchmaking),
            (setup, start_matchbox_socket),
        )
        .add_systems(OnEnter(GameState::InGame), spawn_players)
        .add_systems(
            Update,
            (
                wait_for_players.run_if(in_state(GameState::Matchmaking)),
                camera_follow.run_if(in_state(GameState::InGame)),
            ),
        )
        .add_systems(ReadInputs, read_local_inputs)
        .add_systems(
            GgrsSchedule,
            (move_players, reload_bullet, fire_bullets, move_bullet).chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical {
                viewport_height: 10.,
            },
            ..OrthographicProjection::default_2d()
        },
    ));

    // Horizontal lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            Transform::from_translation(Vec3::new(0., i as f32 - MAP_SIZE as f32 / 2., 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(MAP_SIZE as f32, GRID_WIDTH)),
                ..default()
            },
        ));
    }

    // Vertical lines
    for i in 0..=MAP_SIZE {
        commands.spawn((
            Transform::from_translation(Vec3::new(i as f32 - MAP_SIZE as f32 / 2., 0., 0.)),
            Sprite {
                color: Color::srgb(0.27, 0.27, 0.27),
                custom_size: Some(Vec2::new(GRID_WIDTH, MAP_SIZE as f32)),
                ..default()
            },
        ));
    }
}

fn camera_follow(
    local_players: Res<LocalPlayers>,
    players: Query<(&Player, &Transform)>,
    mut camera: Query<&mut Transform, (With<Camera>, Without<Player>)>,
) {
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
            Player { handle: 0 },
            BulletReady(true),
            MoveDir(-Vec2::X),
            Transform::from_translation(Vec3::new(-2., 0., 100.)),
            Sprite {
                color: Color::srgb(0., 0.47, 1.),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        ))
        .add_rollback();

    // Player 2
    commands
        .spawn((
            Player { handle: 1 },
            BulletReady(true),
            MoveDir(Vec2::X),
            Transform::from_translation(Vec3::new(2., 0., 100.)),
            Sprite {
                color: Color::srgb(0., 0.47, 1.),
                custom_size: Some(Vec2::ONE),
                ..default()
            },
        ))
        .add_rollback();
}

fn move_players(
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut Transform, &mut MoveDir, &Player)>,
    time: Res<Time>,
) {
    for (mut transform, mut move_dir, player) in &mut players {
        let (input, _) = inputs[player.handle];

        let direction = direction(input);

        if direction == Vec2::ZERO {
            continue;
        }

        move_dir.0 = direction;

        let move_speed = 7.;
        let move_delta = direction * move_speed * time.delta_secs();

        let old_ops = transform.translation.xy();
        let limit = Vec2::splat(MAP_SIZE as f32 / 2. - 0.5);
        let new_pos = (old_ops + move_delta).clamp(-limit, limit);

        transform.translation.x = new_pos.x;
        transform.translation.y = new_pos.y;
    }
}

fn fire_bullets(
    mut commands: Commands,
    inputs: Res<PlayerInputs<Config>>,
    images: Res<ImageAssets>,
    mut players: Query<(&Transform, &Player, &mut BulletReady, &MoveDir)>,
) {
    for (transform, player, mut bullet_ready, move_dir) in &mut players {
        // Spawn bullet
        let (input, _) = inputs[player.handle];
        if fire(input) && bullet_ready.0 {
            commands
                .spawn((
                    Bullet,
                    Transform::from_translation(transform.translation),
                    *move_dir,
                    Sprite {
                        image: images.bullet.clone(),
                        custom_size: Some(Vec2::new(0.3, 0.1)),
                        ..default()
                    },
                ))
                .add_rollback();
            bullet_ready.0 = false;
        }
    }
}

fn reload_bullet(
    inputs: Res<PlayerInputs<Config>>,
    mut players: Query<(&mut BulletReady, &Player)>,
) {
    for (mut can_fire, player) in players.iter_mut() {
        let (input, _) = inputs[player.handle];
        if !fire(input) {
            can_fire.0 = true
        }
    }
}

fn move_bullet(mut bullets: Query<(&mut Transform, &MoveDir), With<Bullet>>, time: Res<Time>) {
    for (mut transform, dir) in &mut bullets {
        let speed = 20.;
        let delta = dir.0 * speed * time.delta_secs();
        transform.translation += delta.extend(0.);
    }
}

fn start_matchbox_socket(mut commands: Commands) {
    let room_url = "ws://localhost:3536/extreme_bevy?next=2";
    info!("connecting to matchbox server: {room_url}");
    commands.insert_resource(MatchboxSocket::new_unreliable(room_url))
}

fn wait_for_players(
    mut commands: Commands,
    mut socket: ResMut<MatchboxSocket>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if socket.get_channel(0).is_err() {
        return; // we've already started
    }

    // Check for new connections
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

    next_state.set(GameState::InGame);
}
