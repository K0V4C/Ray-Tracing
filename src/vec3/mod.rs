use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, },
};

use crate::{camera, utility::{random_double, random_double_clamp}};

#[derive(Default, Clone, Copy, Debug)]
pub struct Vec3 {
    pub e: [f64; 3],
}

// Just an alias
pub type Point3 = Vec3;
pub type Color = Vec3;

impl Vec3 {
    pub fn new(e0: f64, e1: f64, e2: f64) -> Self {
        Self { e: [e0, e1, e2] }
    }

    // Getters
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }

    pub fn length(&self) -> f64 {
        self.length_squared().powf(0.5)
    }

    pub fn length_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }

    pub fn dot(u: &Vec3, v: &Vec3) -> f64 {
        u.e[0] * v.e[0] + u.e[1] * v.e[1] + u.e[2] * v.e[2]
    }

    pub fn cross(u: &Vec3, v: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                u.e[1] * v.e[2] - u.e[2] * v.e[1],
                u.e[2] * v.e[0] - u.e[0] * v.e[2],
                u.e[0] * v.e[1] - u.e[1] * v.e[0],
            ],
        }
    }

    // This uses copy trait
    pub fn unit_vector(v: &Vec3) -> Vec3 {
        *v / v.length()
    }
    
    pub fn random_unit_vector() -> Vec3 {
        loop {
            let p = Vec3::random_clamp(-1.0, 1.0);
            let lensq = p.length_squared();
            
            // First comparison is used because we are dealing with f64, we have finite precision
            // If number generated is too small length will be 0, and we will get +- inf for normalised value
            if 1e-160 < lensq && lensq <= 1.0 {
                return p / lensq.powf(0.5);
            }
        }
    }
    
    pub fn random_on_hemisphere(normal: &Vec3) -> Vec3 {
        let on_unit_hemisphere = Vec3::random_unit_vector();
        
        // If positive that means out vector has "same" direction as normal vector
        if Vec3::dot(&on_unit_hemisphere, &normal) > 0.0 {
            on_unit_hemisphere
        } else {
            -on_unit_hemisphere
        }
    }
    
    pub fn random() -> Vec3 {
        Vec3::new(
            random_double(), 
            random_double(), 
            random_double()
        )
    }
    
    pub fn random_clamp(min: f64, max: f64) -> Vec3 {
        Vec3::new(
            random_double_clamp(min, max),
            random_double_clamp(min, max),
            random_double_clamp(min, max)
        )
    }
}

// Negation operator
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        let negated = [-self.e[0], -self.e[1], -self.e[2]];

        Self { e: negated }
    }
}

// Index
impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        &self.e[index]
    }
}

// Index mutable
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.e[0] += rhs.e[0];
        self.e[1] += rhs.e[1];
        self.e[2] += rhs.e[2];
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.e[0] *= rhs.e[0];
        self.e[1] *= rhs.e[1];
        self.e[2] *= rhs.e[2];
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.e[0] /= rhs.e[0];
        self.e[1] /= rhs.e[1];
        self.e[2] /= rhs.e[2];
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.e[0], self.e[1], self.e[2])
    }
}

// Utility functions
impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Self) -> Self::Output {
        let res = [
            self.e[0] + rhs.e[0],
            self.e[1] + rhs.e[1],
            self.e[2] + rhs.e[2],
        ];

        Self { e: res }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Self) -> Self::Output {
        let res = [
            self.e[0] - rhs.e[0],
            self.e[1] - rhs.e[1],
            self.e[2] - rhs.e[2],
        ];

        Self { e: res }
    }
}

impl Mul<Self> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        let res = [
            self.e[0] * rhs.e[0],
            self.e[1] * rhs.e[1],
            self.e[2] * rhs.e[2],
        ];

        Self { e: res }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        let res = [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs];

        Self { e: res }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Self::Output {
        (1.0 / rhs) * self
    }
}
