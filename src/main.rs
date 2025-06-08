use ray_tracing::{
    camera::CameraBuilder,
    hittable::{
        hittable_list::HittableList,
        material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
        sphere::Sphere,
    },
    image::{ToFile as _, ppm::PPM},
    point3::Point3,
    vec3::Vec3,
};

use anyhow::Result;
use rand::Rng as _;

fn _test_render(world: &mut HittableList) {
    // Camera setup
    // let camera = CameraBuilder {
    //     aspect_ratio: 16.0 / 9.0,
    //     image_width: 400,
    //     samples_per_pixel: 300,
    //     max_depth: 50,
    //
    //     v_fov: 30.0,
    //     lookfrom: Point3::new(-2.0, 2.0, 1.0),
    //     lookat: Point3::new(0.0, 0.0, -1.0),
    //     vup: Point3::new(0.0, 1.0, 0.0),
    //
    //     defocus_angle: 10.0,
    //     focus_dist: 3.4,
    // }
    // .build();

    // Old test code
    let material_ground = Lambertian::new(Vec3::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Vec3::new(0.1, 0.2, 0.5));
    let material_left = Dielectric::new(1.5);
    let material_bubble = Dielectric::new(1.00 / 1.50);
    let material_right = Metal::new(Vec3::new(0.8, 0.8, 0.0), 1.0);

    world.add(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    ));
    world.add(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    ));
    world.add(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    ));
    world.add(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    ));
}

fn first_book_finale() -> HittableList {
    let mut world = HittableList::new();

    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.add(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    ));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rand::rng().random::<f64>();
            let center = Point3::new(
                a as f64 + 0.9 * rand::rng().random::<f64>(),
                0.2,
                b as f64 + 0.9 * rand::rng().random::<f64>(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random() * Vec3::random();
                    world.add(Sphere::new(center, 0.2, Lambertian::new(albedo)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Vec3::random_clamp(0.5, 1.0);
                    let fuzz = rand::rng().random_range(0.0..0.5);
                    world.add(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)));
                } else {
                    // Glass
                    world.add(Sphere::new(center, 0.2, Dielectric::new(1.5)));
                };
            }
        }
    }

    let material1 = Lambertian::new(Vec3::new(0.4, 0.2, 0.1));
    world.add(Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, material1));

    let material2 = Dielectric::new(1.5);
    world.add(Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, material2));

    let material3 = Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0);
    world.add(Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, material3));

    world
}

fn main() -> Result<()> {
    // Camera setup
    let camera = CameraBuilder {
        aspect_ratio: 16.0 / 9.0,
        image_width: 1200,
        samples_per_pixel: 500,
        max_depth: 50,

        v_fov: 20.0,
        lookfrom: Point3::new(13.0, 2.0, 3.0),
        lookat: Point3::new(0.0, 0.0, 0.0),
        vup: Vec3::new(0.0, 1.0, 0.0),

        defocus_angle: 0.6,
        focus_dist: 10.0,
    }
    .build();

    let world = first_book_finale();

    let rendered = camera.render(
        &world,
        |t| println!("Working on it: {t}%"),
        || println!("done"),
    );

    let ppm_new_image: PPM = rendered.into();
    ppm_new_image.save("final_render2.ppm")?;

    Ok(())
}
