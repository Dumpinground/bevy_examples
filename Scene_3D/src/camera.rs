use crate::env::CameraType;
use bevy::prelude::*;
mod movement;
mod my_camera;
mod orthographic;
// mod panorbit;
// use panorbit::ExternalPanOrbitCameraPlugin;
use my_camera::MyCameraPlugin;
use orthographic::OrthographicCameraPlugin;

use self::movement::MovablePlugin;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        let envs = crate::env::read_envs();

        match envs.camera {
            CameraType::Default => {
                app.add_plugins((MyCameraPlugin, MovablePlugin));
            }
            CameraType::Orthographic => {
                app.add_plugins((OrthographicCameraPlugin, MovablePlugin));
            }
            CameraType::PanOrbit => {
                println!("this need fix");
                // app.add_plugins(ExternalPanOrbitCameraPlugin);
            }
        }
    }
}
