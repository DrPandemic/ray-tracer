use crate::base::*;

pub struct Ray {
    pub a: Position,
    pub b: Position,
}

impl Ray {
    pub fn new(a: Position, b: Position) -> Ray {
        Ray { a: a, b: b }
    }

    pub fn origin(&self) -> &Position {
        &self.a
    }

    pub fn direction(&self) -> &Position {
        &self.b
    }

    pub fn point_at_parameter(&self, t: f64) -> Position {
        self.a + t * self.b
    }
}
