use rayon::prelude::*;

use crate::maths::{Vec2, Vec3};

#[derive(Debug, Clone)]
pub struct Ray {
    pub pos: Vec3, // origin
    pub dir: Vec3, // direction
    pub min: f64,  // Start of intersection testing
    pub max: f64,  // End of intersection testing
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.pos + t * self.dir
    }
}

pub enum CameraFovDirection {
    Horizontal = 0,
    Vertical = 1,
    /// Only in fish eye
    Diagonal = 2,
}

const MSAA_1: [[f64; 2]; 1] = [[0.5, 0.5]];
const MSAA_2: [[f64; 2]; 2] = [[0.25, 0.25], [0.75, 0.75]];
const MSAA_4: [[f64; 2]; 4] = [
    [0.375, 0.125],
    [0.875, 0.375],
    [0.625, 0.875],
    [0.125, 0.625],
];
const MSAA_8: [[f64; 2]; 8] = [
    [0.5625, 0.6875],
    [0.4375, 0.3125],
    [0.8125, 0.4375],
    [0.3125, 0.8125],
    [0.1875, 0.1875],
    [0.0625, 0.5625],
    [0.6875, 0.0625],
    [0.9375, 0.9375],
];
const MSAA_16: [[f64; 2]; 16] = [
    [0.5625, 0.4375],
    [0.4375, 0.6875],
    [0.3125, 0.375],
    [0.75, 0.5625],
    [0.1875, 0.625],
    [0.625, 0.1875],
    [0.1875, 0.3125],
    [0.6875, 0.8125],
    [0.375, 0.125],
    [0.5, 0.9375],
    [0.25, 0.875],
    [0.125, 0.25],
    [0.0, 0.5],
    [0.9375, 0.75],
    [0.875, 0.0625],
    [0.0625, 0.0],
];

const MSAA_SAMPLES: [&[[f64; 2]]; 5] = [
    &MSAA_1,
    &MSAA_2,
    &MSAA_4,
    &MSAA_8,
    &MSAA_16,
];

pub trait RayGenerator {
    fn gen_ray(&self, pixel: Vec2) -> Ray;
    fn gen_samples(&self, pixel: Vec2, samples: usize) -> Vec<Ray> {
        let samples = {
            let idx = match samples {
                1 => 0,
                2 => 1,
                4 => 2,
                8 => 3,
                16 => 4,
                _ => panic!("Not a valid sampling size. 2^0=1 .. 2^4=16 is supported.")
            };
            MSAA_SAMPLES[idx]
        };
        
        samples.iter().map(|offset| {
            let new_pixel = pixel + Vec2(*offset);
            self.gen_ray(new_pixel)
        }).collect()
    }
}

pub struct PinholePerspective {
    /// radians
    pub camera_fov: f64,
    /// Camera fov direction
    pub fov_dir: CameraFovDirection,
    /// for conversion in f64
    pub image_size: Vec2,
}

impl RayGenerator for PinholePerspective {
    fn gen_ray(&self, pixel: Vec2) -> Ray {
        let tan_half_angle = (self.camera_fov / 2.).tan();
        let aspect_ratio = self.image_size.x() / self.image_size.y();

        // raster space
        let dir2d = [pixel.x(), pixel.y()];

        // normalized device coordinates
        // todo: add sampling
        let dir2d = [
            (dir2d[0] + 0.5) / self.image_size.x(),
            (dir2d[1] + 0.5) / self.image_size.y(),
        ];

        // screen space
        let dir2d = [2. * dir2d[0] - 1., 1. - (2. * dir2d[1])];

        // accounting fov and arbitrary image aspect ratio
        let dir2d = [
            dir2d[0] * aspect_ratio * tan_half_angle,
            dir2d[1] * tan_half_angle,
        ];

        //println!("[{}, {}]", dir2d[0], dir2d[1]);

        let dir = Vec3::new(dir2d[0], dir2d[1], -1.);

        Ray {
            pos: Vec3::new(0., 0., 0.),
            dir,
            min: 0.,
            max: f64::MAX,
        }
    }
}

pub struct ThinLensPerspective {}

impl RayGenerator for ThinLensPerspective {
    fn gen_ray(&self, pixel: Vec2) -> Ray {
        todo!()
    }
}
