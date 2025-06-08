use std::ops::Range;

use crate::{
    hittable::{HitRecord, Hittable, material::Material},
    point3::Point3,
    ray::Ray,
    vec3::Vec3,
};

#[derive(Default, Clone)]
pub struct Sphere<M> {
    center: Point3,
    radius: f64,
    mat: M,
}

impl<M> Sphere<M> {
    pub fn new(center: Point3, radius: f64, mat: M) -> Self {
        Self {
            center,
            radius: f64::max(radius, 0.0),
            mat,
        }
    }
}

impl<M: Material + Clone + 'static> Hittable for Sphere<M> {
    fn hit(&self, r: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = Vec3::dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.powf(0.5);

        // Find the nearest root that lies in the acceptable range
        let mut root = (h - sqrtd) / a;
        if !ray_t.contains(&root) {
            root = (h + sqrtd) / a;
            if !ray_t.contains(&root) {
                return None;
            }
        }

        let outward_normal = (r.at(root) - self.center) / self.radius;
        Some(HitRecord::new(r, root, self.mat.clone(), outward_normal))
    }
}
