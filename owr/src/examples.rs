use std::io;
use std::io::Write;
use std::sync::Arc;
use crate::{log_print, hittable};
use crate::vec3::Vec3;
use crate::hittable::{Hittable, HittableList};
use crate::sphere::{Sphere, MovingSphere};
use crate::utils;
use crate::sampling::{render_image};
use crate::material;
use crate::types::*;
use crate::camera;
use crate::texture;
use crate::bvh;

extern crate image;

// --------------------------------------------------------------------------------------------------------------------

pub fn run_and_print_ppm(params: &RaytracerParams, camera: &camera::Camera, world: &Arc<dyn Hittable>) {
    log_print!("P3\n{0} {1}\n255\n", params.image_width, params.image_height);

    let results = render_image(true, true, &params, &camera, world).unwrap().into_raw();
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

// --------------------------------------------------------------------------------------------------------------------

pub fn scene_select(scene_num : u32, image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, image: image::RgbaImage) -> (RaytracerParams, camera::Camera, HittableList) {
    match scene_num {
        0 => { first_weekend_example(image_width, image_height, samples_per_pixel, max_depth) }
        1 => { second_weekend_example_4dot4(image_width, image_height, samples_per_pixel, max_depth) }
        2 => { second_weekend_example_5dot1(image_width, image_height, samples_per_pixel, max_depth) }
        3 => { second_weekend_example_6dot2(image_width, image_height, samples_per_pixel, max_depth, image) }
        4 => { second_weekend_example_7dot4(image_width, image_height, samples_per_pixel, max_depth) }
        5 => { second_weekend_example_7dot6(image_width, image_height, samples_per_pixel, max_depth) }
        6 => { second_weekend_example_8dot0(image_width, image_height, samples_per_pixel, max_depth) }
        7 => { second_weekend_example_8dot2(image_width, image_height, samples_per_pixel, max_depth) }
        8 => { second_weekend_example_9dot1(image_width, image_height, samples_per_pixel, max_depth) }
        9 => { second_weekend_final_scene(image_width, image_height, samples_per_pixel, max_depth, image) }
        _ => { first_weekend_example(image_width, image_height, samples_per_pixel, max_depth) }
    }
}

// --------------------------------------------------------------------------------------------------------------------

pub fn first_weekend_example(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList) {

    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

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
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::new(0.70, 0.80, 1.00)
            );
        }
    
        camera
    }
    
    fn example_scene() -> HittableList {
        let mut world = HittableList::default();

        // Ground
        let checker =  Arc::new(texture::Checker::new_from_colors(&Vec3::new(0.2, 0.3, 0.1), &Vec3::new(0.9, 0.9, 0.9)));
        let ground_material = Arc::new(material::Lambertian { albedo: checker.clone() });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: ground_material }));

        for a in -11..11 {
            for b in -11..11 {
                let choose_mat = utils::det_random_float();
                let center = Vec3::new((a as Float) + 0.9*utils::det_random_float(), 0.2, (b as Float) + 0.9*utils::det_random_float());
                let radius = 0.2;

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_mat < 0.8 {
                        // Diffuse
                        let albedo = Arc::new(texture::SolidColor::new(&(utils::det_random_vec3() * utils::det_random_vec3())));
                        let material = Arc::new(material::Lambertian { albedo });
                        let center2 = center +  Vec3::new(0.0, utils::det_random_range(0.0,0.5), 0.0);
                        world.list.push(Arc::new(MovingSphere { center0: center, center1: center2, time0: 0.0, time1: 1.0, radius, material }));
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
            let material2 = Arc::new(material::Lambertian { albedo: Arc::new(texture::SolidColor::new(&Vec3::new(0.4, 0.2, 0.1)))});
            world.list.push(Arc::new(Sphere { center: Vec3::new(-4.0, 1.0, 0.0), radius: 1.0, material: material2 }));
        }

        {
            let material3 = Arc::new(material::Metal { albedo: Vec3::new(0.7, 0.6, 0.5), fuzz: 0.0});
            world.list.push(Arc::new(Sphere { center: Vec3::new(4.0, 1.0, 0.0), radius: 1.0, material: material3 }));
        }

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene())
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_4dot4(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(13.0, 2.0, 3.0);
            let look_at = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::new(0.70, 0.80, 1.00)
            );
        }
    
        camera
    }
    
    fn example_scene() -> HittableList {
        let mut world = HittableList::default();

        let checker =  Arc::new(texture::Checker::new_from_colors(&Vec3::new(0.2, 0.3, 0.1), &Vec3::new(0.9, 0.9, 0.9)));
        let material = Arc::new(material::Lambertian { albedo: checker.clone() });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, -10.0, 0.0), radius: 10.0, material: material.clone() }));
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0,  10.0, 0.0), radius: 10.0, material: material.clone() }));

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene())
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_5dot1(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(13.0, 2.0, 3.0);
            let look_at = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::new(0.70, 0.80, 1.00)
            );
        }
    
        camera
    }
    
    fn example_scene() -> HittableList {
        let mut world = HittableList::default();

        let noise_texture =  Arc::new(texture::Noise::new(4.0));
        let material = Arc::new(material::Lambertian { albedo: noise_texture.clone() });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: material.clone() }));
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0,  2.0, 0.0), radius: 2.0, material: material.clone() }));

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene())
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_6dot2(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, earth_image: image::RgbaImage) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(13.0, 2.0, 3.0);
            let look_at = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::new(0.70, 0.80, 1.00)
            );
        }
    
        camera
    }
    
    fn example_scene(image: image::RgbaImage) -> HittableList {
        let mut world = HittableList::default();

        let earth_texture =  Arc::new(texture::Image::new(image));
        let material = Arc::new(material::Lambertian { albedo: earth_texture.clone() });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0,  0.0, 0.0), radius: 2.0, material: material.clone() }));

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene(earth_image))
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_7dot4(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(13.0, 2.0, 3.0);
            let look_at = Vec3::new(0.0, 0.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::default()
            );
        }
    
        camera
    }
    
    fn example_scene() -> HittableList {
        let mut world = HittableList::default();

        let noise_texture =  Arc::new(texture::Noise::new(4.0));
        let material = Arc::new(material::Lambertian { albedo: noise_texture.clone() });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, -1000.0, 0.0), radius: 1000.0, material: material.clone() }));
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0,  2.0, 0.0), radius: 2.0, material: material.clone() }));

        let diff_light_texture = Arc::new(texture::SolidColor::new(&Vec3::new(4.0, 4.0, 4.0)));
        let diff_light = Arc::new(material::DiffuseLight::new(diff_light_texture));
        world.list.push(Arc::new(hittable::XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, diff_light.clone())));

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene())
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_7dot6(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(278.0, 278.0, -1250.0);
            let look_at = Vec3::new(278.0, 278.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::default()
            );
        }
    
        camera
    }
    
    fn example_scene() -> HittableList {
        let mut world = HittableList::default();

        let red_material = 
            Arc::new(material::Lambertian::new(
                Arc::new(texture::SolidColor::new(&Vec3::new(0.65, 0.05, 0.05)))));

        let white_material = 
            Arc::new(material::Lambertian::new(
                Arc::new(texture::SolidColor::new(&Vec3::new(0.73, 0.73, 0.73)))));

        let green_material = 
            Arc::new(material::Lambertian::new(
                Arc::new(texture::SolidColor::new(&Vec3::new(0.12, 0.45, 0.15)))));

        let diff_light_material = 
            Arc::new(material::DiffuseLight::new(
           Arc::new(texture::SolidColor::new(&Vec3::new(4.0, 4.0, 4.0)))));

        world.list.push(Arc::new(
            hittable::YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green_material.clone())));
        world.list.push(Arc::new(
            hittable::YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red_material.clone())));
        world.list.push(Arc::new(
            hittable::XZRect::new(213.0, 343.0, 227.0, 332.0, 554.0, diff_light_material.clone())));
        world.list.push(Arc::new(
            hittable::XZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, white_material.clone())));
        world.list.push(Arc::new(
            hittable::XZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));
        world.list.push(Arc::new(
            hittable::XYRect::new(0.0, 555.0, 0.0, 555.0, 555.0, white_material.clone())));

        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene())
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_8dot0(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    let mut ret = second_weekend_example_7dot6(image_width, image_height, samples_per_pixel, max_depth);

    let white_material = 
        Arc::new(material::Lambertian::new(
        Arc::new(texture::SolidColor::new(&Vec3::new(0.73, 0.73, 0.73)))));

    ret.2.list.push(
        Arc::new(hittable::Box::new(&Vec3::new(130.0, 0.0, 65.0), &Vec3::new(295.0, 165.0, 230.0), white_material.clone())));
    ret.2.list.push(
        Arc::new(hittable::Box::new(&Vec3::new(265.0, 0.0, 295.0), &Vec3::new(430.0, 330.0, 460.0), white_material.clone())));

    ret
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_8dot2(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    let mut ret = second_weekend_example_7dot6(image_width, image_height, samples_per_pixel, max_depth);

    let white_material = 
        Arc::new(material::Lambertian::new(
        Arc::new(texture::SolidColor::new(&Vec3::new(0.73, 0.73, 0.73)))));

    let box1 =
        Arc::new(hittable::Box::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 330.0, 165.0), white_material.clone()));
    let box1_rotated = Arc::new(hittable::RotateY::new(box1, 15.0));
    let box1_rotated_and_translated = Arc::new(hittable::Translate::new(box1_rotated, Vec3::new(265.0, 0.0, 295.0)));
    ret.2.list.push(box1_rotated_and_translated);

    let box2 =
        Arc::new(hittable::Box::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 165.0, 165.0), white_material.clone()));
    let box2_rotated = Arc::new(hittable::RotateY::new(box2, -18.0));
    let box2_rotated_and_translated = Arc::new(hittable::Translate::new(box2_rotated, Vec3::new(130.0, 0.0, 65.0)));
    ret.2.list.push(box2_rotated_and_translated);

    ret
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_example_9dot1(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> (RaytracerParams, camera::Camera, HittableList)  {
    let mut ret = second_weekend_example_7dot6(image_width, image_height, samples_per_pixel, max_depth);

    let white_material = 
        Arc::new(material::Lambertian::new(
        Arc::new(texture::SolidColor::new(&Vec3::new(0.73, 0.73, 0.73)))));

    let box1 =
        Arc::new(hittable::Box::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 330.0, 165.0), white_material.clone()));
    let box1_rotated = Arc::new(hittable::RotateY::new(box1, 15.0));
    let box1_rotated_and_translated = Arc::new(hittable::Translate::new(box1_rotated, Vec3::new(265.0, 0.0, 295.0)));

    let box2 =
        Arc::new(hittable::Box::new(&Vec3::new(0.0, 0.0, 0.0), &Vec3::new(165.0, 165.0, 165.0), white_material.clone()));
    let box2_rotated = Arc::new(hittable::RotateY::new(box2, -18.0));
    let box2_rotated_and_translated = Arc::new(hittable::Translate::new(box2_rotated, Vec3::new(130.0, 0.0, 65.0)));
    
    ret.2.list.push(Arc::new (
        hittable::ConstantMedium::new_with_constant_color(box1_rotated_and_translated, 0.01, &Vec3::new(0.0, 0.0, 0.0))));
    ret.2.list.push(Arc::new (
        hittable::ConstantMedium::new_with_constant_color(box2_rotated_and_translated, 0.01, &Vec3::new(1.0, 1.0, 1.0))));

    ret
}

