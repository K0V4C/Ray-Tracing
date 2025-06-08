use std::ops::Range;

use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

impl HittableList {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add<H: Hittable + 'static>(&mut self, object: H) {
        self.objects.push(Box::new(object));
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Range<f64>) -> Option<HitRecord> {
        let mut ret = None;
        let mut closest_so_far = ray_t.end;

        for object in &self.objects {
            if let Some(temp_rec) = object.hit(r, ray_t.start..closest_so_far) {
                closest_so_far = temp_rec.t;
                ret = Some(temp_rec);
            }
        }

        ret
    }
}
