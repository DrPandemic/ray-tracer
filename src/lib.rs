extern crate wasm_bindgen;
extern crate cgmath;

pub mod canvas;
pub mod base;
pub mod ray;
pub mod hitable;

use wasm_bindgen::prelude::*;
use cgmath::dot;
use std::f64;
use crate::canvas::*;
use crate::base::*;
use crate::ray::*;
use crate::hitable::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(a: f64);
}

// t*t*dot(B, B) + 2*t*dot(B,A-C) + dot(A-C,A-C) - R*R = 0
fn hit_sphere(center: &Position, radius: f64, ray: &Ray) -> f64 {
    let oc = ray.origin() - center;
    let a = dot(*ray.direction(), *ray.direction());
    let b = 2.0 * dot(oc, *ray.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}


#[allow(unstable_name_collisions)]
fn color(ray: &Ray, world: &Hitable) -> Color {
    world.hit(&ray, 0.0, f64::MAX).map_or_else(|_| {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }, |record| {
        0.5 * Color::new(record.normal.x + 1.0, record.normal.y + 1.0, record.normal.z + 1.0)
    })
}

#[wasm_bindgen]
pub fn main() {
    let context = get_context();
    let nx = 200;
    let ny = 100;

    let lower_left_corner = Position::new(-2.0, -1.0, -1.0);
    let horizontal = Position::new(4.0, 0.0, 0.0);
    let vertical = Position::new(0.0, 2.0, 0.0);
    let origin = Position::new(0.0, 0.0, 0.0);

    let list = vec![
        Box::new(Sphere::new(Position::new(0.0, 0.0, -1.0), 0.5)) as Box<Hitable>,
        Box::new(Sphere::new(Position::new(0.0, -100.5, -1.0), 100.0)) as Box<Hitable>,
    ];

    let world = HitableList::new(list);

    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = f64::from(i) / f64::from(nx);
            let v = f64::from(j) / f64::from(ny);
            let ray = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            draw_pixel(
                &context,
                Pixel {
                    position: Position::new(f64::from(i), f64::from(ny - j), 0.0),
                    color: color(&ray, &world)
                }
            );
        }
    }
    // alert(&format!("Hello, {:?}!", Vector3::new(1.0, 2.0, 3.0)));
}
