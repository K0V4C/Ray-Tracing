use crate::{ray::Ray, utility::random_double, vec3::{Color, Vec3}};

use super::Material;


pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64, 
}

impl Dielectric {
    pub fn new(index: f64) -> Self  {
        Self { refraction_index: index }
    }
    
    
    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powf(1.0 - cosine, 5.0)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &crate::ray::Ray, rec: &crate::hittable::HitRecord, attenuation: &mut crate::vec3::Color, scattered: &mut crate::ray::Ray) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {1.0 / self.refraction_index} else {self.refraction_index};
        
        let unit_direction = Vec3::unit_vector(r_in.direction());
        
        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);
        
        let cannot_refract = ri * sin_theta > 1.0;
        
        let direction = if cannot_refract || (Self::reflectance(cos_theta, ri) > random_double())  {
            Vec3::reflect(&unit_direction, &rec.normal)
        } else {
           Vec3::refract(&unit_direction, &rec.normal, ri)
        };
        
        *scattered = Ray::new(rec.p, direction);
        true
    }
}