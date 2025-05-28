use std::path::Path;

use anyhow::{Result, anyhow};
use pixel::fPixel;

pub mod pixel;
pub mod ppm;

pub struct Image {
    data: Vec<fPixel>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            data: vec![],
            width,
            height,
        }
    }

    pub fn load_data(&mut self, data: Vec<fPixel>) {
        self.data = data;
    }
}

pub trait ToFile {
    fn save(&self, image_path: &str) -> Result<()>;

    fn get_metadata(&self) -> String;

    fn delete_if_exits(image_path: &str) -> Result<()> {
        if Path::new(image_path).exists() {
            match std::fs::remove_file(image_path) {
                Ok(_) => return Ok(()),
                Err(_) => return Err(anyhow!("Could not delete the file.")),
            }
        }

        Ok(())
    }
}
