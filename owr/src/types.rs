// --------------------------------------------------------------------------------------------------------------------

pub type Float = f32;
pub type Color = [Float; 4];

// --------------------------------------------------------------------------------------------------------------------

#[derive(Copy, Clone, Default)]
pub struct RaytracerParams {
    pub aspect_ratio: Float,
    pub image_width: u32,
    pub image_height: u32,
    pub samples_per_pixel: u32,
    pub max_depth: u32
}
