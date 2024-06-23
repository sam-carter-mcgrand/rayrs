use std::rc::Rc;

use ray_tracing::camera::Camera;
use ray_tracing::colour::Colour;
use ray_tracing::hittable::*;
use ray_tracing::material::{Lambertian, Metal};
use ray_tracing::sphere::Sphere;
use ray_tracing::vec3::*;

fn main() {
    let mut world = HittableList::new();
    let material_ground = Rc::new(Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Colour::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Colour::new(0.8, 0.8, 0.8)));
    let material_right = Rc::new(Metal::new(Colour::new(0.8, 0.6, 0.2)));

    world.add(Box::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = Camera::new();
    cam.render(&world);
}
