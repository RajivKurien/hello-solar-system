use nalgebra::Vector2;
use crate::G;
use crate::dT;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct CelestialBody {
    pub mass: u32,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
}

impl CelestialBody {
    pub fn new(mass: u32, position: Vector2<f64>) -> CelestialBody {
        CelestialBody {
            mass,
            position,
            velocity: Vector2::new(0_f64, 0_f64),
            acceleration: Vector2::new(0_f64, 0_f64),
        }
    }

    /// Using Verlet integration
    ///
    pub fn update(&mut self, acceleration: Vector2<f64>) {
        self.position = self.position + self.velocity * dT + 0.5 * self.acceleration * dT * dT;
        self.velocity = self.velocity + 0.5 * (self.acceleration + acceleration) * dT;
    }

    pub fn field_at(self, position: Vector2<f64>) -> Vector2<f64> {
        -G * (self.mass as f64) * (position - self.position)
    }
}

#[test]
fn has_mass() {
    let position = Vector2::new(0_f64, 0_f64);
    let planet = CelestialBody::new(10, position);

    assert_eq!(planet.mass, 10);
}

#[test]
fn has_position() {
    let position = Vector2::new(1_f64, 1_f64);
    let planet = CelestialBody::new(10, position);

    assert_eq!(planet.position, position);
}

#[test]
fn has_acceleration() {
    let position = Vector2::new(1_f64, 1_f64);
    let planet = CelestialBody::new(10, position);

    assert_eq!(planet.acceleration, Vector2::new(0_f64, 0_f64));
}

#[test]
fn with_velocity_moves() {
    let mut planet = CelestialBody {
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(1_f64, 2_f64),
        acceleration: Vector2::new(0_f64, 0_f64),
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.position, Vector2::new(1_f64, 2_f64));
}

#[test]
fn with_acceleration_changes_velocity() {
    let mut planet = CelestialBody {
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: Vector2::new(1_f64, 2_f64),
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.velocity, Vector2::new(0.5_f64, 1_f64));
}

#[test]
fn with_new_acceleration_changes_velocity() {
    let mut planet = CelestialBody {
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: Vector2::new(0_f64, 0_f64),
    };

    planet.update(Vector2::new(1_f64, 2_f64));

    assert_eq!(planet.velocity, Vector2::new(0.5_f64, 1_f64));
}

#[test]
fn with_acceleration_changes_position() {
    let mut planet = CelestialBody {
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: Vector2::new(1_f64, 2_f64),
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.position, Vector2::new(0.5_f64, 1_f64));
}

#[test]
fn produces_acceleration() {
    let mass = 1;
    let planet = CelestialBody::new(mass, Vector2::new(0_f64, 0_f64));
    let position = Vector2::new(10_f64, 0_f64);

    assert_eq!(planet.field_at(position), -G * position * (mass as f64));
}

