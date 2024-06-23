use crate::colour;
use crate::colour::Colour;
use crate::common;
use crate::hittable::Hittable;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;
use crate::vec3::{Point3, Vec3};

const IMAGE_WIDTH: i32 = 1280;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 10;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let viewport_height = 2.0;
        let viewport_width = ASPECT_RATIO * viewport_height;
        let focal_length = 1.0;

        let origin = Point3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical,
        )
    }

    fn ray_colour(r: &Ray, world: &dyn Hittable, depth: i32) -> Colour {
        let mut rec = HitRecord::new();

        if depth <= 0 {
            return Colour::new(0.0, 0.0, 0.0);
        }

        if world.hit(r, 0.001, common::INFINITY, &mut rec) {
            let mut attenuation = Colour::default();
            let mut scattered = Ray::default();
            if rec
                .mat
                .as_ref()
                .unwrap()
                .scatter(r, &rec, &mut attenuation, &mut scattered)
            {
                return Self::ray_colour(&scattered, world, depth - 1) * attenuation;
            }
            return Colour::new(0.0, 0.0, 0.0);
        }

        let unit_direction = unit_vector(r.direction);
        let t = (1.0 - 0.5) * (unit_direction.y() + 1.0);

        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }

    pub fn render(&self, world: &dyn Hittable) {
        println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

        for j in (0..IMAGE_HEIGHT).rev() {
            eprint!("\rScan lines remaining: {}", j);
            for i in 0..IMAGE_WIDTH {
                let mut pixel_colour = Colour::new(0.0, 0.0, 0.0);

                for _ in 0..SAMPLES_PER_PIXEL {
                    let u = (i as f64 + common::random_double()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (j as f64 + common::random_double()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = self.get_ray(u, v);

                    pixel_colour += Self::ray_colour(&r, world, MAX_DEPTH);
                }

                colour::write_colour(&mut std::io::stdout(), pixel_colour, SAMPLES_PER_PIXEL);
            }
        }
        eprintln!("\nDone!");
    }
}
