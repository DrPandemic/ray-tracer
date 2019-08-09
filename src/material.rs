use crate::base::*;
use crate::hitable::*;
use crate::ray::*;
use cgmath::dot;

pub trait Material {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Color, Ray)>;
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
    fn scatter(&self, _: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let target = record.p + record.normal + random_in_unit_sphere();
        Some((self.albedo, Ray::new(record.p, target - record.p)))
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Color {
    v - 2.0 * dot(*v, *n) *n
}

fn refract(v: &Vec3, n: &Vec3, ni_over_nt: f64) -> Option<Vec3> {
    let uv = v.unit_vector();
    let dt = dot(uv, *n);
    let discriminant = 1.0 - ni_over_nt*ni_over_nt * (1.0 - dt*dt);

    if discriminant > 0.0 {
        Some(ni_over_nt * (uv - n*dt) - n * discriminant.sqrt())
    } else {
        None
    }
}

fn schlick(cosine: f64, refraction_index: f64) -> f64 {
    let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
    r0 *= r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
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
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let reflected = reflect(&ray.direction().unit_vector(), &record.normal);
        let scattered = Ray::new(record.p, reflected + self.fuzz * random_in_unit_sphere());
        if dot(*scattered.direction(), record.normal) > 0.0 {
            Some((self.albedo, scattered))
        } else {
            None
        }
    }
}

pub struct Dialectric {
    pub refraction_index: f64,
}

impl Dialectric {
    pub fn new(ri: f64) -> Dialectric { Dialectric { refraction_index: ri } }
}

impl Material for Dialectric {
    fn scatter(&self, ray: &Ray, record: HitRecord) -> Option<(Color, Ray)> {
        let (outward_normal, ni_over_nt, cosine) =
            if dot(*ray.direction(), record.normal) > 0.0 {
                let cosine = dot(*ray.direction(), record.normal) / ray.direction().length();
                (
                    -record.normal,
                    self.refraction_index,
                    (1.0 - self.refraction_index * self.refraction_index * (1.0 - cosine*cosine)).sqrt()
                )
            } else {
                (
                    record.normal,
                    1.0 / self.refraction_index,
                    -dot(*ray.direction(), record.normal) / ray.direction().length()
                )
            };

        let (refracted, reflect_probability) =
            refract(ray.direction(), &outward_normal, ni_over_nt).map_or_else(
                || {(Vec3::new(0.0, 0.0, 0.0), 1.0)},
                |refracted| {(refracted, schlick(cosine, self.refraction_index))}
            );

        if random() < reflect_probability {
            Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(record.p, reflect(ray.direction(), &record.normal))))
        } else {
            Some((Vec3::new(1.0, 1.0, 1.0), Ray::new(record.p, refracted)))
        }
    }
}
