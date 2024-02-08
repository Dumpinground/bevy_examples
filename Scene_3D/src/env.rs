use std::env;

#[derive(Default)]
pub enum CameraType {
    #[default]
    Default,
    PanOrbit,
}

#[derive(Default)]
pub struct Envs {
    pub camera: CameraType
}

pub fn read_envs() -> Envs {

    let mut env_vars = Envs::default();

    if dotenvy::dotenv().is_err() {
        println!("local .env file not found");
    }

    if let Ok(camera) = env::var("camera") {
        
        let camera = camera.as_str();
        
        if camera == "pan_orbit" {
            env_vars.camera = CameraType::PanOrbit;
        }
    }

    env_vars
}