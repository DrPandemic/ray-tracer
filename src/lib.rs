extern crate wasm_bindgen;
extern crate cgmath;
extern crate rand;

pub mod canvas;
pub mod base;
pub mod ray;
pub mod hitable;
pub mod camera;
pub mod material;

use wasm_bindgen::prelude::*;
use std::f64;
use std::rc::Rc;
use crate::canvas::*;
use crate::base::*;
use crate::ray::*;
use crate::hitable::*;
use crate::camera::*;
use crate::material::*;

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_f64(a: f64);
}

#[allow(unstable_name_collisions)]
fn color(ray: &Ray, world: &Hitable, depth: u8) -> Color {
    world.hit(&ray, 0.001, f64::MAX).map_or_else(|_| {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }, |record| {
        if depth > 50 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            let (
                got_scaterred,
                attenuation,
                scattered,
            ) = record.material.scatter(&ray, record.clone());

            if !got_scaterred {
                Color::new(0.0, 0.0, 0.0)
            } else {
                attenuation.mul(&color(&scattered, world, depth + 1))
            }
        }
    })
}

#[wasm_bindgen]
pub fn main() {
    let context = get_context();
    let nx = 200;
    let ny = 100;
    let ns = 100;

    let list = vec![
        Box::new(Sphere::new(Position::new(0.0, 0.0, -1.0), 0.5, Rc::new(Lambertian::new(&Color::new(0.8, 0.3, 0.3))))) as Box<Hitable>,
        Box::new(Sphere::new(Position::new(0.0, -100.5, -1.0), 100.0, Rc::new(Lambertian::new(&Color::new(0.3, 0.8, 0.3))))) as Box<Hitable>,
        Box::new(Sphere::new(Position::new(1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0)))) as Box<Hitable>,
        Box::new(Sphere::new(Position::new(-1.0, 0.0, -1.0), 0.5, Rc::new(Metal::new(&Color::new(0.8, 0.8, 0.8), 0.2)))) as Box<Hitable>,
    ];

    let world = HitableList::new(list);
    let camera = Camera::new();

    for j in (0..ny).rev() {
        for i in 0..nx {
            let mut col = Color::new(0.0, 0.0, 0.0);

            for _ in 0..ns {
                let u = (f64::from(i) + random()) / f64::from(nx);
                let v = (f64::from(j) + random()) / f64::from(ny);
                let ray = camera.get_ray(u, v);
                col += color(&ray, &world, 0);
            }

            col /= f64::from(ns);
            col.x = col.x.sqrt();
            col.y = col.y.sqrt();
            col.z = col.z.sqrt();

            draw_pixel(
                &context,
                Pixel {
                    position: Position::new(f64::from(i), f64::from(ny - j), 0.0),
                    color: col
                }
            );
        }
    }
}
