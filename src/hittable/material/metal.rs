use crate::{
    ray::Ray,
    vec3::{Color, Vec3},
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzz_factor: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz_factor: f64) -> Self {
        let fuzz = if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 };

        Self {
            albedo: albedo,
            fuzz_factor: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &crate::ray::Ray,
        rec: &crate::hittable::HitRecord,
        attenuation: &mut crate::vec3::Color,
        scattered: &mut crate::ray::Ray,
    ) -> bool {
        let reflected = Vec3::reflect(&r_in.direction(), &rec.normal);
        let reflected =
            Vec3::unit_vector(&reflected) + (self.fuzz_factor * Vec3::random_unit_vector());
        *scattered = Ray::new(rec.p, reflected);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
