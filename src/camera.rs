use rand::Rng as _;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable, Scattering, hittable_list::HittableList},
    image::Image,
    point3::Point3,
    ray::Ray,
    timer::Timer,
    vec3::Vec3,
};

pub struct Camera {
    image_width: u32,       // Image width
    samples_per_pixel: u32, // Count of random samples for each pixel
    max_depth: u32,         // Max number of bounces

    defocus_angle: f64,   // Variation angle of rays for each pixel
    defocus_disk_u: Vec3, // Defocus disck horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius

    image_height: u32,       // Image height
    center: Point3,          // Camera Center
    pixel_00_loc: Point3,    // Locaiton of pixel (0,0)
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel to below
    pixel_sample_scale: f64, // Collor sample scale for a sum of pixel samples
}

impl Camera {
    /// Renders an image of the world
    pub fn render<A: FnMut(f64), B: FnMut()>(
        &self,
        world: &HittableList,
        on_tick: A,
        on_done: B,
    ) -> Image {
        let mut new_data =
            vec![Default::default(); (self.image_width * self.image_height) as usize];

        let mut timer = Timer::new(self.image_height as usize, 1, on_tick, on_done);
        timer.wind_up();

        // Render
        for j in 0..self.image_height {
            timer.tick();
            for i in 0..self.image_width {
                let mut pixel_color = Color::default();

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }

                new_data[(j * self.image_width + i) as usize] =
                    pixel_color * self.pixel_sample_scale;
            }
        }

        // Save the image
        Image::new(self.image_width, self.image_height, new_data)
    }

    fn ray_color(r: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth == 0 {
            return Color::default();
        }

        // Go through all object and check if they are hit
        if let Some(HitRecord { scattered, .. }) = world.hit(r, 0.001..f64::INFINITY) {
            return match scattered {
                Some(Scattering {
                    attenuation,
                    scattered,
                }) => attenuation * Camera::ray_color(&scattered, depth - 1, world),
                None => Color::default(),
            };
        }

        // If ray missed all of the geometry it "will hit the sky" and the color that it should be is this one
        let unit_direction = r.direction().unit_vector();
        let a = 0.5 * (unit_direction.y() + 1.0);
        Color::combine(&Color::new(1.0, 1.0, 1.0), &Color::new(0.5, 0.7, 1.0), a)
    }

    /// Construct a camera ray originating from the defocus disk and directed at a randomly
    /// sampled point around the pixel location i, j.
    fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();

        // Fires a ray somewhere around (i,j) coordinate
        let pixel_sample = self.pixel_00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // Create Ray
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }

    /// Returns random point in camera defocus disk
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disc();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(
            rand::rng().random::<f64>() - 0.5,
            rand::rng().random::<f64>() - 0.5,
            0.0,
        )
    }
}

#[derive(Clone, Copy)]
pub struct CameraBuilder {
    pub aspect_ratio: f64,      // Image Ratio
    pub image_width: u32,       // Image width
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32,         // Max number of bounces

    pub v_fov: f64,       // Vertical view angle (field of view)
    pub lookfrom: Point3, // Camera looking from
    pub lookat: Point3,   // Looking at
    pub vup: Vec3,        // Camera-relative "up" direction

    pub defocus_angle: f64, // Variation angle of rays for each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus
}

impl Default for CameraBuilder {
    fn default() -> Self {
        CameraBuilder {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            v_fov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Vec3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
        }
    }
}

impl CameraBuilder {
    pub fn build(self) -> Camera {
        let Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            v_fov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
        } = self;

        // Calculate Image height and ensure its at least one
        let image_height = (image_width as f64 / aspect_ratio) as u32;
        let image_height = if image_height < 1 { 1 } else { image_height };

        let pixel_sample_scale = 1.0 / samples_per_pixel as f64;

        let center = lookfrom;

        // Camera
        // Fov calculations and viewport size
        let theta = v_fov.to_radians();
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focus_dist;
        let viewport_width: f64 = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        let w = Vec3::unit_vector(&(lookfrom - lookat));
        let u = Vec3::unit_vector(&Vec3::cross(&vup, &w));
        let v = Vec3::cross(&w, &u);

        // Calculate the vectors accross the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * u; // Vector across viewport horizontal edge
        let viewport_v = -viewport_height * v; // Vector down viewport vertical edge

        // Calculate horizontal and vertical delta vectors from pixel to pixel
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate location of upper left pixel
        let viewport_upper_left = center - (focus_dist * w) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        // Calcualte camera defocus disk basis vectors
        let defocus_radius = focus_dist
            * f64::tan({
                let degrees = defocus_angle / 2.0;
                degrees.to_radians()
            });
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            image_width,
            samples_per_pixel,
            max_depth,
            defocus_angle,
            defocus_disk_u,
            defocus_disk_v,
            image_height,
            center,
            pixel_00_loc,
            pixel_delta_u,
            pixel_delta_v,
            pixel_sample_scale,
        }
    }
}
