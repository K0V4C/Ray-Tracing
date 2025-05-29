use std::f64::INFINITY;

use crate::{
    hittable::{HitRecord, Hittable, hittable_list::HittableList},
    image::{Image, ToFile, ppm::PPM},
    ray::Ray,
    timer::Timer,
    utility::{degrees_to_radians, interval::Interval, random_double},
    vec3::{Color, Point3, Vec3},
};

pub struct Camera {
    // Public
    pub aspect_ratio: f64,      // Image Ratio
    pub image_width: u32,       // Image width
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32,         // Max number of bounces

    pub v_fov: f64,       // Vertical view angle (field of view)
    pub lookfrom: Point3, // Camera looking from
    pub lookat: Point3,   // Looking at
    pub vup: Vec3,        // Camera-relative "up" direction
    
    pub defocus_angle: f64, // Variation angle of rays for each pixel
    pub focus_dist: f64, // Distance from camera lookfrom point to plane of perfect focus
    
    defocus_disk_u: Vec3, // Defocus disck horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius

    // Camera frame basis vectors
    u: Vec3,
    v: Vec3,
    w: Vec3,

    // Private
    image_height: u32,       // Image height
    center: Point3,          // Camera Center
    pixel_00_loc: Point3,    // Locaiton of pixel (0,0)
    pixel_delta_u: Vec3,     // Offset to pixel to the right
    pixel_delta_v: Vec3,     // Offset to pixel to below
    pixel_sample_scale: f64, // Collor sample scale for a sum of pixel samples
}

impl Default for Camera {
    fn default() -> Self {
        Camera {
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
            defocus_disk_u: Default::default(),
            defocus_disk_v: Default::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            image_height: 0,
            center: Point3::default(),
            pixel_00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            pixel_sample_scale: 1.0,
        }
    }
}

impl Camera {
    pub fn render(&mut self, world: &HittableList) {
        self.initialize();

        let mut new_data = vec![];
        new_data.resize_with((self.image_width * self.image_height) as usize, || {
            Default::default()
        });

        let mut timer = Timer::new(self.image_height as usize, 1);
        timer.wind_up();

        // Render
        for j in 0..self.image_height {
            timer.tick();
            for i in 0..self.image_width {
                let mut pixel_color: Vec3 = Vec3::default();

                for _ in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, &world);
                }

                new_data[(j * self.image_width + i) as usize] =
                    pixel_color * self.pixel_sample_scale;
            }
        }

        // Save the image
        let mut new_image = Image::new(self.image_width, self.image_height);
        new_image.load_data(new_data);

        let ppm_new_image: PPM = new_image.into();
        ppm_new_image.save("final_render.ppm").unwrap();
    }

    fn initialize(&mut self) {
        // Calculate Image height and ensure its at least one
        let image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if image_height < 1 { 1 } else { image_height };

        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;

        self.center = self.lookfrom;

        // Camera
        // Fov calculations and viewport size
        let theta = degrees_to_radians(self.v_fov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width: f64 =
            viewport_height * (self.image_width as f64 / self.image_height as f64);

        // Calculate the u, v, w unit basis vectors for the camera coordinate frame
        self.w = Vec3::unit_vector(&(self.lookfrom - self.lookat));
        self.u = Vec3::unit_vector(&Vec3::cross(&self.vup, &self.w));
        self.v = Vec3::cross(&self.w, &self.u);

        // Calculate the vectors accross the horizontal and down the vertical viewport edges
        let viewport_u = viewport_width * self.u; // Vector across viewport horizontal edge
        let viewport_v = viewport_height * -self.v; // Vector down viewport vertical edge

        // Calculate horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        // Calculate location of upper left pixel
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
        // Calcualte camera defocus disk basis vectors
        let defocus_radius = self.focus_dist * f64::tan(degrees_to_radians(self.defocus_angle / 2.0));
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn ray_color(r: &Ray, depth: u32, world: &HittableList) -> Color {
        if depth <= 0 {
            return Color::default();
        }

        let mut rec = HitRecord::default();

        // Go through all object and check if they are hit
        if world.hit(&r, Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::default();
            let mut attenuation = Color::default();
            let material = rec.mat.as_ref().unwrap();

            if material.scatter(&r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Camera::ray_color(&mut scattered, depth - 1, &world);
            }
            return Color::default();
        }

        // If ray missed all of the geometry it "will hit the sky" and the color that it should be is this one
        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        let c = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        c.into()
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the defocus disk and directed at a randomly
        // sampled point around the pixel location i, j.

        let offset = Self::sample_square();

        // Fires a ray somewhere around (i,j) coordinate
        let pixel_sample = self.pixel_00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // Create Ray
        let ray_origin = if self.defocus_angle <= 0.0 {self.center} else {self.defocus_disk_sample()};
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
    
    fn defocus_disk_sample(&self) -> Point3 {
        // Returns random point in camera defocus disk
        
        let p = Vec3::random_in_unit_disc();
        self.center + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    fn sample_square() -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }
}
