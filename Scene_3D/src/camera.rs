use crate::env::{CameraType, ReferenceType};
use bevy::prelude::*;
mod external;
mod movement;
mod my_camera;
mod panorbit;
mod orthographic;
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
                if let ReferenceType::External = envs.reference {
                    app.add_plugins(external::panorbit::CameraPlugin);
                } else {
                    app.add_plugins(panorbit::CameraPlugin);
                }
            }
        }
    }
}
