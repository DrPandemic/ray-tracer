use crate::base::*;
use crate::ray::*;
use cgmath::dot;
use std::vec::Vec;

pub struct HitRecord {
    pub t: f64,
    pub p: Position,
    pub normal: Position,
}

impl HitRecord {
    pub fn default() -> HitRecord {
        HitRecord { t: 0.0, p: Position::new(0.0, 0.0, 0.0), normal: Position::new(0.0, 0.0, 0.0) }
    }

    pub fn clone(&mut self, other: &HitRecord) {
        self.t = other.t;
        self.p = other.p;
        self.normal = other.normal;
    }
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
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64, record: &mut HitRecord) -> bool {
        let mut tmp_record = HitRecord::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for item in self.list.iter() {
            if item.hit(&ray, t_min, closest_so_far, &mut tmp_record) {
                hit_anything = true;
                closest_so_far = tmp_record.t;
                record.clone(&tmp_record);
            }
        }

        hit_anything
    }
}
