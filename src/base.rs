extern crate cgmath;
use cgmath::Vector3;
use std::f64;
use rand::{thread_rng, Rng};
use std::iter;

pub type Color = Vector3<f64>;
pub type Position = Vector3<f64>;
pub struct Pixel {
    pub color: Color,
    pub position: Position,
}

pub trait Vec3 {
    fn length(&self) -> f64;
    fn squared_length(&self) -> f64;
    fn unit_vector(&self) -> Vector3<f64>;
    fn mul(&self, rhs: &Vector3<f64>) -> Vector3<f64>;
}

impl Vec3 for Vector3<f64> {
    fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }

    fn squared_length(&self) -> f64 {
        self.x * self.x
            + self.y * self.y
            + self.z * self.z
    }

    fn unit_vector(&self) -> Vector3<f64> {
        self / self.length()
    }

    fn mul(&self, rhs: &Vector3<f64>) -> Self {
        Vector3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
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

pub fn random() -> f64 {
    thread_rng().gen_range(0.0f64, 1.0f64)
}

pub fn random_in_unit_sphere() -> Position {
    iter::repeat(None).find_map(|_: Option<Position>| {
        // TODO vec - vec is broken
        let p = 2.0 * Position::new(random(), random(), random()) - Position::new(1.0, 1.0, 1.0);
        if p.squared_length() < 1.0 {
            Some(p)
        } else {
            None
        }
    }).unwrap()
}
