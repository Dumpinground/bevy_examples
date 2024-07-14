use std::{env::temp_dir, path::Path};

use bevy::{pbr::wireframe::Wireframe, prelude::*, render::{mesh::{PlaneMeshBuilder, SphereKind, VertexAttributeValues}, render_resource::TextureFormat}};

#[cfg(feature = "atmosphere")]
use bevy::{time::Stopwatch, utils::Duration};
use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use bevy_water::{ImageReformat, ImageUtilsPlugin, WaterParam, WaterPlugin, WaterSettings};

const WATER_HEIGHT: f32 = 1.;
#[cfg(feature = "atmosphere")]
const SPEED_MIN: f32 = 0.05;
#[cfg(feature = "atmosphere")]
const SPEED_DELTA: f32 = 0.01;
#[cfg(feature = "atmosphere")]
const SPEED_MAX: f32 = 1.;

fn main() {
    let mut app = App::new();
    app.add_plugins(
        DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Pirates".to_string(),
                    resolution: (1200., 600.).into(),
                    ..default()
                }),
                ..default()
            })
            .set(AssetPlugin::default()),
    );

    #[cfg(feature = "panorbit")]
    app.add_plugins(PanOrbitCameraPlugin);

    app.insert_resource(bevy::pbr::DirectionalLightShadowMap { size: 4 * 1024 })
        .insert_resource(WaterSettings {
            height: WATER_HEIGHT,
            ..default()
        })
        .add_plugins((WaterPlugin, ImageUtilsPlugin))
        .add_systems(Startup, setup);

    app.run();
}

fn handle_quit(input: Res<ButtonInput<KeyCode>>, mut exit: EventWriter<AppExit>) {
    if input.pressed(KeyCode::KeyQ) {
        exit.send(AppExit::Success);
    }
}

#[derive(Resource, Clone, Debug, Default)]
struct UiState {
    show_wireframe: bool,
}

fn toggle_wireframe(
    input: Res<ButtonInput<KeyCode>>,
    query: Query<Entity, With<Handle<Mesh>>>,
    mut commands: Commands,
    mut state: ResMut<UiState>,
) {
    if input.just_pressed(KeyCode::KeyR) {
        let show_wireframe = !state.show_wireframe;
        state.show_wireframe = show_wireframe;

        for entity in query.iter() {
            let mut entity = commands.entity(entity);
            if show_wireframe {
                entity.insert(Wireframe);
            } else {
                entity.remove::<Wireframe>();
            }
        }
    }
}

#[derive(Component)]
struct Sun;

#[derive(Resource)]
#[cfg(feature = "atmosphere")]
struct CycleTimer {
    update: Timer,
    time: Stopwatch,
    speed: f32,
}

#[cfg(feature = "atmosphere")]
impl CycleTimer {
    pub fn new(duration: Duration, speed: f32) -> Self {
        Self {
            update: Timer::new(duration, TimerMode::Repeating),
            time: Stopwatch::new(),
            speed,
        }
    }
}

#[derive(Bundle, Default)]
struct ShipBundle {
    ship: Ship,
    name: Name,
    spatial: SpatialBundle,
}

#[derive(Component, Default, Clone)]
struct Ship {
    water_line: f32,
    front: Vec3,
    back_left: Vec3,
    back_right: Vec3,
}

impl Ship {
    fn new(
        water_line: f32,
        front: f32,
        back: f32,
        left: f32,
        right: f32,
        #[cfg(feature = "debug")]
        lines: &mut DebugLines,
    ) -> Self {
        Self {
            water_line,
            front: Vec3::new(0., 0., front),
            back_left: Vec3::new(left, 0., back),
            back_right: Vec3::new(right, 0., back),
        }
    }

    fn update(&self, water: &WaterParam, pos: Vec3, transform: &mut Transform) {
        let (yaw, _pitch, _roll) = transform.rotation.to_euler(EulerRot::YZX);
        let global = Transform::from_translation(pos).with_rotation(Quat::from_rotation_y(yaw));

        // Get the wave position at the front, back left and back right.
        let mut front = water.wave_point(global.transform_point(self.front));
        let left = water.wave_point(global.transform_point(self.back_left));
        let right = water.wave_point(global.transform_point(self.back_right));
        let normal = (left - front).cross(right - front).normalize();
        
        front.y += self.water_line - 0.2;
        transform.look_at(front, normal);

        transform.translation.y = ((front.y + left.y + right.y) / 3.) + self.water_line;
    }
}

fn update_ships(
    water: WaterParam,
    mut ships: Query<(&Ship, &mut Transform, &GlobalTransform)>,
) {
    for (ship, mut transform, global) in ships.iter_mut() {
        let pos = global.translation();
    }
}

