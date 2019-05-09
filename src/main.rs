pub mod entities;
use nalgebra::base::Vector2;
use crate::entities::celestial_body::*;

pub const G: f64 = 6.67e-11f64;
pub const dT: f64 = 1_f64;

fn main() {
    println!("Hello, world!");
    println!("Gravitational constant {}", G);

    let a = CelestialBody::new(0, Vector2::new(0_f64, 0_f64));
}
