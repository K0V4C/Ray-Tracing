use crate::{ray::Ray, vec3::{Color, Vec3}};

use super::Material;


pub struct Metal {
    albedo: Color
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Self { albedo: albedo }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray) -> bool {
        
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        true
    }
}