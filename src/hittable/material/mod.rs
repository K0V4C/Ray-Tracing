use crate::{ray::Ray, vec3::Color};

pub mod metal;
pub mod dielectric;
pub mod lambertian;

use super::HitRecord;


pub trait Material {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, attenuation: &mut Color, scattered: &mut Ray) -> bool {
        return false;
    }
}