use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

use crate::math::random_double_in_range;
use crate::random_double;

// TODO: add generics
#[derive(Debug, Clone, Copy, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub const ZERO: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 0.0,
    };
    pub const ONE: Vec3 = Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    };
    pub const FORWARD: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: 1.0,
    };
    pub const BACK: Vec3 = Vec3 {
        x: 0.0,
        y: 0.0,
        z: -1.0,
    };
    pub const UP: Vec3 = Vec3 {
        x: 0.0,
        y: 1.0,
        z: 0.0,
    };
    pub const DOWN: Vec3 = Vec3 {
        x: 0.0,
        y: -1.0,
        z: 0.0,
    };
    pub const LEFT: Vec3 = Vec3 {
        x: -1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const RIGHT: Vec3 = Vec3 {
        x: 1.0,
        y: 0.0,
        z: 0.0,
    };
    pub const POSITIVE_INFINITY: Vec3 = Vec3 {
        x: f64::MAX,
        y: f64::MAX,
        z: f64::MAX,
    };
    pub const NEGATIVE_INFINITY: Vec3 = Vec3 {
        x: f64::MIN,
        y: f64::MIN,
        z: f64::MIN,
    };

    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn new_away_from_camera(x: f64, y: f64, z: f64) -> Self {
        Self { x: -x, y, z }
    }
    pub fn from_float(val: f64) -> Self {
        Vec3::new(val, val, val)
    }
    pub fn length(&self) -> f64 {
        self.squared_length().sqrt()
    }
    pub fn squared_length(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn unit_vector(&self) -> Self {
        self / self.length()
    }
    pub fn dot_product(a: &Vec3, b: &Vec3) -> f64 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }
    pub fn cross_product(a: &Vec3, b: &Vec3) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }
    pub fn random() -> Self {
        Vec3::new(random_double(), random_double(), random_double())
    }
    pub fn random_in_range(min: f64, max: f64) -> Self {
        Vec3::new_away_from_camera(
            random_double_in_range(min, max),
            random_double_in_range(min, max),
            random_double_in_range(min, max),
        )
    }
    pub fn random_point_in_unit_sphere() -> Self {
        loop {
            let p = Vec3::random_in_range(-1.0, 1.0);
            if p.squared_length() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_point_in_unit_vector() -> Self {
        Vec3::random_point_in_unit_sphere().unit_vector()
    }
    pub fn random_point_in_hemisphere(normal: Vec3) -> Self {
        let in_unit_sphere = Vec3::random_point_in_unit_sphere();
        if Vec3::dot_product(&in_unit_sphere, &normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn reflect(vector: Vec3, normal: Vec3) -> Self {
        vector - 2.0 * Vec3::dot_product(&vector, &normal) * normal
    }
    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Self {
        let cos_theta = Vec3::dot_product(&-uv, &normal).min(1.0_f64);
        let ray_out_perpendicular = etai_over_etat * (uv + cos_theta * normal);
        let ray_out_parallel = (1.0 - ray_out_perpendicular.squared_length()).abs().sqrt() * normal;
        ray_out_perpendicular + ray_out_parallel
    }
    pub fn near_zero(&self) -> bool {
        const S: f64 = 1e-8;
        (self.x.abs() < S) && (self.y.abs() < S) && (self.z.abs() < S)
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Vec3 {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
            z: self.z / rhs.z,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        Vec3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
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
        if index == 0 {
            &self.x
        } else if index == 1 {
            &self.y
        } else {
            &self.z
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index == 0 {
            &mut self.x
        } else if index == 1 {
            &mut self.y
        } else {
            &mut self.z
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

pub type Color = Vec3;
pub type Point3 = Vec3;
