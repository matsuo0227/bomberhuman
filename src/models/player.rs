use rand::Rng;

use super::Vector;
use crate::geometry::{Advance, Collide, Position};
use crate::geometry::{Point, Size};

/// The `Player` is the rocket controlled by the user
#[derive(Default)]
pub struct Player {
    pub vector: Vector,
    pub speed: f64,
}

derive_position_direction!(Player);

/// The player is represented as the polygon below
pub const POLYGON: &'static [[f64; 2]] = &[[0.0, -8.0], [20.0, 0.0], [0.0, 8.0]];

impl Player {
    /// Create a new `Player` with a random position and direction
    pub fn random<R: Rng>(rng: &mut R, bounds: Size) -> Player {
        Player {
            vector: Vector::random(rng, bounds),
            speed: 1000.0,
        }
    }

    /// Returns the front of the rocket
    pub fn front(&self) -> Point {
        Point::new(POLYGON[1][0], POLYGON[1][1])
            .rotate(self.direction())
            .translate(&self.position())
    }
}

impl Collide for Player {
    fn radius(&self) -> f64 {
        6.0
    }
}
