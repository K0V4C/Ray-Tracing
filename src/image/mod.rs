use anyhow::Result;
use pixel::FPixel;

use crate::color::Color;

pub mod pixel;
pub mod ppm;

pub struct Image {
    data: Vec<FPixel>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(width: u32, height: u32, data: Vec<Color>) -> Self {
        assert_eq!(width as usize * height as usize, data.len());
        let data = data.iter().map(Color::to_fpixel).collect();
        Self {
            data,
            width,
            height,
        }
    }
}

pub trait ToFile {
    fn save(&self, image_path: &str) -> Result<()>;
}
