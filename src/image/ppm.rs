use std::{
    fs::OpenOptions,
    io::Write,
};

use anyhow::{Result, anyhow};

use crate::utility::interval::Interval;

use super::{Image, ToFile, pixel::uPixel};

pub struct PPM {
    data: Vec<uPixel>,
    width: u32,
    height: u32,
}

impl PPM {
    
    fn linear_to_gama(linear_component: f64) -> f64 {
        if linear_component > 0.0 {
            return linear_component.powf(0.5);
        }
        return 0.0;
    }
    
}

const INTENSITY: Interval = Interval {min: 0.000, max: 0.999};

impl From<Image> for PPM {
    fn from(value: Image) -> Self {
        // Image stores pixels as values from 0.0 to 1.0
        // PPM image format requires us to use values from 0 to 255
        let converted = value
            .data
            .iter()
            .map(|x| {
                
                let red = Self::linear_to_gama(x.red);
                let green = Self::linear_to_gama(x.green);
                let blue = Self::linear_to_gama(x.blue);
                let alpha = x.alpha;
                
                uPixel {
                    red: (256.0 * INTENSITY.clamp(red)) as u8,
                    green: (256.0 * INTENSITY.clamp(green)) as u8,
                    blue: (256.0 * INTENSITY.clamp(blue)) as u8,
                    alpha: (256.0 * INTENSITY.clamp(alpha)) as u8,
                }
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
                for (_idx, upixel) in self.data.iter().enumerate() {
                    let pixel = format!("{} {} {}\n", upixel.red, upixel.green, upixel.blue);
                    open_file.write(pixel.as_bytes())?;
                }
                Ok(())
            }

            Err(_) => Err(anyhow!("Could not create new file for image saving.")),
        }
    }
}
