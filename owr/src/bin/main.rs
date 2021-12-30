use owr::examples::*;
use owr::sampling::*;

// --------------------------------------------------------------------------------------------------------------------

pub fn main() {
    let example_scene = first_weekend_example(576, 1024, 32, 50);
    let final_image = render_image(true, &example_scene.0, &example_scene.1, &example_scene.2);
    final_image.unwrap().save("output.png").unwrap();
}
