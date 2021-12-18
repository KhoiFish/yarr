mod color;
mod ray;
mod vec3;
mod hittable;
mod sphere;
mod camera;
mod utils;
mod material;

use std::rc::Rc;
use std::io;
use std::io::Write;
use color::Color64;
use ray::Ray;
use vec3::Vec3;
use hittable::{Hittable, HittableList};
use sphere::Sphere;
use camera::{Camera};

// --------------------------------------------------------------------------------------------------------------------

fn random_scene() -> HittableList {
    let mut world = HittableList::default();

    // Ground
    let ground_material = Rc::new(material::Lambertian { albedo: Color64::new(0.5, 0.5, 0.5) });
    world.list.push(Rc::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: ground_material }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::random_float();
            let center = Vec3::new((a as f64) + 0.9*utils::random_float(), 0.2, (b as f64) + 0.9*utils::random_float());
            let radius = 0.2;

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = color::multiply_colors(&color::random(), &color::random());
                    let material = Rc::new(material::Lambertian { albedo });
                    world.list.push(Rc::new(Sphere { center, radius, material }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = color::random_range(0.5, 1.0);
                    let fuzz = utils::random_range(0.0, 0.5);
                    let material = Rc::new(material::Metal { albedo, fuzz });
                    world.list.push(Rc::new(Sphere { center, radius, material }));
                } else {
                    // Glass
                    let material = Rc::new(material::Dielectric { index_of_refraction: 1.5 });
                    world.list.push(Rc::new(Sphere { center, radius, material }));
                }
            }
        }
    }

    {
        let material1 = Rc::new(material::Dielectric { index_of_refraction: 1.5 });
        world.list.push(Rc::new(Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: material1 }));
    }

    {
        let material2 = Rc::new(material::Lambertian { albedo: Color64::new(0.4, 0.2, 0.1)});
        world.list.push(Rc::new(Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));
    }

    {
        let material3 = Rc::new(material::Metal { albedo: Color64::new(0.7, 0.6, 0.5), fuzz: 0.0});
        world.list.push(Rc::new(Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));
    }

    world
}

// --------------------------------------------------------------------------------------------------------------------

fn ray_color(r : &Ray<f64>, world: &HittableList, depth: u32) -> Color64 {
    if depth <= 0 {
        return Color64::default();
    }

    match world.hit(&r, 0.001, f64::MAX) {
        Some(hit) => {
            match hit.material.scatter(&r, &hit) {
                Some(scatter_result) => {
                    return color::multiply_colors(
                        &ray_color(&scatter_result.scattered, world, depth-1),
                        &scatter_result.attenuation
                    );
                }
                _ => {
                    return Color64::default();
                }
            }
        }
        _ => {}
    }

    let unit_direction = r.dir.unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);

    (Color64::new(1.0, 1.0, 1.0) * (1.0 - t)) + (Color64::new(0.5, 0.7, 1.0) * t)
}

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    // Image
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 320_i32;
    let image_height = ((image_width as f64) / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let max_depth = 50;

    // World
    let world = random_scene();

    // Camera
    let camera;
    {
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let dist_to_focus = 10.0;
        let aperture = 0.1;

        camera = Camera::new(
            &look_from,
            &look_at,
            &up,
            90.0,
            aspect_ratio,
            aperture,
            dist_to_focus
        );
    }

    print!("P3\n{0} {1}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let mut pixel_color = Color64::default();
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + utils::random_range(0.0, 1.0)) / ((image_width - 1) as f64);
                let v = ((j as f64) + utils::random_range(0.0, 1.0)) / ((image_height - 1) as f64);
                let r = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
            }
            color::write_color64(&pixel_color, samples_per_pixel);
        }
    }
    io::stdout().flush().unwrap();
}
