use crate::{components::Planet, internal_imports::*};

pub fn gravitational_cycle(
    mut planets: Query<(&mut Planet, &mut Transform)>,
    time: Res<Time>,
) {
    // snapshot: read-only pass over the same query
    let snapshot: Vec<(Planet, Transform)> = planets
    .iter()
    .map(|(planet, transform)| (planet.clone(), *transform))
    .collect();

    // mutate: exclusive pass over the same query
    for (mut planet, mut transform) in planets.iter_mut() {
        planet.acting_forces(&transform, &snapshot);
        planet.net_force();
        planet.accelerate();

        transform.translation += (planet.velocity.normalize_or_zero() * (planet.velocity.length() / MPP) * time.delta_secs() as f64).as_vec3();
        println!("{}", planet.velocity);
    }
}

pub fn collision_detection(
    mut planets: Query<(&mut Planet, &mut Transform)>,
    time: Res<Time>,
) {
    let snapshot: Vec<(Planet, Transform)> = planets
    .iter()
    .map(|(planet, transform)| (planet.clone(), *transform))
    .collect();

}