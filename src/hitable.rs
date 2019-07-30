use crate::base::*;
use crate::ray::*;
use cgmath::dot;
use std::vec::Vec;

pub struct HitRecord {
    pub t: f64,
    pub p: Position,
    pub normal: Position,
}

pub trait Hitable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool;
}

pub struct Sphere {
    pub center: Position,
    pub radius: f64,
}

impl Sphere {
    pub fn default() -> Sphere {
        Sphere {
            center: Position::new(0.0, 0.0, 0.0),
            radius: 1.0,
        }
    }

    pub fn new(center: Position, radius: f64) -> Sphere {
        Sphere {
            center: center,
            radius: radius,
        }
    }
}

impl Hitable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let oc = ray.origin() - self.center;
        let a = dot(*ray.direction(), *ray.direction());
        let b = 2.0 * dot(oc, *ray.direction());
        let c = dot(oc, oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant > 0.0 {
            let tmp = (-b - discriminant.sqrt()) / (2.0 * a);
            if tmp < t_max && tmp > t_min {
                record.t = tmp;
                record.p = ray.point_at_parameter(tmp);
                record.normal = (record.p - self.center) / self.radius;
                return true;
            }
        }

        false
    }
}

pub struct HitableList {
    pub list:Box<Vec<dyn Hitable>>,
}
