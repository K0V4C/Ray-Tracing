use crate::{
    hittable::{Material, Scattering},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz_factor: f64,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz_factor: f64) -> Self {
        let fuzz = if fuzz_factor < 1.0 { fuzz_factor } else { 1.0 };

        Self {
            albedo,
            fuzz_factor: fuzz,
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, t: f64, normal: Vec3, _: bool) -> Option<Scattering> {
        let reflected = Vec3::reflect(r_in.direction(), &normal);
        let reflected =
            Vec3::unit_vector(&reflected) + (self.fuzz_factor * Vec3::random_unit_vector());
        let scattered = Ray::new(r_in.at(t), reflected);
        let attenuation = self.albedo;
        if Vec3::dot(scattered.direction(), &normal) > 0.0 {
            Some(Scattering {
                attenuation,
                scattered,
            })
        } else {
            None
        }
    }
}
