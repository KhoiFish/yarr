use std::io;
use std::io::Write;
use raytracer::{ray_color, random_scene};
use raytracer::{vec3::Vec3, camera::Camera, utils, color, color::Color64};

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
