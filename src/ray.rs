use crate::vec3::Point3;
use crate::vec3::Vec3;

#[derive(Default)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub tm: f64,
}

impl Ray {
    pub fn new_tm(origin: Vec3, direction: Vec3, tm: f64) -> Ray {
        Ray {
            origin,
            direction,
            tm,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.origin + t * self.direction
    }
}
