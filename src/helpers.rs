use crate::internal_imports::*;

pub fn com_length_pixels_2d(com1: Vec3, com2: Vec3) -> f64 // pixel distance, decimals included, floor it
{
    let x  = com1[0] - com2[0]; // difference of x coordinates of the two planets centers of mass
    let y = com1[1] - com2[1]; // difference of y coordinates of the two planets centers of mass
    h_pythagorean_theorem(x, y)
}

pub fn com_length_meters_2d(com1: Vec3, com2: Vec3) -> f64 // meter distance
{
    let x  = com1[0] - com2[0]; // difference of x coordinates of the two planets centers of mass
    let y = com1[1] - com2[1]; // difference of y coordinates of the two planets centers of mass
    h_pythagorean_theorem(x, y) * MPP
}

pub fn universal_gravity_theory(m1: f64, m2: f64, com1: Vec3, com2: Vec3) -> f64 {
    // self = own mass, m2 = other mass, com1/2 = center of mass both
    let r = com_length_meters_2d(com1, com2);
    let r_sq = r.powi(2).clamp(1.0, f64::MAX); // r squared as stated for the formula

    G * m1 * m2 / r_sq
}

pub fn recalculate_origin(newtons: f64, own_translation: &Vec3, other_translation: &Vec3) -> DVec3 {
    let translation_vec: DVec3 = other_translation.as_dvec3() - own_translation.as_dvec3();
    DVec3 { x: newtons, y: translation_vec[0], z: translation_vec[1]}
}