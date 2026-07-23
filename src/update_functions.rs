use crate::{components::Planet, internal_imports::*, helpers::*};


pub fn wait_for_start(
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



pub fn gravitational_cycle(
    mut planets: Query<(&mut Planet, &mut Transform)>,
) {
    // snapshot: read-only pass over the same query
    let snapshot: Vec<(Planet, Transform)> = planets
    .iter()
    .map(|(planet, transform)| (planet.clone(), *transform))
    .collect();

    // mutate: exclusive pass over the same query
    for (mut planet, transform) in planets.iter_mut() {
        planet.acting_forces(&transform, &snapshot);
        planet.net_force();
        planet.accelerate();
        }
}

pub fn collision_detection(
    mut planets: Query<(&mut Planet, &mut Transform)>,
) {
    let snapshot: Vec<(Planet, Transform)> = planets
    .iter()
    .map(|(planet, transform)| (planet.clone(), *transform))
    .collect();
    for (mut planet,transform) in planets.iter_mut() {
        planet.adjust_for_collision(&transform, &snapshot);
    }
}



pub fn apply_velocity(
    mut planets: Query<(&mut Planet, &mut Transform)>,
    time: Res<Time>
) {
    for (planet, mut transform) in planets.iter_mut() {
        transform.translation += (planet.velocity.normalize() * (planet.velocity.length() / MWU) * time.delta_secs() as f64).as_vec3();
    }
}