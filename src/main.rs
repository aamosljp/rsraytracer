pub mod camera;
pub mod object;
pub mod ray;
pub mod utils;
pub mod vec3;
use crate::camera::Camera;
use crate::object::{Metal, Lambertian, Dielectric, Object, ObjectList, Sphere};
use crate::ray::Ray;
use crate::utils::random_float;
use crate::vec3::{dot, unit_vector, write_color, Color, Vec3};

fn ray_color(r: &Ray, world: &dyn Object, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let rec = world.hit(r, 0.001, std::f64::INFINITY);
    match rec {
        Some(rec) => {
            match rec.material.scatter(r, &rec) {
                Some((attenuation, scattered)) => attenuation * ray_color(&scattered, world, depth - 1),
                None => Color::new(0.0, 0.0, 0.0),
            }
        }
        None => {
            let unit_direction = unit_vector(&r.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);
            Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;
    const SAMPLES_PER_PIXEL: i32 = 50;
    const MAX_DEPTH: i32 = 30;
    let mut world = ObjectList::new();
    let material_ground = Box::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Box::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Box::new(Dielectric {
        ref_idx: 1.5,
    });
    let material_right = Box::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        material: material_ground,
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        material: material_center,
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_left.clone(),
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(-1.0, 0.0, -1.0),
        radius: -0.4,
        material: material_left,
    }));
    world.push(Box::new(Sphere {
        center: Vec3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        material: material_right,
    }));
    let cam = Camera::new();
    println!("P3\n{IMAGE_WIDTH} {IMAGE_HEIGHT}\n255");
    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scalines remaining: {j}");
        for i in 0..IMAGE_WIDTH {
            let mut color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..SAMPLES_PER_PIXEL {
                let u = (i as f64 + random_float()) / (IMAGE_WIDTH - 1) as f64;
                color = color + ray_color(&r, &world, MAX_DEPTH);
            }
            println!("{}", write_color(color, SAMPLES_PER_PIXEL));
        }
    }
}
