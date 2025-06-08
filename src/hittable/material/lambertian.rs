use crate::{
    hittable::{Material, Scattering},
    ray::Ray,
    vec3::Vec3,
};

#[derive(Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, t: f64, normal: Vec3, _: bool) -> Option<Scattering> {
        let mut scatter_direction = normal + Vec3::random_unit_vector();

        if scatter_direction.near_zero() {
            scatter_direction = normal;
        }

        let scattered = Ray::new(r_in.at(t), scatter_direction);
        let attenuation = self.albedo;
        Some(Scattering {
            attenuation,
            scattered,
        })
    }
}
