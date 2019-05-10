use crate::entities::celestial_body::CelestialBody;
use nalgebra::base::Vector2;

#[derive(Debug, PartialEq)]
pub struct Universe {
    pub entities: Vec<CelestialBody>,
    pub counter: i32,
}

impl Universe {
    pub fn new() -> Universe {
        Universe { entities: Vec::new(), counter: 0 }
    }
    pub fn add(&mut self, body: CelestialBody) {
        self.entities.push(body);
    }
    fn field_at(&self, entity: &CelestialBody) -> Vector2<f64> {
        let mut field = Vector2::new(0_f64, 0_f64);

        for e in &self.entities {
            if entity != e {
                field += e.field_at(&entity.position);
            }
        }

        field
    }
    pub fn update(&mut self) {
        let mut accelerations = Vec::new();
        for entity in &self.entities {
            accelerations.push(self.field_at(&entity));
        }

        let mut i = 0;
        for entity in &mut self.entities {
            entity.update(accelerations[i]);
            i += 1;
            if entity.name == "Earth" && self.counter > 100 {
                println!("{}, {}, {}", entity.position[0], entity.velocity[0], entity.acceleration[0]);
                self.counter = 0;
            }
        }
        self.counter += 1;
    }
}

#[test]
fn contains_things() {
    let sun = CelestialBody::new("", 1000, Vector2::new(0_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);

    assert_eq!(universe.entities, vec![CelestialBody::new("", 1000, Vector2::new(0_f64, 0_f64))]);
}

#[test]
fn computes_aggregate_field() {
    let sun = CelestialBody::new("", 1000, Vector2::new(-1_f64, 0_f64));
    let earth = CelestialBody::new("", 1000, Vector2::new(1_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);
    universe.add(earth);
    let position = Vector2::new(0_f64, 0_f64);

    let acc = universe.field_at(&CelestialBody::new("", 1000, position));

    assert_eq!(acc, Vector2::new(0_f64, 0_f64));
}

#[test]
fn updates_position_of_entities() {
    let sun = CelestialBody::new("Sun", 1000, Vector2::new(-1_f64, 0_f64));
    let earth = CelestialBody::new("Earth", 1000, Vector2::new(1_f64, 0_f64));
    let mut universe = Universe::new();

    universe.add(sun);
    universe.add(earth);

    universe.update(); // update velocities with acc
    universe.update(); // update positions with acc

    assert_ne!(universe.entities[1].position, Vector2::new(1_f64, 0_f64));
}
