use bevy::prelude::*;
use bevy_panorbit_camera::*;

mod components;


mod update_functions;
use update_functions::*;

mod setup_functions;
use setup_functions::*;

mod helpers;
use helpers::*;

mod internal_imports;



pub const G: f64 = 10.0; // G, universal gravitational force
pub const MWU: f64 = 100.0; // Meters world unit
pub const WM: f64 = 1000000.0; // wheight modifier, area of the planet * WM = mass
pub const EC: f64 = 1.0; // ellastic coefficient, 0 - 1
pub const B: f32 = 0.1; // brightness of the light

pub const MIN_WS: i32 = -100000; // minimum world scale
pub const MAX_WS: i32 = 100000; // maximum world scale

pub const MIN_R: i32 = 80; // min radius
pub const MAX_R: i32 = 2500; // max radius




fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanOrbitCameraPlugin)
        .init_state::<SimState>()
        // Just the camera, so the window isn't blank while you fullscreen it.
        .add_systems(Startup, (setup_camera, spawn_planets, setup_lighting))
        .add_systems(Update, wait_for_start)

        .add_systems(Update, (gravitational_cycle, collision_detection, apply_velocity).run_if(in_state(SimState::Running)))
        .run();
}




