use std::env;

#[derive(Default)]
pub enum CameraType {
    #[default]
    Default,
    PanOrbit,
    Orthographic,
}

#[derive(Default)]
pub enum ReferenceType {
    #[default]
    Internal,
    External,
}

#[derive(Default)]
pub struct Envs {
    pub camera: CameraType,
    pub reference: ReferenceType,
}

pub fn read_envs() -> Envs {
    let mut env_vars = Envs::default();

    if dotenvy::dotenv().is_err() {
        println!("local .env file not found");
    }

    if let Ok(camera) = env::var("camera") {
        let camera = camera.as_str();

        env_vars.camera = match camera {
            "pan_orbit" => CameraType::PanOrbit,
            "orthographic" => CameraType::Orthographic,
            _ => CameraType::Default,
        }
    }

    if let Ok(reference) = env::var("reference") {
        let reference = reference.as_str();

        env_vars.reference = match reference {
            "external" => ReferenceType::External,
            _ => ReferenceType::Internal,
        }
    }

    env_vars
}
