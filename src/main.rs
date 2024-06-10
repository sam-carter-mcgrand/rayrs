use ray_tracing::colour;
use ray_tracing::colour::Colour;
use ray_tracing::ray::Ray;
use ray_tracing::vec3::*;

fn ray_color(r: Ray) -> Colour {
    let sphere_centre = Point3::new(0.0, 0.0, -1.0);
    let ray_time = sphere_ray_intersects(sphere_centre, 0.5, &r);

    let unit_direction = unit_vector(r.direction);
    let t = (1.0 - 0.5) * (unit_direction.y() + 1.0);

    if ray_time > 0.0 {
        let normal = unit_vector(r.at(ray_time) - sphere_centre);
        let translated_normal = 0.5 * (normal + 1.0);
        Colour::new(translated_normal.x(), translated_normal.y(), translated_normal.z())
    } else {
        (1.0 - t) * Colour::new(1.0, 1.0, 1.0) + t * Colour::new(0.5, 0.7, 1.0)
    }
}

fn sphere_ray_intersects(centre: Point3, radius: f64, ray: &Ray) -> f64 {

    let oc = centre - ray.origin;
    let a = ray.direction.length_squared();
    let b = -2.0 * ray.direction.dot(oc);
    let c = oc.length_squared() - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    }
    else {
        (-b - f64::sqrt(discriminant)) / (2.0 * a)
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (400 as f64 / ASPECT_RATIO) as i32;

    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScan lines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let u = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel_colour = ray_color(r);
            colour::write_colour(&mut std::io::stdout(), pixel_colour);
        }
    }
    eprintln!("\nDone!");
}
