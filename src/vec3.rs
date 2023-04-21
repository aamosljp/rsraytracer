use crate::utils::{clamp, random_float, random_floatmx};
use core::ops::{Add, Div, Mul, Sub};
use std::ops::Neg;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    e: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }
    pub fn x(&self) -> f64 {
        self.e[0]
    }
    pub fn y(&self) -> f64 {
        self.e[1]
    }
    pub fn z(&self) -> f64 {
        self.e[2]
    }
    pub fn len_squared(&self) -> f64 {
        self.e[0] * self.e[0] + self.e[1] * self.e[1] + self.e[2] * self.e[2]
    }
    pub fn len(&self) -> f64 {
        self.len_squared().sqrt()
    }
    pub fn random() -> Vec3 {
        Vec3 {
            e: [random_float(), random_float(), random_float()],
        }
    }
    pub fn random_range(min: f64, max: f64) -> Vec3 {
        Vec3 {
            e: [
                random_floatmx(min, max),
                random_floatmx(min, max),
                random_floatmx(min, max),
            ],
        }
    }
    pub fn unit_vector(&self) -> Vec3 {
        self / self.len()
    }
    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random_range(-1.0, 1.0);
            if p.len_squared() >= 1.0 {
                continue;
            }
            return p;
        }
    }
    pub fn random_unit_vector() -> Vec3 {
        Vec3::random_in_unit_sphere().unit_vector()
    }
    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let in_unit_sphere = Vec3::random_in_unit_sphere();
        if dot(in_unit_sphere, *normal) > 0.0 {
            in_unit_sphere
        } else {
            -in_unit_sphere
        }
    }
    pub fn near_zero(&self) -> bool {
        let s = 1e-8;
        (self.e[0].abs() < s) && (self.e[1].abs() < s) && (self.e[2].abs() < s)
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] * rhs, self.e[1] * rhs, self.e[2] * rhs],
        }
    }
}

impl Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] * rhs.e[0],
                self.e[1] * rhs.e[1],
                self.e[2] * rhs.e[2],
            ],
        }
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] + rhs.e[0],
                self.e[1] + rhs.e[1],
                self.e[2] + rhs.e[2],
            ],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl Div<f64> for &Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        Vec3 {
            e: [self.e[0] / rhs, self.e[1] / rhs, self.e[2] / rhs],
        }
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Sub<&Vec3> for &Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: &Vec3) -> Vec3 {
        Vec3 {
            e: [
                self.e[0] - rhs.e[0],
                self.e[1] - rhs.e[1],
                self.e[2] - rhs.e[2],
            ],
        }
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            e: [-self.e[0], -self.e[1], -self.e[2]],
        }
    }
}

pub type Point = Vec3;
pub type Color = Vec3;

pub fn dot(a: Vec3, b: Vec3) -> f64 {
    a.e[0] * b.e[0] + a.e[1] * b.e[1] + a.e[2] * b.e[2]
}

pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        e: [
            a.e[1] * b.e[2] - a.e[2] * b.e[1],
            a.e[2] * b.e[0] - a.e[0] * b.e[2],
            a.e[0] * b.e[1] - a.e[1] * b.e[0],
        ],
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    v / v.len()
}

pub fn write_color(pixel_color: Vec3, samples_per_pixel: i32) -> String {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();
    let scale = 1.0 / samples_per_pixel as f64;
    r = (r * scale).sqrt();
    g = (g * scale).sqrt();
    b = (b * scale).sqrt();
    String::from(format!(
        "{} {} {}",
        (256.0 * clamp(r, 0.0, 0.999)) as i32,
        (256.0 * clamp(g, 0.0, 0.999)) as i32,
        (256.0 * clamp(b, 0.0, 0.999)) as i32
    ))
}
