use crate::ray::*;
use crate::base::*;

pub struct Camera {
    pub lower_left_corner: Position,
    pub horizontal: Position,
    pub vertical: Position,
    pub origin: Position,
}

impl Camera {
    pub fn new() -> Camera {
        Camera {
            lower_left_corner: Position::new(-2.0, -1.0, -1.0),
            horizontal: Position::new(4.0, 0.0, 0.0),
            vertical: Position::new(0.0, 2.0, 0.0),
            origin: Position::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin)
    }
}
