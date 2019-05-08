use std::ops::Add;
use crate::entities::celestial_body::CelestialBody;

#[derive(Debug, PartialEq)]
pub struct Universe {
    pub entities: Vec<CelestialBody>
}

impl Universe {
    pub fn new() -> Universe {
        Universe { entities: Vec::new()}
    }
    fn add(&mut self, body: CelestialBody) {
        self.entities.push(body);
    }
}

#[test]
fn contains_things() {
    let sun = CelestialBody::new(1000);
    let mut universe = Universe::new();

    universe.add(sun);

    assert_eq!(universe.entities, vec![sun]);
}
