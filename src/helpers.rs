use crate::internal_imports::*;

pub fn com_length_wu_3d(com1: Vec3, com2: Vec3) -> f64 // pixel distance, decimals included, floor it
{
    (com1 - com2).length() as f64
}

pub fn universal_gravity_theory(m1: f64, m2: f64, com1: Vec3, com2: Vec3) -> f64 {
    // self = own mass, m2 = other mass, com1/2 = center of mass both
    let r = com_length_wu_3d(com1, com2) * MWU;
    let r_sq = r.powi(2); //.clamp(1.0, f64::MAX); // r squared as stated for the formula

    G * m1 * m2 / r_sq
}

pub fn recalculate_origin(newtons: f64, own_translation: &Vec3, other_translation: &Vec3) -> DVec3 {
    let translation_unit: DVec3 = (other_translation.as_dvec3() - own_translation.as_dvec3()).normalize();
    translation_unit * newtons
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimState {
    #[default]
    Waiting,

    Running,
}
