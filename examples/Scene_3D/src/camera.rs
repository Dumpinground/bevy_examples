use crate::env::{CameraType, ReferenceType};
use bevy::prelude::*;
// mod external;
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

        match (envs.camera, envs.reference) {
            (CameraType::Orthographic, ReferenceType::Internal) => {
                app.add_plugins((OrthographicCameraPlugin, MovablePlugin));
            }
            (CameraType::PanOrbit, ReferenceType::Internal) => {
                app.add_plugins(panorbit::CameraPlugin);
            }
            // wait for fix
            // (CameraType::PanOrbit, ReferenceType::External) => {
            //     app.add_plugins(external::panorbit::CameraPlugin);
            // }
            (_, _) => {
                app.add_plugins((MyCameraPlugin, MovablePlugin));
            }
        }
    }
}
