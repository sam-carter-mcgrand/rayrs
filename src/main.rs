use std::rc::Rc;

use ray_tracing::camera::Camera;
use ray_tracing::colour::{self, Colour};
use ray_tracing::common::{random_double, random_double_range};
use ray_tracing::hittable::*;
use ray_tracing::material::{Dialectric, Lambertian, Metal};
use ray_tracing::sphere::Sphere;
use ray_tracing::vec3;
use ray_tracing::vec3::*;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        material_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let centre = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (centre - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = colour::random() * colour::random();
                    let material = Rc::new(Lambertian::new(albedo));
                    // add movement with a second centre point at t = 1
                    let centre2 = centre + Point3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Box::new(Sphere::new_moving(centre, centre2, 0.2, material)));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = vec3::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    let material = Rc::new(Metal::new(albedo, fuzz));
                    world.add(Box::new(Sphere::new(centre, 0.2, material)));
                } else {
                    // glass
                    let material = Rc::new(Dialectric::new(1.5));
                    world.add(Box::new(Sphere::new(centre, 0.2, material)));
                }
            }
        }
    }

    let material1 = Rc::new(Dialectric::new(1.5));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let cam = Camera::new();
    cam.render(&world);
}
