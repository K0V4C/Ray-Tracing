use std::ops::Range;

use material::Material;

use crate::{point3::Point3, ray::Ray, vec3::Vec3};

pub mod hittable_list;
pub mod material;
pub mod sphere;

#[derive(Clone, Copy)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    // pub mat: Box<dyn Material>,
    pub scattered: Option<Scattering>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // With this implementation we choose to set front_face at time of hitting
    // So we have to remember it, also object has to set it
    pub fn new<M: Material + 'static>(ray: &Ray, t: f64, mat: M, outward_normal: Vec3) -> Self {
        let p = ray.at(t);
        let dot = Vec3::dot(ray.direction(), &outward_normal);
        let front_face = dot < 0.0;
        let normal = -dot.signum() * outward_normal;
        let scattered = mat.scatter(ray, t, normal, front_face);
        Self {
            p,
            normal,
            // mat: Box::new(mat),
            scattered,
            t,
            front_face,
        }
    }
}

#[derive(Clone, Copy)]
pub struct Scattering {
    pub attenuation: Vec3,
    pub scattered: Ray,
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Range<f64>) -> Option<HitRecord>;
}
