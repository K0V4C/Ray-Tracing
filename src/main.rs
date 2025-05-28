use image::{pixel::{fPixel, Color}, ppm::PPM, Image, ToFile};
use ray::Ray;
use vec3::{Point3, Vec3};

mod image;
mod ray;
pub mod vec3;

fn ray_color(r: &Ray) -> Color {

    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, r) {
         return Vec3::new(1.0, 0.0, 0.0).into();
    }

    let unit_direction = Vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.0);
    let c = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
    c.into()
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = *center - *r.origin();
    let a = Vec3::dot(r.direction(), r.direction());
    let b = -2.0 * Vec3::dot(r.direction(), &oc);
    let c = Vec3::dot(&oc, &oc) - radius * radius;

    let discriminant = b*b - 4.0 * a * c;

    discriminant >= 0.0
}

fn main() {

    // Aspect Ratio

    let aspect_ratio: f64 = 16.0 / 9.0;
    let image_width: u32 = 400;

    // Calculate Image height and ensure its at least one
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let image_height = if image_height < 1 {
        1
    } else {
        image_height
    };

    // Camera
    // Viewport widths less then one are ok since they are real valued
    let viewport_height: f64 = 2.0;
    let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);
    let focal_length = 1.0;
    let camera_center: Point3 = Point3::new(0.0, 0.0, 0.0);

    // Calculate the vectors accross the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;

    // Calculate location of upper left pixel
    let viewport_upper_left = camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut new_data = vec![];
    new_data.resize_with((image_width * image_height) as usize, || Default::default());

    // Render
    for j in 0..image_height {
        for i in 0..image_width {

            let pixel_center = pixel_00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            // This is compromise for ray color

            new_data[(j * image_width + i) as usize] = ray_color(&r);
        }
    }

    let mut new_image = Image::new(image_width, image_height);
    new_image.load_data(new_data);

    let ppm_new_image: PPM = new_image.into();
    ppm_new_image.save("test_2.ppm").unwrap();




}
