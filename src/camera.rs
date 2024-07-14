use crate::colour;
use crate::colour::Colour;
use crate::common::{self, degrees_to_radians, random_double_range};
use crate::hittable::Hittable;
use crate::hittable::*;
use crate::ray::Ray;
use crate::vec3::*;
use crate::vec3::{Point3, Vec3};

const IMAGE_WIDTH: i32 = 300;
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
const SAMPLES_PER_PIXEL: i32 = 100;
const MAX_DEPTH: i32 = 5;

pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    defocus_disk_basis_u: Vec3,
    defocus_disk_basis_v: Vec3,
    defocus_angle: f64,
}

impl Camera {
    pub fn new() -> Camera {
        let v_fov = 20.0;
        let v_fov_radians = degrees_to_radians(v_fov);
        let h = f64::tan(v_fov_radians / 2.0);

        let look_from = Point3::new(13.0, 2.0, 3.0);
        let look_at = Point3::new(0.0, 0.0, -1.0);
        let v_up = Vec3::new(0.0, 1.0, 0.0);

        // Defocus disk
        let defocus_angle = 0.6;
        let focus_distance = 10.0;

        // Basis vectors for camera coordinate frame
        let w = unit_vector(look_from - look_at);
        let u = unit_vector(v_up.cross(&w));
        let v = w.cross(&u);

        let viewport_height = 2.0 * h * focus_distance;
        let viewport_width = ASPECT_RATIO * viewport_height;

        let viewport_u = viewport_width * u;
        let viewport_v = viewport_height * v;

        let origin = look_from;
        let lower_left_corner = origin - viewport_u / 2.0 - viewport_v / 2.0 - w * focus_distance;

        // Calculate the camera defocus disk basis vectors.
        let defocus_radius = focus_distance * f64::tan(degrees_to_radians(defocus_angle / 2.0));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        Camera {
            origin,
            lower_left_corner,
            horizontal: viewport_u,
            vertical: viewport_v,
            defocus_disk_basis_u: defocus_disk_u,
            defocus_disk_basis_v: defocus_disk_v,
            defocus_angle,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.origin
        } else {
            self.defocus_disk_sample()
        };

        let ray_time = random_double_range(0.0, 1.0);

        Ray::new_tm(
            ray_origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - ray_origin,
            ray_time,
        )
    }

    fn defocus_disk_sample(&self) -> Vec3 {
        let p = random_in_unit_disk();
        self.origin + p.x() * self.defocus_disk_basis_u + p.y() * self.defocus_disk_basis_v
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
