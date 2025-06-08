use rand::Rng as _;

use crate::{
    hittable::{Material, Scattering},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

impl Dielectric {
    pub fn new(index: f64) -> Self {
        Self {
            refraction_index: index,
        }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).sqrt()
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, t: f64, normal: Vec3, front_face: bool) -> Option<Scattering> {
        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let ri = if front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = Vec3::unit_vector(r_in.direction());

        let cos_theta = f64::min(Vec3::dot(&-unit_direction, &normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;

        let direction =
            if cannot_refract || (Self::reflectance(cos_theta, ri) > rand::rng().random::<f64>()) {
                Vec3::reflect(&unit_direction, &normal)
            } else {
                Vec3::refract(&unit_direction, &normal, ri)
            };

        let scattered = Ray::new(r_in.at(t), direction);
        Some(Scattering {
            attenuation,
            scattered,
        })
    }
}
