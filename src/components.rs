use crate::internal_imports::*;
use crate::helpers::*;

#[derive(Component, Default, Clone)]
pub struct Planet {
    pub id: i32,
    pub velocity: DVec3,
    pub net_force: DVec3, // x, y, (z = 0.0) 
    pub acting_forces: Vec<DVec3>,
    pub radius: i32, // radius in pixels
    pub mass: f64,
}
impl Planet {
    pub fn new(id: i32, radius: i32, starting_velocity: DVec3) -> Self {
        Self {
            id: id,
            velocity: starting_velocity, 
            net_force: DVec3::ZERO,
            acting_forces: Vec::new(),
            radius: radius,
            mass: h_pi() * radius.pow(2) as f64 * WM,
        }
    }


    pub fn acting_forces(&mut self, own_transform: &Mut<Transform>, planet_list: &Vec<(Planet, Transform)>) { // list of all planets
        for planet in planet_list {
            if planet.0.id != self.id {
                self.acting_forces.push(recalculate_origin( 
                    universal_gravity_theory(self.mass, planet.0.mass,own_transform.translation, planet.1.translation), 
                    &own_transform.translation, 
                    &planet.1.translation, 
                ));
            }
        }
    }

    pub fn net_force(&mut self) {
        let mut net_force = DVec3::ZERO;

        for force in &self.acting_forces {
            let magnitude = (force.y * force.y + force.z * force.z).sqrt();

            // newtons * (x / magnitude)
            net_force.x += force[0] * (force[1] / magnitude);
            net_force.y += force[0] * (force[2] / magnitude);
        }
        self.net_force = net_force;
        self.acting_forces.clear();
    }

    pub fn accelerate(&mut self) {
        // a = m/F, F in this case is the magnitude of the net force vector
        self.velocity += self.net_force/self.mass;
    }

    pub fn adjust_for_collision(&mut self, own_transform: &Mut<Transform>, all_planets: &Vec<(Planet, Transform)>) {
        for planet in all_planets {
            if planet.0.id != self.id {
                if com_length_pixels_2d(own_transform.translation, planet.1.translation) <= self.radius as f64 + planet.0.radius as f64 {
                    let mass_scalar_ratio = self.velocity - (2.0 * planet.0.mass)/(self.mass + planet.0.mass);
                    let velocity_difference = self.velocity - planet.0.velocity;
                    let transform_difference = own_transform.translation.as_dvec3();

                    let dot_product = velocity_difference.dot(transform_difference);
                    if dot_product >= 0.0 {
                        return; 
                    }
                    let distance_squared = transform_difference.length_squared().max(0.01);
                    self.velocity = self.velocity - (mass_scalar_ratio * (dot_product / distance_squared) * transform_difference);
                    println!("{}", self.velocity.length());
                }
            }
        }
    }
}
