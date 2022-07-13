use std::ops::{Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign};
use crate::math::random_double_in_range;
use crate::random_double;

// TODO: add generics
#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0
    };
    pub const FORWARD: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0
    };
    pub const BACK: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0
    };
    pub const UP: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0
    };
    pub const DOWN: Vec3 = Vec3 {
        x: 0.0,
        y: -1.0,
        z: 0.0
    };
    pub const LEFT: Vec3 = Vec3 {
        x: -1.0,
        y: 0.0,
        z: 0.0
    };
    pub const RIGHT: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0
    };
    pub const POSITIVE_INFINITY: Vec3 = Vec3 {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX
    };
    pub const NEGATIVE_INFINITY: Vec3 = Vec3 {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN
    };

    pub fn new(x: f64, y: f64, z: f64) -> Vec3{
        Vec3 { x, y, z}
    }
    pub fn from_float(val: f64) -> Vec3 {
        Vec3::new(val, val, val)
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.length()
    }
    //TODO: can this be the error?
    pub fn dot_product(a: &Vec3, b: &Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn cross_product(a: &Vec3, b: &Vec3) -> Vec3 {
        Vec3{
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x
        }
    }
    pub fn random() -> Vec3 {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_in_range(min: f64, max: f64) -> Vec3 {
        Vec3::new(random_double_in_range(min, max), random_double_in_range(min, max), random_double_in_range(min, max))
    }
    pub fn random_point_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.squared_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_point_in_unit_vector() -> Vec3 {
        Vec3::random_point_in_unit_sphere().unit_vector()
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {x: self.x + rhs.x, y: self.y + rhs.y, z: self.z + rhs.z}
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3 {x: self.x - rhs.x, y: self.y - rhs.y, z: self.z - rhs.z}
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Vec3 {x: self.x * rhs.x, y: self.y * rhs.y, z: self.z * rhs.z}
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3 {x: self.x * rhs, y: self.y * rhs, z: self.z * rhs}
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {x: self * rhs.x, y: self * rhs.y, z: self * rhs.z}
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {x: self.x / rhs.x, y: self.y / rhs.y, z: self.z / rhs.z}
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {x: self.x / rhs, y: self.y / rhs, z: self.z / rhs }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
        self.z *= rhs.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
        self.z /= rhs.z;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        return if index == 0 { &self.x } else if index == 1 { &self.y } else { &self.z }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        return if index == 0 { &mut self.x } else if index == 1 { &mut self.y } else { &mut self.z }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3{ x: -self.x, y: -self.y, z: -self.z }
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;