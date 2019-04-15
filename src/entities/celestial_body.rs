use std::ops::Add;

#[derive(Debug)]
pub struct CelestialBody {
    pub mass: u32,
    pub position: Point,
    pub velocity: Point,
}

impl CelestialBody {
    pub fn new(mass: u32) -> CelestialBody {
        CelestialBody {
            mass,
            position: Point { x: 0, y: 0 },
            velocity: Point { x: 0, y: 0 },
        }
    }

    pub fn update(&mut self){
        self.position = self.position + self.velocity;
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

#[test]
fn celestial_body_has_mass() {
    let planet = CelestialBody::new(10);

    assert_eq!(planet.mass, 10);
}

#[test]
fn celestial_body_has_position() {
    let planet = CelestialBody::new(10);

    assert_eq!(planet.position, Point {
        x: 0,
        y: 0,
    });
}

#[test]
fn celestial_body_with_velocity_moves() {
    let mut planet = CelestialBody {
        mass: 10,
        position: Point { x: 0, y: 0 },
        velocity: Point { x: 1, y: 2 },
    };

    planet.update();

    assert_eq!(planet.position, Point {
        x: 1,
        y: 2,
    });
}

