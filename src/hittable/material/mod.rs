use std::ops::Deref;

use crate::{hittable::Scattering, ray::Ray, vec3::Vec3};

pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub trait Material {
    fn scatter(&self, r_in: &Ray, t: f64, normal: Vec3, front_face: bool) -> Option<Scattering>;
}

impl<T: Deref<Target: Material>> Material for T {
    fn scatter(&self, r_in: &Ray, t: f64, normal: Vec3, front_face: bool) -> Option<Scattering> {
        (**self).scatter(r_in, t, normal, front_face)
    }
}
