pub struct CelestialBody {
    pub mass: u32
}


#[test]
fn celestial_body_has_mass() {
    let planet = CelestialBody { mass: 10 };

    assert_eq!(planet.mass, 10);
}
