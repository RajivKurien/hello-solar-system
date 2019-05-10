pub mod entities;

use nalgebra::base::Vector2;
use crate::entities::celestial_body::*;
use crate::entities::universe::Universe;
use std::time::Duration;
use std::thread;

pub const G: f64 = 6.67e-11f64;
//pub const G: f64 = 1_f64;
//pub const G: f64 = 1_e-2f64;
pub const dT: f64 = 1_e-1f64;

fn main() {
    let sun = CelestialBody::new("Sun", 1_000, Vector2::new(-5_f64, 0_f64));
    let earth = CelestialBody::new("Earth", 1_000, Vector2::new(5_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);
    universe.add(earth);

    let mut i = 0;

    while i < 100_000_00 {
        universe.update();
        i += 1;
    }
}
