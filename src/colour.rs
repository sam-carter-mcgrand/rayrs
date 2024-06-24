use crate::{
    common,
    vec3::{self, Vec3},
};
use std::io::Write;

pub type Colour = Vec3;

pub fn write_colour(out: &mut impl Write, pixel_colour: Colour, samples_per_pixel: i32) {
    // Divide colour by number of samples
    let scale = 1.0 / samples_per_pixel as f64;

    // Gamma correction for gamma=2
    let r = f64::sqrt(pixel_colour.x() * scale);
    let g = f64::sqrt(pixel_colour.y() * scale);
    let b = f64::sqrt(pixel_colour.z() * scale);

    let r_clamped = (256.0 * common::clamp(r, 0.0, 0.999)) as i32;
    let g_clamped = (256.0 * common::clamp(g, 0.0, 0.999)) as i32;
    let b_clamped = (256.0 * common::clamp(b, 0.0, 0.999)) as i32;

    writeln!(out, "{} {} {}", r_clamped, g_clamped, b_clamped).expect("Writing colour");
}

pub fn random() -> Colour {
    vec3::random_in_unit_sphere()
}
