use std::rc::Rc;

use camera::Camera;
use hittable::{hittable_list::HittableList, material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material}, sphere::Sphere};
use vec3::{Color, Point3};

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
    
    
    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0))); 
    let material_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new( 1.5));
    let material_bubble = Rc::new(Dielectric::new( 1.00 / 1.50));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.0), 1.0));
    
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right
    )));
    
    camera.render(&world);
}
