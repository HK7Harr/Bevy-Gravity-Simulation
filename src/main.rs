use bevy::prelude::*;
use bevy::camera_controller::pan_camera::{PanCamera, PanCameraPlugin};

mod components;


mod update_functions;
use update_functions::*;

mod setup_functions;
use setup_functions::*;

mod helpers;

mod internal_imports;



pub const G: f64 = 1.0; // G, universal gravitational force
pub const MPP: f64 = 100.0; // Meters per pixel
pub const WM: f64 = 10000000.0; // wheight modifier, area of the planet * WM = mass

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
enum SimState {
    #[default]
    Waiting,

    Running,
}



fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(PanCameraPlugin)
        .init_state::<SimState>()
        // Just the camera, so the window isn't blank while you fullscreen it.
        .add_systems(Startup, (setup_camera, spawn_planets))
        .add_systems(Update, wait_for_start)

        .add_systems(Update, gravitational_cycle.run_if(in_state(SimState::Running)))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((Camera2d, PanCamera::default()));
}



fn wait_for_start(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<SimState>>,
    mut next_state: ResMut<NextState<SimState>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        if state.get() == &SimState::Waiting {
            next_state.set(SimState::Running);
        }
        else {
            next_state.set(SimState::Waiting)
        }
    }
}

