use std::path::Path;

use anyhow::{anyhow, Result};
use pixel::fPixel;

pub mod ppm;
pub mod pixel;

pub struct Image {
    data: Vec<fPixel>,
    width: u32,
    height: u32
}

impl Image {
    
    pub fn new(width: u32, height: u32) -> Self {
       Self {
           data: vec![],
           width,
           height
       } 
    }
    
    pub fn load_data(&mut self, data: Vec<fPixel>) {
        self.data = data;
    }
}

pub trait ToFile {
    
    fn save(&self, image_path: &str) -> Result<()>;
    
    fn get_metadata(&self) -> String;
    
    fn log_lines_remaining(idx: usize, image_width: u32, remaining: &mut u32) {
        
        if idx / image_width as usize != 0 {
            return;
        }
        
        println!("Lines remaining: {}", remaining);
        *remaining = remaining.saturating_sub(1);
        
        if *remaining == 0 {
            println!("Done!");
        }
    }
    
    fn delete_if_exits(image_path: &str) -> Result<()> {
        
        if Path::new(image_path).exists() {
            
            match std::fs::remove_file(image_path) {
                Ok(_) => {return Ok(())},
                Err(_) => {
                    return Err(anyhow!("Could not delete the file."))
                }
            }
            
        } 
        
        Ok(())
    }
    
}