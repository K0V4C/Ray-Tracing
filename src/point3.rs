use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::vec3::Vec3;

#[derive(Default, Clone, Copy)]
pub struct Point3 {
    e: [f64; 3],
}

impl Point3 {
    pub fn new(a: f64, b: f64, c: f64) -> Self {
        Self { e: [a, b, c] }
    }
}

impl Add<Vec3> for Point3 {
    type Output = Point3;

    fn add(self, rhs: Vec3) -> Self::Output {
        let [a, b, c] = self.e;
        let [p, q, r] = rhs.e;
        Point3 {
            e: [a + p, b + q, c + r],
        }
    }
}

impl Add<Point3> for Vec3 {
    type Output = Point3;

    fn add(self, rhs: Point3) -> Self::Output {
        rhs + self
    }
}

impl AddAssign<Vec3> for Point3 {
    fn add_assign(&mut self, rhs: Vec3) {
        *self = *self + rhs;
    }
}

impl Sub<Vec3> for Point3 {
    type Output = Point3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        let [a, b, c] = self.e;
        let [p, q, r] = rhs.e;
        Point3 {
            e: [a - p, b - q, c - r],
        }
    }
}

impl Sub<Point3> for Vec3 {
    type Output = Point3;

    fn sub(self, rhs: Point3) -> Self::Output {
        rhs - self
    }
}

impl Sub for Point3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        let [a, b, c] = self.e;
        let [p, q, r] = rhs.e;
        Vec3 {
            e: [a - p, b - q, c - r],
        }
    }
}

impl SubAssign<Vec3> for Point3 {
    fn sub_assign(&mut self, rhs: Vec3) {
        *self = *self - rhs;
    }
}
