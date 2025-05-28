use std::f64::INFINITY;

use crate::{hittable::{hittable_list::HittableList, HitRecord, Hittable}, image::{pixel::Color, ppm::PPM, Image, ToFile}, ray::Ray, utility::interval::Interval, vec3::{Point3, Vec3}};

pub struct Camera {
    
    // Public
    pub aspect_ratio: f64, // Image Ratio
    pub image_width: u32, // Image width
    
    // Private
    image_height: u32, // Image height
    center: Point3, // Camera Center
    pixel_00_loc: Point3, // Locaiton of pixel (0,0)
    pixel_delta_u: Vec3, // Offset to pixel to the right
    pixel_delta_v: Vec3, // Offset to pixel to below
}

impl Default for Camera {
    fn default() -> Self {
        Camera { 
            aspect_ratio: 1.0,
            image_width: 100,
            image_height: 0,
            center: Point3::new(0.0, 0.0, 0.0),
            pixel_00_loc: Point3::new(0.0, 0.0, 0.0),
            pixel_delta_u: Vec3::new(0.0, 0.0, 0.0),
            pixel_delta_v: Vec3::new(0.0, 0.0, 0.0)
        }
    }
}

impl Camera {
    
    pub fn render(&mut self, world: &HittableList) {
        
        self.initialize();
        
        let mut new_data = vec![];
        new_data.resize_with((self.image_width * self.image_height) as usize, || Default::default());
    
        // Render
        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel_center =
                    self.pixel_00_loc + (i as f64 * self.pixel_delta_u) + (j as f64 * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let r = Ray::new(self.center, ray_direction);
    
                // This is compromise for ray color
    
                new_data[(j * self.image_width + i) as usize] = Camera::ray_color(&r, &world);
            }
        }
    
        let mut new_image = Image::new(self.image_width, self.image_height);
        new_image.load_data(new_data);
    
        let ppm_new_image: PPM = new_image.into();
        ppm_new_image.save("test_2.ppm").unwrap();
        
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
        
        
    }
    
    fn ray_color(r: &Ray, world: &HittableList) -> Color {
        let mut rec = HitRecord::default();
    
        if world.hit(&r, Interval::new(0.0, INFINITY), &mut rec) {
            return (0.5 * (rec.normal + Vec3::new(1.0, 1.0, 1.0))).into();
        }
    
        let unit_direction = Vec3::unit_vector(r.direction());
        let a = 0.5 * (unit_direction.y() + 1.0);
        let c = (1.0 - a) * Vec3::new(1.0, 1.0, 1.0) + a * Vec3::new(0.5, 0.7, 1.0);
        c.into()
    }
    
}