use std::f64::consts::PI;
use crate::ray::*;
use crate::base::*;

pub struct Camera {
    pub lower_left_corner: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub origin: Vec3,
    pub lens_radius: f64,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
}

impl Camera {
    pub fn new(lookfrom: Vec3, lookat: Vec3, vup: Vec3, vfov: f64, aspect: f64, aperture: f64, focus_distance: f64) -> Camera {
        let theta = vfov * PI / 180.0;
        let half_height = (theta/2.0).tan();
        let half_width = aspect * half_height;
        let w = (lookfrom - lookat).unit_vector();
        let u = (vup.cross(w)).unit_vector();
        let v = w.cross(u);

        Camera {
            lower_left_corner: lookfrom - half_width * focus_distance * u - half_height * focus_distance * v - focus_distance * w,
            horizontal: 2.0 * half_width * focus_distance * u,
            vertical: 2.0 * half_height * focus_distance * v,
            origin: lookfrom,
            lens_radius: aperture / 2.0,
            u: u,
            v: v,
            w: w,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(self.origin + offset, self.lower_left_corner + s*self.horizontal + t * self.vertical - self.origin - offset)
    }
}
