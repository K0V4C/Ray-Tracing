use crate::vec3::Vec3;


#[derive(Default)]
pub struct fPixel {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
    pub alpha: f32
}

impl From<Vec3> for fPixel {
    fn from(value: Vec3) -> Self {
        Self {
            red: value.e[0] as f32,
            green: value.e[1] as f32,
            blue: value.e[2] as f32,
            alpha: 0.0,
        }
    }
}

pub type Color = fPixel;

#[derive(Default)]
pub struct uPixel {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    pub alpha: u8
}


