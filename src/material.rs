use crate::base::*;
use crate::hitable::*;
use crate::ray::*;
use cgmath::dot;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> (bool, Color, Ray);
}

#[derive(Copy, Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(color: &Color) -> Lambertian {
        Lambertian { albedo: *color }
    }
}

impl Material for Lambertian {
    fn scatter(&self, _: &Ray, record: HitRecord) -> (bool, Color, Ray) {
        let target = record.p + record.normal + random_in_unit_sphere();
        (true, self.albedo, Ray::new(record.p, target - record.p))
    }
}

fn reflect(v: &Position, n: &Position) -> Color {
    v - 2.0 * dot(*v, *n) *n
}

#[derive(Copy, Clone)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(color: &Color, fuzz: f64) -> Metal {
        Metal { albedo: *color, fuzz: if fuzz > 1.0 { 1.0 } else { fuzz } }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> (bool, Color, Ray) {
        let reflected = reflect(&ray.direction().unit_vector(), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());
        (
            (dot(*scattered.direction(), record.normal) > 0.0),
            self.albedo,
            scattered
        )
    }
}
