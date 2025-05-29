use std::f64::INFINITY;

use crate::{hittable::{hittable_list::HittableList, HitRecord, Hittable}, image::{ppm::PPM, Image, ToFile}, ray::Ray, timer::Timer, utility::{interval::Interval, random_double}, vec3::{Color, Point3, Vec3}};

pub struct Camera {
    
    // Public
    pub aspect_ratio: f64, // Image Ratio
    pub image_width: u32, // Image width
    pub samples_per_pixel: u32, // Count of random samples for each pixel
    pub max_depth: u32, // Max number of bounces
    // Private
    image_height: u32, // Image height
    center: Point3, // Camera Center
    pixel_00_loc: Point3, // Locaiton of pixel (0,0)
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel to below
    pixel_sample_scale: f64, // Collor sample scale for a sum of pixel samples
}

impl Default for Camera {
    fn default() -> Self {
        Camera { 
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0),
            pixel_sample_scale: 1.0,
        }
    }
}

impl Camera {
    
    pub fn render(&mut self, world: &HittableList) {
        
        self.initialize();
        
        let mut new_data = vec![];
        new_data.resize_with((self.image_width * self.image_height) as usize, || Default::default());
        
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
                
                new_data[(j * self.image_width + i) as usize] = pixel_color * self.pixel_sample_scale;
            }
        }
    
        
        // Save the image
        let mut new_image = Image::new(self.image_width, self.image_height);
        new_image.load_data(new_data);
        
        let ppm_new_image: PPM = new_image.into();
        ppm_new_image.save("test_22.ppm").unwrap();
        
    }
    
    fn initialize(&mut self) {
        
        // Calculate Image height and ensure its at least one
        let image_height = (self.image_width as f64 / self.aspect_ratio) as u32;
        self.image_height = if image_height < 1 { 1 } else { image_height };
        
        self.center = Point3::new(0.0, 0.0, 0.0);
     
        // Camera
        // Viewport widths less then one are ok since they are real valued
        let focal_length = 1.0;
        let viewport_height: f64 = 2.0;
        let viewport_width: f64 = viewport_height * (self.image_width as f64 / self.image_height as f64);
    
        // Calculate the vectors accross the horizontal and down the vertical viewport edges
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    
        // Calculate horizontal and vertical delta vectors from pixel to pixel
        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;
    
        // Calculate location of upper left pixel
        let viewport_upper_left =
            self.center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel_00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);
        
        self.pixel_sample_scale = 1.0 / self.samples_per_pixel as f64;
        
        
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
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.
        
        let offset = Self::sample_square();
        
        // Fires a ray somewhere around (i,j) coordinate
        let pixel_sample = self.pixel_00_loc 
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);
        
        // Create Ray
        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;
        Ray::new(ray_origin, ray_direction)
    }
    
    fn sample_square() -> Vec3 {
        Vec3::new(
            random_double() - 0.5,
           random_double() - 0.5,
           0.0)
    }
    
}