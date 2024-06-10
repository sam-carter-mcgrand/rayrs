use std::fmt::{Display, Formatter};
use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    f: [f64; 3],
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Vec3 {
        Vec3 { f: [x, y, z] }
    }

    pub fn x(self) -> f64 {
        self.f[0]
    }
    pub fn y(self) -> f64 {
        self.f[1]
    }
    pub fn z(self) -> f64 {
        self.f[2]
    }

    pub fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    pub fn length_squared(self) -> f64 {
        self.x() * self.x() + self.y() * self.y() + self.z() * self.z()
    }
    pub fn dot(self, rhs: Self) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z()
    }
}

pub type Point3 = Vec3;

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.f[0], self.f[1], self.f[2])
    }
}

impl Neg for Vec3 {
    type Output = Vec3;
    fn neg(self) -> Vec3 {
        Vec3 {
            f: [-self.x(), -self.y(), -self.z()],
        }
    }
}

impl Add<f64> for Vec3 {
    type Output = Vec3;
    fn add(self, r: f64) -> Vec3 {
        Vec3 {
            f: [self.x() + r, self.y() + r, self.z() + r],
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, r: Vec3) -> Vec3 {
        Vec3 {
            f: [self.x() + r.x(), self.y() + r.y(), self.z() + r.z()],
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, r: Vec3) -> Vec3 {
        Vec3 {
            f: [self.x() - r.x(), self.y() - r.y(), self.z() - r.z()],
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, r: f64) -> Vec3 {
        Vec3 {
            f: [self.x() * r, self.y() * r, self.z() * r],
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, r: Vec3) -> Vec3 {
        Vec3 {
            f: [r.x() * self, r.y() * self, r.z() * self],
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, r: f64) -> Vec3 {
        Vec3 {
            f: [self.x() / r, self.y() / r, self.z() / r],
        }
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    v / v.length()
}
