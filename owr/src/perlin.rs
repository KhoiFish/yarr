use crate::types::*;
use crate::utils;
use crate::vec3::Vec3;

// --------------------------------------------------------------------------------------------------------------------

const POINT_COUNT: usize = 256;

// --------------------------------------------------------------------------------------------------------------------

pub struct Perlin {
    perm_x: Vec<u32>,
    perm_y: Vec<u32>,
    perm_z: Vec<u32>,
    random_floats: Vec<Float>,
    random_vec: Vec::<Vec3<Float>>
}

impl Perlin {
    pub fn new() -> Self {
        let mut random_floats = Vec::with_capacity(POINT_COUNT);
        for _ in 0..POINT_COUNT {
            random_floats.push(utils::random_float());
        }

        let mut random_vec = Vec::with_capacity(POINT_COUNT);
        for _i in 0..POINT_COUNT {
            random_vec.push(utils::random_range_vec3(-1.0, 1.0).unit_vector());
        }

        let perm_x = Perlin::perlin_generate_perm();
        let perm_y = Perlin::perlin_generate_perm();
        let perm_z = Perlin::perlin_generate_perm();

        Self {
            perm_x,
            perm_y,
            perm_z,
            random_floats,
            random_vec
        }
    }

    pub fn turb(&self, p: &Vec3<Float>, depth: u32) -> Float {
        let mut accum = 0.0;
        let mut temp_p = p.clone();
        let mut weight = 1.0;

        for _ in 0..depth {
            accum += weight * self.smoother_noise(&temp_p);
            weight *= 0.5;
            temp_p = temp_p * 2.0;
        }

        accum.abs()
    }

    pub fn smoother_noise(&self, point: &Vec3<Float>) -> Float {
        let mut u = point.x() - Float::floor(point.x());
        let mut v = point.y() - Float::floor(point.y());
        let mut w = point.z() - Float::floor(point.z());

        // Hermite cubic
        u = u * u * (3.0 - 2.0*u);
        v = v * v * (3.0 - 2.0*v);
        w = w * w * (3.0 - 2.0*w);

        let i = Float::floor(point.x()) as i32;
        let j = Float::floor(point.y()) as i32;
        let k = Float::floor(point.z()) as i32;
        
        let mut c : [[[Vec3<Float>; 2]; 2]; 2] = [[[Vec3::<Float>::default(); 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_vec[
                        ( self.perm_x[((i+di as i32) & 255) as usize] ^
                          self.perm_y[((j+dj as i32) & 255) as usize] ^
                          self.perm_z[((k+dk as i32) & 255) as usize] ) as usize
                    ]
                }
            }
        }

        Perlin::perlin_interp(&c, u, v, w)
    }


    pub fn smooth_noise(&self, point: &Vec3<Float>) -> Float {
        let mut u = point.x() - Float::floor(point.x());
        let mut v = point.y() - Float::floor(point.y());
        let mut w = point.z() - Float::floor(point.z());

        // Hermite cubic
        u = u * u * (3.0 - 2.0*u);
        v = v * v * (3.0 - 2.0*v);
        w = w * w * (3.0 - 2.0*w);

        let i = Float::floor(point.x()) as i32;
        let j = Float::floor(point.y()) as i32;
        let k = Float::floor(point.z()) as i32;
        
        let mut c : [[[Float; 2]; 2]; 2] = [[[0.0; 2]; 2]; 2];
        for di in 0..2 {
            for dj in 0..2 {
                for dk in 0..2 {
                    c[di][dj][dk] = self.random_floats[
                        ( self.perm_x[((i+di as i32) & 255) as usize] ^
                          self.perm_y[((j+dj as i32) & 255) as usize] ^
                          self.perm_z[((k+dk as i32) & 255) as usize] ) as usize
                    ]
                }
            }
        }

        Perlin::trilinear_interp(&c, u, v, w)
    }

    pub fn blocky_noise(&self, point: &Vec3<Float>) -> Float {
        let i = ((4.0 * point.x()) as i32 & 255) as usize;
        let j = ((4.0 * point.y()) as i32 & 255) as usize;
        let k = ((4.0 * point.z()) as i32 & 255) as usize;
        let ei = (self.perm_x[i] ^ self.perm_y[j] ^ self.perm_z[k]) as usize;

        self.random_floats[ei]
    }

    fn perlin_interp(c: &[[[Vec3<Float>; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
        let uu = u * u * (3.0 - 2.0*u);
        let vv = v * v * (3.0 - 2.0*v);
        let ww = w * w * (3.0 - 2.0*w);

        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let weight_v = Vec3::new(u - i as Float, v - j as Float, w - k as Float);
                    accum += ((i as Float * uu) + (1.0 - i as Float) * (1.0 - uu as Float)) *
                             ((j as Float * vv) + (1.0 - j as Float) * (1.0 - vv as Float)) *
                             ((k as Float * ww) + (1.0 - k as Float) * (1.0 - ww as Float)) * 
                             Vec3::dot(&c[i][j][k], &weight_v);
                }
            }
        }

        accum
    }

    fn trilinear_interp(c: &[[[Float; 2]; 2]; 2], u: Float, v: Float, w: Float) -> Float {
        let mut accum = 0.0;
        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    accum += ((i as Float * u) + (1.0 - i as Float) * (1.0 - u as Float)) *
                             ((j as Float * v) + (1.0 - j as Float) * (1.0 - v as Float)) *
                             ((k as Float * w) + (1.0 - k as Float) * (1.0 - w as Float)) * 
                             c[i][j][k];
                }
            }
        }

        accum
    }

    fn perlin_generate_perm() -> Vec<u32> {
        let mut permuted_values = Vec::with_capacity(POINT_COUNT);
        for i in 0..POINT_COUNT {
            permuted_values.push(i as u32);
        }
        Perlin::permute(&mut permuted_values);

        permuted_values
    }

    fn permute(values: &mut Vec<u32>) {
        for i in (1..values.len()).rev() {
            let rand_i = utils::rand_u32() as usize % i;
            values.swap(i, rand_i);
        }
    }
}
