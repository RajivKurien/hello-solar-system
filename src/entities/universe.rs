use crate::entities::celestial_body::CelestialBody;
use nalgebra::base::Vector2;
use std::f64::consts::SQRT_2;

#[derive(Debug, PartialEq)]
pub struct Universe {
    pub entities: Vec<CelestialBody>
}

impl Universe {
    pub fn new() -> Universe {
        Universe { entities: Vec::new() }
    }
    fn add(&mut self, body: CelestialBody) {
        self.entities.push(body);
    }
    fn field_at(self, position: Vector2<f64>) -> Vector2<f64> {
        let mut field = Vector2::new(0_f64, 0_f64);

        for entity in self.entities {
            field += entity.field_at(position);
        }

        field
    }
}

#[test]
fn contains_things() {
    let sun = CelestialBody::new(1000, Vector2::new(0_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);

    assert_eq!(universe.entities, vec![sun]);
}

#[test]
fn computes_aggregate_field() {
    let sun = CelestialBody::new(1000, Vector2::new(-1_f64, 0_f64));
    let earth = CelestialBody::new(1000, Vector2::new(1_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);
    universe.add(earth);
    let position = Vector2::new(0_f64, 0_f64);

    let acc = universe.field_at(position);

    assert_eq!(acc, Vector2::new(0_f64, 0_f64));
}
