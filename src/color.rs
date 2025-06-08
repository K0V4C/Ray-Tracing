use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{image::pixel::FPixel, vec3::Vec3};

#[derive(Default, Clone, Copy)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        Self { red, green, blue }
    }

    pub fn to_fpixel(&self) -> FPixel {
        let &Self { red, green, blue } = self;
        FPixel { red, green, blue }
    }

    pub fn combine(a: &Color, b: &Color, r: f64) -> Color {
        assert!((0.0..=1.0).contains(&r));
        let s = 1.0 - r;
        Color {
            red: a.red * s + b.red * r,
            green: a.green * s + b.green * r,
            blue: a.blue * s + b.blue * r,
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red + rhs.red,
            green: self.green + rhs.green,
            blue: self.blue + rhs.blue,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs
    }
}

impl Sub for Color {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            red: self.red - rhs.red,
            green: self.green - rhs.green,
            blue: self.blue - rhs.blue,
        }
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red * rhs,
            green: self.green * rhs,
            blue: self.blue * rhs,
        }
    }
}

impl Mul<Color> for f64 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<f64> for Color {
    fn mul_assign(&mut self, rhs: f64) {
        *self = *self * rhs;
    }
}

impl Div<f64> for Color {
    type Output = Color;

    fn div(self, rhs: f64) -> Self::Output {
        Color {
            red: self.red / rhs,
            green: self.green / rhs,
            blue: self.blue / rhs,
        }
    }
}

impl DivAssign<f64> for Color {
    fn div_assign(&mut self, rhs: f64) {
        *self = *self / rhs;
    }
}

impl Mul<Vec3> for Color {
    type Output = Color;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Color {
            red: self.red * rhs.x(),
            green: self.green * rhs.y(),
            blue: self.blue * rhs.z(),
        }
    }
}

impl Mul<Color> for Vec3 {
    type Output = Color;

    fn mul(self, rhs: Color) -> Self::Output {
        rhs * self
    }
}

impl MulAssign<Vec3> for Color {
    fn mul_assign(&mut self, rhs: Vec3) {
        *self = *self * rhs;
    }
}

impl Div<Vec3> for Color {
    type Output = Color;

    fn div(self, rhs: Vec3) -> Self::Output {
        Color {
            red: self.red / rhs.x(),
            green: self.green / rhs.y(),
            blue: self.blue / rhs.z(),
        }
    }
}

impl DivAssign<Vec3> for Color {
    fn div_assign(&mut self, rhs: Vec3) {
        *self = *self / rhs;
    }
}
