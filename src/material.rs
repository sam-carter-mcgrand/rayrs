use crate::colour::Colour;
use crate::common::random_double;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3;

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
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + vec3::random_unit_vector();

        // if scatter is degenerate (zero) return some default
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *attenuation = self.albedo;
        *scattered = Ray::new_tm(rec.p, scatter_direction, r_in.tm);

        true
    }
}

pub struct Metal {
    albedo: Colour,
    fuzz: f64,
}

impl Metal {
    pub fn new(a: Colour, fuzz: f64) -> Metal {
        Metal {
            albedo: a,
            fuzz: if fuzz < 1.0 { fuzz } else { 1.0 },
        }
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
        *scattered = Ray::new_tm(
            rec.p,
            reflected + self.fuzz * vec3::random_in_unit_sphere(),
            r_in.tm,
        );
        scattered.direction.dot(&rec.normal) > 0.0
    }
}

pub struct Dialectric {
    // Relative refractive index compared to surrounding medium
    refractive_index: f64,
}

impl Dialectric {
    pub fn new(r: f64) -> Dialectric {
        Dialectric {
            refractive_index: r,
        }
    }

    fn reflectance(cosine: f64, refractive_index: f64) -> f64 {
        // Use Schlick approximation
        let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
        r0 + (1.0 - r0) * ((1.0 - cosine).powi(5))
    }
}

impl Material for Dialectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Colour,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Colour::new(1.0, 1.0, 1.0);
        let refractive_index = if rec.front_face {
            1.0 / self.refractive_index
        } else {
            self.refractive_index
        };
        let unit_direction = vec3::unit_vector(r_in.direction);
        let cos_theta = -unit_direction.dot(&rec.normal).min(1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta.powi(2));

        // No solution to Snell's law in the below case, ray must reflect instead
        let cannnot_refract = (refractive_index * sin_theta) > 1.0
            || Self::reflectance(cos_theta, refractive_index) > random_double();

        let result_ray = if cannnot_refract {
            vec3::reflect(unit_direction, rec.normal)
        } else {
            vec3::refract(unit_direction, rec.normal, refractive_index)
        };

        *scattered = Ray::new_tm(rec.p, result_ray, r_in.tm);
        true
    }
}
