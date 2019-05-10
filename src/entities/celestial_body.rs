use nalgebra::Vector2;
use crate::G;
use crate::dT;

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub mass: u64,
    pub position: Vector2<f64>,
    pub velocity: Vector2<f64>,
    pub acceleration: Vector2<f64>,
}

impl CelestialBody {
    pub fn new(name: &str, mass: u64, position: Vector2<f64>) -> CelestialBody {
        CelestialBody {
            name: name.to_string(),
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
        self.acceleration = acceleration;

        if self.position[0] > 10_f64 {
            panic!();
        }
    }

    pub fn field_at(&self, position: &Vector2<f64>) -> Vector2<f64> {
        let mut direction = position - self.position;
        let magnitude_squared = direction.dot(&direction);
        direction = direction.normalize();

        -G * (self.mass as f64) * direction / (magnitude_squared)
    }
}

impl PartialEq for CelestialBody{
    fn eq(&self, other: &CelestialBody) -> bool {
        self.name == other.name
    }
}

#[test]
fn has_mass() {
    let position = Vector2::new(0_f64, 0_f64);
    let planet = CelestialBody::new("Sun", 10, position);

    assert_eq!(planet.mass, 10);
}

#[test]
fn has_position() {
    let position = Vector2::new(1_f64, 1_f64);
    let planet = CelestialBody::new("Sun", 10, position);

    assert_eq!(planet.position, position);
}

#[test]
fn has_acceleration() {
    let position = Vector2::new(1_f64, 1_f64);
    let planet = CelestialBody::new("Sun", 10, position);

    assert_eq!(planet.acceleration, Vector2::new(0_f64, 0_f64));
}

#[test]
fn with_velocity_moves() {
    let initial_velocity = Vector2::new(1_f64, 2_f64);
    let mut planet = CelestialBody {
        name: String::new(),
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: initial_velocity,
        acceleration: Vector2::new(0_f64, 0_f64),
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.position, initial_velocity * dT);
}

#[test]
fn with_acceleration_changes_velocity() {
    let initial_acceleration = Vector2::new(1_f64, 2_f64);
    let mut planet = CelestialBody {
        name: String::new(),
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: initial_acceleration,
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.velocity, 0.5 * initial_acceleration * dT);
}

#[test]
fn with_new_acceleration_changes_velocity() {
    let mut planet = CelestialBody {
        name: String::new(),
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: Vector2::new(0_f64, 0_f64),
    };
    let new_acceleration = Vector2::new(1_f64, 2_f64);

    planet.update(new_acceleration);

    assert_eq!(planet.velocity, 0.5 * Vector2::new(1_f64, 2_f64) * dT);
}

#[test]
fn with_acceleration_changes_position() {
    let initial_acceleration = Vector2::new(1_f64, 2_f64);
    let mut planet = CelestialBody {
        name: String::new(),
        mass: 10,
        position: Vector2::new(0_f64, 0_f64),
        velocity: Vector2::new(0_f64, 0_f64),
        acceleration: initial_acceleration,
    };

    planet.update(Vector2::new(0_f64, 0_f64));

    assert_eq!(planet.position, 0.5 * initial_acceleration * dT * dT);
}

#[test]
fn produces_acceleration() {
    let mass = 1;
    let sun = CelestialBody::new("Sun", mass, Vector2::new(0_f64, 0_f64));
    let position = Vector2::new(10_f64, 0_f64);
    let expected = (-G * (mass as f64) / (10_f64 * 10_f64)) * position.normalize();

    assert_eq!(sun.field_at(&position), expected);
}

#[test]
fn equals_another_with_the_same_name(){
    let sun = CelestialBody::new("Sun", 1, Vector2::new(0_f64, 0_f64));
    let another_sun = CelestialBody::new("Sun", 10, Vector2::new(0_f64, 0_f64));

    assert_eq!(sun, another_sun);
}

#[test]
fn does_not_equal_another_with_different_name(){
    let sun = CelestialBody::new("Sun", 1, Vector2::new(0_f64, 0_f64));
    let sirius = CelestialBody::new("Sirius", 10, Vector2::new(0_f64, 0_f64));

    assert_ne!(sun, sirius);
}

