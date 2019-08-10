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
use std::cell::RefCell;
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
fn color(ray: &Ray, world: &Box<Hitable>, depth: u8) -> Color {
    world.hit(&ray, 0.001, f64::MAX).map_or_else(|_| {
        let unit_direction = ray.direction().unit_vector();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
    }, |record| {
        if depth > 50 {
            Color::new(0.0, 0.0, 0.0)
        } else {
            record.material.scatter(&ray, record.clone())
                .map_or_else(
                    || Color::new(0.0, 0.0, 0.0),
                    |(ref attenuation, ref scattered)| attenuation.mul(&color(&scattered, world, depth + 1)))
        }
    })
}

thread_local!(static NEXT_X: RefCell<u16> = RefCell::new(0));
thread_local!(static NEXT_Y: RefCell<u16> = RefCell::new(0));
thread_local!(static WORLD: RefCell<Box<dyn Hitable>> = RefCell::new(random_scene()));

#[wasm_bindgen]
pub fn run() -> bool {
    let nx = 200;
    let ny = 100;
    let ns = 4;
    let context = get_context();

    let lookfrom = Vec3::new(13.0, 2.0, 4.0);
    let lookat = Vec3::new(-2.0, 1.0, -1.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        f64::from(nx) / f64::from(ny),
        0.2,
        (lookfrom - lookat).length()
    );

    NEXT_X.with(|x| {
        NEXT_Y.with(|y| {
            let mut current_x = x.borrow_mut();
            let mut current_y = y.borrow_mut();
            let x = f64::from(current_x.clone());
            let y = f64::from(current_y.clone());
            WORLD.with(|w| {
                let world = w.borrow();

                let mut col = Color::new(0.0, 0.0, 0.0);

                for _ in 0..ns {
                    let u = (x + random()) / f64::from(nx);
                    let v = (y + random()) / f64::from(ny);
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
                        position: Vec3::new(x, f64::from(ny) - y, 0.0),
                        color: col
                    }
                );

                let next_x = (current_x.clone() + 1) % nx;
                let next_y = if next_x == 0 {
                    (current_y.clone() + 1) % ny
                } else {
                    current_y.clone()
                };
                *current_x = next_x;
                *current_y = next_y;

                return next_x == 0 && next_y == 0
            });
        });
    });
    false
}

fn random_scene() -> Box<dyn Hitable> {
    let mut list = Vec::new();
    list.push(Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0, Rc::new(Lambertian::new(&Color::new(0.5, 0.5, 0.5))))) as Box<Hitable>);
    list.push(Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0, Rc::new(Dialectric::new(1.5)))) as Box<Hitable>);
    list.push(Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0, Rc::new(Lambertian::new(&Color::new(0.4, 0.2, 0.1))))) as Box<Hitable>);
    list.push(Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0, Rc::new(Metal::new(&Color::new(0.5, 0.5, 0.5), 0.0)))) as Box<Hitable>);

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Vec3::new(f64::from(a) + 0.9 * random(), 0.2, f64::from(b) + 0.9 * random());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 { // diffuse
                    list.push(Box::new(Sphere::new(center, 0.2, Rc::new(Lambertian::new(&Color::new(random() * random(), random() * random(), random() * random()))))) as Box<Hitable>);
                }
            } else if choose_mat < 0.85 { // metal
                list.push(Box::new(Sphere::new(center, 0.2, Rc::new(Metal::new(&Color::new(0.5 * (1.0 + random()), 0.5 * (1.0 + random()), 0.5 * (1.0 + random())), 0.5 * random())))) as Box<Hitable>);
            } else { // glass
                list.push(Box::new(Sphere::new(center, 0.2, Rc::new(Dialectric::new(1.5)))) as Box<Hitable>);
            }
        }
    }


    Box::new(HitableList::new(list))
}
