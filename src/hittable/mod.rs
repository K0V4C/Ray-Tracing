use std::rc::Rc;

use material::Material;
use sphere::Sphere;

use crate::{
    ray::Ray,
    utility::interval::Interval,
    vec3::{Point3, Vec3},
};

pub mod hittable_list;
pub mod sphere;

pub mod material;

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Option<Rc<dyn Material>>,
    pub t: f64,
    pub front_face: bool,
}

impl HitRecord {
    // With this implementation we choose to set front_face at time of hitting
    // So we have to remember it, also object has to set it
    pub fn set_face_normal(&mut self, ray: &Ray, outword_normal: &Vec3) {
        // Sets the hit record normal vector
        // NOTE: this is assumed to have unit length

        self.front_face = Vec3::dot(ray.direction(), outword_normal) < 0.0;
        self.normal = if self.front_face {
            *outword_normal
        } else {
            -*outword_normal
        };
    }
}

pub trait Hittable {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
}
