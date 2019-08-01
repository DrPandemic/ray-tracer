extern crate cgmath;
use cgmath::Vector3;
use std::f64;

pub type Color = Vector3<f64>;
pub type Position = Vector3<f64>;
pub struct Pixel {
    pub color: Color,
    pub position: Position,
}

pub trait Vec3 {
    fn length(&self) -> f64;
    fn unit_vector(&self) -> Vector3<f64>;
}

impl Vec3 for Vector3<f64> {
    fn length(&self) -> f64 {
        (self.x * self.x
            + self.y * self.y
            + self.z * self.z).sqrt()
    }

    fn unit_vector(&self) -> Vector3<f64> {
        self / self.length()
    }
}

pub trait OrElse<T, E> {
    fn map_or_else<U, M: FnOnce(T) -> U, F: FnOnce(E) -> U>(self, fallback: F, map: M) -> U;
}

impl <T, E> OrElse<T, E> for Result<T, E> {
    fn map_or_else<U, M: FnOnce(T) -> U, F: FnOnce(E) -> U>(self, fallback: F, map: M) -> U {
        self.map(map).unwrap_or_else(fallback)
    }
}
