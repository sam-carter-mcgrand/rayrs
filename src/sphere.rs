use std::rc::Rc;

use crate::hittable::{HitRecord, Hittable};
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::Point3;

pub struct Sphere {
    centre: Point3,
    radius: f64,
    mat: Rc<dyn Material>,
}

impl Sphere {
    pub fn new(c: Point3, r: f64, m: Rc<dyn Material>) -> Sphere {
        Sphere {
            centre: c,
            radius: r,
            mat: m,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool {
        let oc = self.centre - ray.origin;
        let a = ray.direction.length_squared();
        let h = ray.direction.dot(&oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return false;
        }

        let sqrt_discriminant = f64::sqrt(discriminant);

        // find the nearest root that lies in the desired ray range
        let mut root = (h - sqrt_discriminant) / a;
        if root <= ray_tmin || root >= ray_tmax {
            root = (h + sqrt_discriminant) / a;
            if root <= ray_tmin || root >= ray_tmax {
                return false;
            }
        }

        rec.t = root;
        rec.p = ray.at(root);
        let outward_normal = (rec.p - self.centre) / self.radius;
        rec.set_face_normal(&ray, outward_normal);
        rec.mat = Some(self.mat.clone());
        true
    }
}
