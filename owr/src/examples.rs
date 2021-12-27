use std::io;
use std::io::Write;
use std::sync::Arc;
use crate::{log_print};
use crate::vec3::Vec3;
use crate::hittable::{HittableList};
use crate::sphere::Sphere;
use crate::utils;
use crate::sampling::{render_image};
use crate::material;
use crate::types::*;
use crate::camera;

// --------------------------------------------------------------------------------------------------------------------

pub fn example_params() -> RaytracerParams {
    let aspect_ratio = 3.0 / 2.0;
    let image_width = 320;

    RaytracerParams {
        aspect_ratio,
        image_width,
        image_height: ((image_width as Float) / aspect_ratio) as u32,
        samples_per_pixel: 4,
        max_depth: 32
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
    let camera;
    {
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let up = Vec3::new(0.0, 1.0, 0.0);
        let focus_dist = 10.0;
        let aperture = 0.1;

        camera = camera::Camera::new(
            &look_from,
            &look_at,
            &up,
            90.0,
            aspect_ratio,
            aperture,
            focus_dist
        );
    }

    camera
}

// --------------------------------------------------------------------------------------------------------------------

pub fn first_weekend_scene() -> HittableList {
    let mut world = HittableList::default();

    // Ground
    let ground_material = Arc::new(material::Lambertian { albedo: Vec3::new(0.5, 0.5, 0.5) });
    world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: ground_material }));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = utils::det_random_float();
            let center = Vec3::new((a as Float) + 0.9*utils::det_random_float(), 0.2, (b as Float) + 0.9*utils::det_random_float());
            let radius = 0.2;

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = utils::det_random_vec3() * utils::det_random_vec3();
                    let material = Arc::new(material::Lambertian { albedo });
                    world.list.push(Arc::new(Sphere { center, radius, material }));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = utils::det_random_range_vec3(0.5, 1.0);
                    let fuzz = utils::det_random_range(0.0, 0.5);
                    let material = Arc::new(material::Metal { albedo, fuzz });
                    world.list.push(Arc::new(Sphere { center, radius, material }));
                } else {
                    // Glass
                    let material = Arc::new(material::Dielectric { index_of_refraction: 1.5 });
                    world.list.push(Arc::new(Sphere { center, radius, material }));
                }
            }
        }
    }

    {
        let material1 = Arc::new(material::Dielectric { index_of_refraction: 1.5 });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, 1.0, 0.0), radius: 1.0, material: material1 }));
    }

    {
        let material2 = Arc::new(material::Lambertian { albedo: Vec3::new(0.4, 0.2, 0.1)});
        world.list.push(Arc::new(Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));
    }

    {
        let material3 = Arc::new(material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0});
        world.list.push(Arc::new(Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));
    }

    world
}

// --------------------------------------------------------------------------------------------------------------------

pub fn run_and_print_ppm(params: &RaytracerParams, camera: &camera::Camera, world: &HittableList) {
    log_print!("P3\n{0} {1}\n255\n", params.image_width, params.image_height);

    let results = render_image(true, &params, &camera, &world);
    let mut count = 0;
    for &color in &results {
        count = count + 1;
        if (count % 4) == 0 {
            log_print!("\n");
        } else {
            log_print!("{0} ", color);
        }
    }

    io::stdout().flush().unwrap();
}
