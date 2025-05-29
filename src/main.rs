use std::{f64::consts::PI, rc::Rc};

use camera::Camera;
use hittable::{
    hittable_list::HittableList,
    material::{Material, dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    sphere::Sphere,
};
use utility::{random_double, random_double_clamp};
use vec3::{Color, Point3};

mod camera;
mod hittable;
mod image;
mod ray;
pub mod timer;
pub mod utility;
pub mod vec3;

fn _test_render(world:  &mut HittableList, camera: &mut Camera) {
    // Camera setup
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 400;
    camera.samples_per_pixel = 300;
    camera.max_depth = 50;
    
    camera.v_fov = 30.0;
    camera.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    camera.lookat = Point3::new(0.0, 0.0, -1.0);
    camera.vup = Point3::new(0.0, 1.0, 0.0);
    
    camera.defocus_angle = 10.0;
    camera.focus_dist = 3.4;
    
    // Old test code
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
}
fn first_book_finale(world:  &mut HittableList, camera: &mut Camera) {

    // Camera setup
    camera.aspect_ratio = 16.0 / 9.0;
    camera.image_width = 1200;
    camera.samples_per_pixel = 500;
    camera.max_depth = 50;
    
    camera.v_fov = 20.0;
    camera.lookfrom = Point3::new(13.0, 2.0, 3.0);
    camera.lookat = Point3::new(0.0, 0.0, 0.0);
    camera.vup = Point3::new(0.0, 1.0, 0.0);
    
    camera.defocus_angle = 0.6;
    camera.focus_dist = 10.0;
    
    let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(
        Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material
        )
    ));
    
    for a in -11..11 {
        for b in -11..11 {
            
            let choose_mat = random_double();
            let center = Point3::new(a as f64 + 0.9 * random_double(), 0.2, b as f64 + 0.9 * random_double());
            
            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Rc<dyn Material> = if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color::random() * Color::random();
                    Rc::new(Lambertian::new(albedo))
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color::random_clamp(0.5, 1.0);
                    let fuzz = random_double_clamp(0.0, 0.5);
                    Rc::new(Metal::new(albedo, fuzz))
                } else {
                    // Glass
                    Rc::new(Dielectric::new(1.5))
                };
                
                world.add(Rc::new(
                    Sphere::new(
                    center,
                    0.2,
                    sphere_material
                    )
                ));
            }
        }
    }
    
    let material1 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add( Rc::new(
        Sphere::new(
        Point3::new(0.0, 1.0, 0.0 ),
        1.0,
        material1
        )
    ));
    
    let material2 = Rc::new(Dielectric::new(1.5));
    world.add( Rc::new(
        Sphere::new(
        Point3::new(-4.0, 1.0, 0.0 ),
        1.0,
        material2
        )
    ));
    
    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add( Rc::new(
        Sphere::new(
        Point3::new(4.0, 1.0, 0.0 ),
        1.0,
        material3
        )
    ));
    
    
}   

fn main() {
    // Aspect Ratio

    let mut camera = Camera::default();

    // World
    let mut world = HittableList::default();

    first_book_finale(&mut world, &mut camera);
    
    camera.render(&world);
}
