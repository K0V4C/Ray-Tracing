use image::{pixel::fPixel, ppm::PPM, Image, ToFile};

mod image;

fn main() {
    
    let mut data = vec![];
    data.resize_with(256 * 256, || {fPixel::default()});
    
    for i in 0..256 {
        for j in 0..256 {
            
            data[i*256 + j] = fPixel {
                red: (i as f32) / 256.0,
                green: (j as f32) / 256.0,
                blue: 0.0,
                alpha: 0.0,
            }
            
        }
    }
    
    let mut image = Image::new(256, 256);
    image.load_data(data);
    
    let ppm_image: PPM = image.into();
    match ppm_image.save("test_image.ppm") {
        Ok(_) => {},
        Err(e) => {println!("{}", e);}
    }
    
    
}
