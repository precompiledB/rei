use crate::{maths::Vec3, ray::Ray};

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<[u8; 4]> for Color {
    fn from(value: [u8; 4]) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
            a: value[3],
        }
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Color {
            r: value[0],
            g: value[1],
            b: value[2],
            a: 0xFF,
        }
    }
}

impl From<Color> for [u8; 3] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b]
    }
}

pub struct PBRMaterial {
    color: Color,
    specular_exponent: f64,
}

// page 105
pub fn reflect_light(incoming: Ray, normal: Vec3, point: Vec3) -> Ray {
    let dir = incoming.dir - 2. * incoming.dir.dotp(normal) * normal;
    Ray {
        pos: point,
        dir,
        min: incoming.min,
        max: incoming.max,
    }
}

const IOR_AIR: f64 = 1.; // roughly that shit

// page 106 onwards
pub fn refract_light(
    incoming: Ray,
    normal: Vec3,
    point: Vec3,
    eta_in: f64,
    eta_out: f64,
) -> Option<Ray> {
    let eta = eta_in / eta_out; // relative ior
    let c1 = -incoming.dir.dotp(normal); // cos( index of reflection)
    let w = eta * c1;
    let c2m = (w - eta) * (w + eta); // cos^2 (outgoing iof) - 1

    if c2m < -1.0 {
        None // total internal reflecion
    } else {
        let dir = eta * incoming.dir + (w - (1. + c2m).sqrt()) * normal;
        Some(Ray {
            pos: point,
            dir,
            min: incoming.min,
            max: incoming.max,
        })
    }
}

/* pub fn light_source_from(
    incoming: &Ray,
    point: Vec3,
    normal: Vec3,
    light: &PointLight,
) -> Ray {
    let light_dir = light.point - point;
    Ray { pos: point, dir: light_dir, min: 0., max: f64::INFINITY }
} */

// TODO: emmisive

pub struct PointLight {
    pub point: Vec3,
    pub color: Color,
    pub intensity: f64,
}
