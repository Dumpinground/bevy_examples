use bevy::{input::mouse::{MouseMotion, MouseWheel}, prelude::*};

#[derive(Component)]
struct PanOrbitCamera {
    pub focus: Vec3,
    pub radius: f32,
    pub upside_down: bool,
}

impl Default for PanOrbitCamera {
    fn default() -> Self {
        PanOrbitCamera {
            focus: Vec3::ZERO,
            radius: 5.,
            upside_down: false,
        }
    }
}

fn setup(mut commands: Commands) {
    let translation = Vec3::new(-2., 2.5, 5.);
    let radius = translation.length();

    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_translation(translation).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        PanOrbitCamera {
            radius,
            ..default()
        }
    ));
}

fn pan_orbit_camera(mut ev_motion: EventReader<MouseMotion>, mut ev_scroll: EventReader<MouseWheel>, input_mouse: Res<ButtonInput<MouseButton>>, mut query: Query<(&mut PanOrbitCamera, &mut Transform, &Projection)>) {
    let orbit_button = MouseButton::Right;
    let pan_button = MouseButton::Middle;

    let mut pan = Vec2::ZERO;
    let mut rotation_move = Vec2::ZERO;
    let mut scroll = 0.;
    let mut orbit_button_changed = false;

    if input_mouse.pressed(orbit_button) {
        
    }
}