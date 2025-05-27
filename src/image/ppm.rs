use std::{fs::OpenOptions, io::{Seek, Write}};

use anyhow::{anyhow, Result};

use super::{pixel::uPixel, Image, ToFile};


pub struct PPM {
    data: Vec<uPixel>,
    width: u32,
    height: u32,
}

impl PPM {}

impl From<Image> for PPM {
    fn from(value: Image) -> Self {
        
        let converted = value.data.iter().map(|x| {
            uPixel {
                red: (x.red * 255.0) as u8,
                green: (x.green * 255.0) as u8,
                blue: (x.blue * 255.0) as u8,
                alpha: (x.alpha * 255.0) as u8
            }
        }).collect();
        
        PPM {
            data: converted,
            width: value.width,
            height: value.height
        } 
    }
}

impl ToFile for PPM {
    
    fn get_metadata(&self) -> String {
        format!("P3\n{} {}\n255\n", self.width, self.height)
    }
    
    fn save(&self, image_path: &str) -> Result<()> {
        
        PPM::delete_if_exits(image_path)?;
        
        match OpenOptions::new().write(true).create(true).open(image_path) {
            
            Ok(mut open_file) => {
                
                let metadata = self.get_metadata();
                open_file.write(metadata.as_bytes())?;
                
                for upixel in &self.data {
                    let pixel = format!("{} {} {}\n", upixel.red, upixel.green, upixel.blue);
                    open_file.write(pixel.as_bytes())?;
                }
                
                
                Ok(())
            },
            
            Err(_) => {
                Err(anyhow!("Could not create new file for image saving."))
            }
            
        }
    }
}