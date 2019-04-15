use std::ops::Add;
use crate::entities::celestial_body::CelestialBody;

pub struct Universe {
    pub entities: Vec<CelestialBody>
}

impl Universe {
    pub fn new() -> Universe {
        Universe { entities: Vec::new()}
    }
}

//#[test]
//fn universe_contains_things() {
//    let sun = CelestialBody::new(1000);
//    let universe = Universe::new();
//
//    universe.add(sun);
//
////    assert_eq!(universe.entities, vec![sun]);
//}
