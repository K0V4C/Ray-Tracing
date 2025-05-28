use std::{
    fs::OpenOptions,
    io::{Seek, Write},
};

use anyhow::{Result, anyhow};

use super::{Image, ToFile, pixel::uPixel};

pub struct PPM {
    data: Vec<uPixel>,
    width: u32,
    height: u32,
}

impl PPM {}

impl From<Image> for PPM {
    fn from(value: Image) -> Self {
        // Image stores pixels as values from 0.0 to 1.0
        // PPM image format requires us to use values from 0 to 255
        let converted = value
            .data
            .iter()
            .map(|x| uPixel {
                red: (x.red * 255.0) as u8,
                green: (x.green * 255.0) as u8,
                blue: (x.blue * 255.0) as u8,
                alpha: (x.alpha * 255.0) as u8,
            })
            .collect();

        PPM {
            data: converted,
            width: value.width,
            height: value.height,
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
                open_file.write(self.get_metadata().as_bytes())?;
                for (idx, upixel) in self.data.iter().enumerate() {
                    let pixel = format!("{} {} {}\n", upixel.red, upixel.green, upixel.blue);
                    open_file.write(pixel.as_bytes())?;
                }
                Ok(())
            }

            Err(_) => Err(anyhow!("Could not create new file for image saving.")),
        }
    }
}