fn scale_uvs(mesh: &mut Mesh, scale: f32) {
    match mesh.attribute_mut(Mesh::ATTRIBUTE_UV_0) {
        Some(VertexAttributeValues::Float32x2(uvs)) => {
            for [x, y] in uvs.iter_mut() {
                *x *= scale;
                *y *= scale;
            }
        }
        Some(_) => {
            panic!("Unexpected UV format");
        }
        _ => {
            panic!("Mesh doesn't have UVS");
        }
    }
}

macro_rules! texture_dir {
    ($name:expr) => {
        concat!("textures/coast_sand_01_1k/", $name, ".jpg")
    };
}

#[test]
fn text() {
    let a = texture_dir!("diff");
    println!("{a}");
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 11127.65,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_rotation_x(-0.340)),
        ..default()
    }).insert(Sun); // Marks the light as Sun

    // Prepare textures
    let base_color_texture = Some(asset_server.load(texture_dir!("diff")));
    let metallic_roughness_texture = Some(ImageReformat::reformat(&mut commands, &asset_server, texture_dir!("rough"), TextureFormat::Rgba8Unorm));
    let normal_map_texture = Some(ImageReformat::reformat(&mut commands, &asset_server, texture_dir!("normal"), TextureFormat::Rgba8Unorm));
    ImageReformat::uv_repeat(&mut commands, &asset_server, texture_dir!("diff"));
    ImageReformat::uv_repeat(&mut commands, &asset_server, texture_dir!("rough"));
    ImageReformat::uv_repeat(&mut commands, &asset_server, texture_dir!("normal"));

    // Coast sand material
    let sandy = materials.add(StandardMaterial {
        perceptual_roughness: 1.,
        metallic: 0.,
        reflectance: 0.5,
        base_color_texture,
        metallic_roughness_texture,
        normal_map_texture,
        cull_mode: None,
        double_sided: true,
        ..default()
    });

    let floor_mesh = {
        let mut mesh = PlaneMeshBuilder::from_length(256. * 6.).subdivisions(25).build();
        mesh.generate_tangents().expect("tangents");
        scale_uvs(&mut mesh, 50.);
        meshes.add(mesh)
    };

    commands.spawn((MaterialMeshBundle {
        mesh: floor_mesh.clone(),
        material: sandy.clone(),
        transform: Transform::from_xyz(0., -5., 0.),
        ..default()
    }, Name::new("Sea floor".to_string())));

    let island_mesh = {
        let mut mesh = Sphere::new(2.).mesh().kind(SphereKind::Uv { sectors: 90, stacks: 60 }).build();
        mesh.generate_tangents().expect("tangents");
        scale_uvs(&mut mesh, 20.);
        meshes.add(mesh)
    };

    commands.spawn((MaterialMeshBundle {
        mesh: island_mesh.clone(),
        material: sandy.clone(),
        transform: Transform::from_xyz(-30., -10., -30.).with_scale(Vec3::new(30., 6.5, 30.)),
        ..default()
    }, Name::new("Sandy island".to_string())));

    let orb_mesh = {
        let mut mesh = Sphere::new(1.).mesh().kind(SphereKind::Uv { sectors: 90, stacks: 60 }).build();
        mesh.generate_tangents().expect("tangents");
        meshes.add(mesh)
    };

    commands.spawn((MaterialMeshBundle {
        mesh: orb_mesh.clone(),
        material: materials.add(Color::srgba(0.1, 0.2, 0.4, 1.)),
        transform: Transform::from_xyz(-30., -10., -30.),
        ..default()
    }, Name::new("Orb".to_string())));

    // camera
    let mut cam = commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(-20., WATER_HEIGHT + 5., 20.).looking_at(Vec3::new(0., WATER_HEIGHT, 0.), Vec3::Y),
            ..default()
        },
        EnvironmentMapLight {
            diffuse_map: asset_server.load("environment_maps/table_mountain_2_puresky_4k_diffuse.ktx2"),
            specular_map: asset_server.load("environment_maps/table_mountain_2_puresky_4k_specular.ktx2"),
            intensity: 1.,
        },
        FogSettings {
            color: Color::srgba(0.1, 0.2, 0.4, 1.),
            falloff: FogFalloff::from_visibility_colors(400., Color::srgb(0.35, 0.5, 0.66), Color::srgb(0.8, 0.844, 1.)),
            ..default()
        }
    ));

    #[cfg(feature = "panorbit")]
    cam.insert(PanOrbitCamera {
        focus: Vec3::new(26., WATER_HEIGHT + 5., -11.),
        radius: Some(60.),
        yaw: Some(-std::f32::consts::FRAC_PI_2),
        pitch: Some(0.),
        ..default()
    });

    // Spawn ships
    // let scene = asset_server.load("models/dutch_ship_medium_1k/dutch_ship_medium_1k.gltf#Scene0");
    // let ship = Ship::new(-0.4, -8., 9., -2., 2.);

    // for x in 1..10 {
    //     let f = (x as f32) * 2.4;
    //     let f2 = ((x % 6) as f32) * -20.9;
    // }
}
