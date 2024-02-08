use bevy::prelude::*;

mod my_camera;
mod panrobit;

use my_camera::MyCameraPlugin;
use panrobit::MyPanOrbitCameraPlugin;

use crate::env::CameraType;
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        
        let envs = crate::env::read_envs();

        match envs.camera {
            CameraType::Default => {
                app.add_plugins(MyCameraPlugin);
            }
            CameraType::PanOrbit => {
                app.add_plugins(MyPanOrbitCameraPlugin);
            }
        }
    }
}