// --------------------------------------------------------------------------------------------------------------------

pub fn second_weekend_final_scene(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32, earth_image: image::RgbaImage) -> (RaytracerParams, camera::Camera, HittableList)  {
    fn example_params(image_width: u32, image_height: u32, samples_per_pixel: u32, max_depth: u32) -> RaytracerParams {
        RaytracerParams {
            aspect_ratio: (image_width as Float) / (image_height as Float),
            image_width,
            image_height,
            samples_per_pixel,
            max_depth,
        }
    }

    pub fn example_camera(aspect_ratio: Float) -> camera::Camera {
        let camera;
        {
            let look_from = Vec3::new(478.0, 278.0, -600.0);
            let look_at = Vec3::new(278.0, 278.0, 0.0);
            let up = Vec3::new(0.0, 1.0, 0.0);
            let focus_dist = 10.0;
            let aperture = 0.0;
    
            camera = camera::Camera::new(
                &look_from,
                &look_at,
                &up,
                45.0,
                aspect_ratio,
                aperture,
                focus_dist,
                0.0,
                1.0,
                Vec3::default()
            );
        }
    
        camera
    }
    
    fn example_scene(earth_image: image::RgbaImage) -> HittableList {
        let mut world = HittableList::default();

        let ground = Arc::new(material::Lambertian::new(
            Arc::new(texture::SolidColor::new(&Vec3::new(0.48, 0.83, 0.53)))));

        // Random ground boxes
        let boxes_per_side = 20;
        let mut boxes1 = HittableList::default();
        for i in 0..boxes_per_side {
            for j in 0..boxes_per_side {
                let w = 100.0;
                let x0 = -1000.0 + (i as Float * w);
                let z0 = -1000.0 + (j as Float * w);
                let y0 = 0.0;
                let x1 = x0 + w;
                let y1 = utils::det_random_range(1.0, 101.0);
                let z1 = z0 + w;

                boxes1.list.push(
                    Arc::new(hittable::Box::new(&Vec3::new(x0, y0, z0), &Vec3::new(x1, y1, z1), ground.clone())));
            }
        }
        // Add ground boxes with a bvh wrapper
        world.list.push(bvh::BvhNode::build_bvh(&boxes1, 0.0, 1.0));

        // Light
        let diff_light_material = 
            Arc::new(material::DiffuseLight::new(
                Arc::new(texture::SolidColor::new(&Vec3::new(7.0, 7.0, 7.0)))));
        world.list.push(Arc::new(
        hittable::XZRect::new(123.0, 423.0, 147.0, 412.0, 554.0, diff_light_material.clone())));

        // Moving sphere
        let center1 = Vec3::new(400.0, 400.0, 200.0);
        let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
        let moving_sphere_material = Arc::new(material::Lambertian::new(
            Arc::new(texture::SolidColor::new(&Vec3::new(0.7, 0.3, 0.1)))));
        world.list.push(Arc::new(MovingSphere { center0: center1, center1: center2, time0: 0.0, time1: 1.0, radius: 50.0, material: moving_sphere_material }));

        // Dielectric
        let dielectric_material = Arc::new(material::Dielectric { index_of_refraction: 1.5 });
        world.list.push(Arc::new(Sphere { center: Vec3::new(260.0, 150.0, 145.0), radius: 50.0, material: dielectric_material.clone() }));

        // Metal sphere
        let metal_material = Arc::new(material::Metal { albedo: Vec3::new(0.8, 0.8, 0.9), fuzz: 1.0 });
        world.list.push(Arc::new(Sphere { center: Vec3::new(0.0, 150.0, 145.0), radius: 50.0, material: metal_material.clone() }));

        // Blue sphere (constant medium with dielectric)
        let boundary = Arc::new(Sphere { center: Vec3::new(360.0, 150.0, 145.0), radius: 70.0, material: dielectric_material.clone() });
        world.list.push(boundary.clone());
        world.list.push(Arc::new(hittable::ConstantMedium::new_with_constant_color(boundary.clone(), 0.2, &Vec3::new(0.2, 0.4, 0.9))));

        // Constant medium
        let boundary2 = Arc::new(Sphere { center: Vec3::new(0.0, 0.0, 0.0), radius: 5000.0, material: dielectric_material.clone() });
        world.list.push(Arc::new(hittable::ConstantMedium::new_with_constant_color(boundary2.clone(), 0.0001, &Vec3::new(1.0, 1.0, 1.0))));

        // Earth sphere
        let earth_material = Arc::new(material::Lambertian { albedo: Arc::new(texture::Image::new(earth_image))});
        world.list.push(Arc::new(Sphere { center: Vec3::new(400.0,  200.0, 400.0), radius: 100.0, material: earth_material.clone() }));

        // Perlin sphere
        let perlin_material = Arc::new(material::Lambertian { albedo: Arc::new(texture::Noise::new(0.1)) });
        world.list.push(Arc::new(Sphere { center: Vec3::new(220.0, 280.0, 300.0), radius: 80.0, material: perlin_material.clone() }));

        // A bunch of random spheres
        let mut boxes2 = HittableList::default();
        let white_material = Arc::new(material::Lambertian::new(
                Arc::new(texture::SolidColor::new(&Vec3::new(0.73, 0.73, 0.73)))));
        let num_spheres = 1000;
        for _ in 0..num_spheres {
            boxes2.list.push(Arc::new(Sphere { center: utils::det_random_range_vec3(0.0, 165.0), radius: 10.0, material: white_material.clone() }));
        }
        // Wrap to bvh
        let bvh_spheres = bvh::BvhNode::build_bvh(&boxes2, 0.0, 1.0);
        // Rotate it and translate it
        let rotated_spheres = Arc::new(hittable::RotateY::new(bvh_spheres, 15.0));
        let final_spheres = Arc::new(hittable::Translate::new(rotated_spheres, Vec3::new(-100.0, 270.0, 395.0)));
        // Finall add
        world.list.push(final_spheres);

        // Wow, finally return the world
        world
    }

    // Return
    let params = example_params(image_width, image_height, samples_per_pixel, max_depth);
    (params, example_camera(params.aspect_ratio), example_scene(earth_image))
}