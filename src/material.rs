use crate::colour::Colour;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool;
}

pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(a: Colour) -> Lambertian {
        Lambertian { albedo: a }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // if scatter is degenerate (zero) return some default
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, scatter_direction);

        true
    }
}

pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(a: Colour) -> Metal {
        Metal { albedo: a }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = vec3::reflect(vec3::unit_vector(r_in.direction), rec.normal);

        *attenuation = self.albedo;
        *scattered = Ray::new(rec.p, reflected);
        scattered.direction.dot(&rec.normal) > 0.0
    }
}