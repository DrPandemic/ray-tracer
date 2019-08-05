use crate::base::*;
use crate::material::*;
use crate::ray::*;
use cgmath::dot;
use std::vec::Vec;
use std::rc::Rc;

#[derive(Clone)]
pub struct HitRecord {
    pub t: f64,
    pub p: Position,
    pub normal: Position,
    pub material: Rc<Material>,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord {
            t: 0.0,
            p: Position::new(0.0, 0.0, 0.0),
            normal: Position::new(0.0, 0.0, 0.0),
            material: Rc::new(Lambertian::new(&Color::new(0.3, 0.3, 0.3))),
        }
    }

    pub fn new(t: f64, p: Position, normal: Position, material: Rc<Material>) -> HitRecord {
        HitRecord { t: t, p: p, normal: normal, material: material }
    }

    pub fn clone_into(&mut self, other: &HitRecord) {
        self.t = other.t;
        self.p = other.p;
        self.normal = other.normal;
        self.material = Rc::clone(&other.material);
    }

    pub fn clone(&self) -> HitRecord {
        HitRecord {
            t: self.t,
            p: self.p,
            normal: self.normal,
            material: Rc::clone(&self.material),
        }
    }
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Result<HitRecord, ()>;
}

#[derive(Clone)]
pub struct Sphere {
    pub center: Position,
    pub radius: f64,
    pub material: Rc<Material>,
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            center: Position::new(0.0, 0.0, 0.0),
            radius: 1.0,
            material: Rc::new(Lambertian::new(&Color::new(0.8, 0.3, 0.3))),
        }
    }

    pub fn new(center: Position, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
            material: material,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Result<HitRecord, ()> {
        let oc = ray.origin() - self.center;
        let a = dot(*ray.direction(), *ray.direction());
        let b = 2.0 * dot(oc, *ray.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let tmp = (-b - discriminant.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                let p = ray.point_at_parameter(tmp);
                return Ok(
                    HitRecord::new(
                        tmp,
                        p,
                        (p - self.center) / self.radius,
                        Rc::clone(&self.material),
                    )
                )
            }
        }

        Err(())
    }
}

pub struct HitableList {
    pub list: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new(list: Vec<Box<dyn Hitable>>) -> HitableList {
        HitableList {
            list: list
        }
    }
}

impl Hitable for HitableList {
    #[allow(unstable_name_collisions)]
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Result<HitRecord, ()> {
        self.list.iter().fold(
            (t_max, Err(())),
            |(closest_so_far, previous), item|
                item.hit(&ray, t_min, closest_so_far).map_or_else(
                    |_| (closest_so_far, previous),
                    |record| (record.t, Ok(record))
                )
        ).1
    }
}
