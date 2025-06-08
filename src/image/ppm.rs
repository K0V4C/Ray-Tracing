use std::{fs::File, io::Write as _};

use anyhow::{Result, anyhow};

use crate::image::{Image, ToFile, pixel::UPixel};

use super::pixel::FPixel;

pub struct PPM {
    data: Vec<UPixel>,
    width: u32,
    height: u32,
}

impl From<Image> for PPM {
    fn from(value: Image) -> Self {
        let Image {
            data,
            width,
            height,
        } = value;

        // Image stores pixels as values from 0.0 to 1.0
        // PPM image format requires us to use values from 0 to 255
        let data = data.iter().map(FPixel::to_upixel).collect();

        PPM {
            data,
            width,
            height,
        }
    }
}

impl ToFile for PPM {
    fn save(&self, image_path: &str) -> Result<()> {
        let mut open_file = File::create(image_path)
            .map_err(|_e| anyhow!("Could not create new file for image saving."))?;
        writeln!(&mut open_file, "P3")?;
        writeln!(&mut open_file, "{} {}", self.width, self.height)?;
        writeln!(&mut open_file, "255")?;
        for UPixel { red, green, blue } in &self.data {
            writeln!(&mut open_file, "{red} {green} {blue}")?;
        }
        Ok(())
    }
}
