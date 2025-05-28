use std::rc::Rc;

use camera::Camera;
use hittable::{hittable_list::HittableList, sphere::Sphere};
use vec3::Point3;

mod hittable;
mod image;
mod camera;
mod ray;
pub mod utility;
pub mod vec3;
pub mod timer;


fn main() {
    // Aspect Ratio

    let mut camera = Camera::default();
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 100;
    camera.max_depth = 50;

    // World
    let mut world = HittableList::default();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));
    
    camera.render(&world);
}
