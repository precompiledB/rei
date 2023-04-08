use std::ops::{Mul, Add, AddAssign};

use crate::{maths::Vec3, ray::Ray};

#[derive(Debug, Clone, Copy)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}
impl Color {
    pub fn to_fcolor(&self) -> FColor {
       let arr = [self.r, self.g, self.b];
       FColor { rgb: arr.map(|x| (x as f64) / 256. )
     } 
    }
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

impl From<Color> for [f64; 3] {
    fn from(val: Color) -> Self {
        [val.r, val.g, val.b].map(|x| x as f64)
    }
}

impl Mul<f64> for Color {
    type Output = Color;

    fn mul(self, rhs: f64) -> Self::Output {
        let scale = |x| x as f64 / 256.;
        let rescale = |x| (x * 256.) as u8;

        let col: [u8; 3] = self.into();        
        col.map(scale).map(|x| x * rhs).map(rescale).into()
    }
}

impl Add<Color> for Color {
    type Output = Color;

    fn add(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct FColor {
    pub rgb: [f64; 3],
}

impl FColor {
    pub fn to_color(self) -> Color {
        let arr = self.rgb.map(|x| (x * 256.) as u8);
        Color::from(arr)
    }
}

impl From<[f64; 3]> for FColor {
    fn from(value: [f64; 3]) -> Self {
        Self { rgb: value }
    }
}

impl Mul<f64> for FColor {
    type Output = FColor;

    fn mul(self, rhs: f64) -> Self::Output {
        FColor { rgb: self.rgb.map(|x| x * rhs) }
    }
}

impl Mul<FColor> for FColor {
    type Output = FColor;

    fn mul(self, rhs: FColor) -> Self::Output {

        FColor { rgb: [
            self.rgb[0] * rhs.rgb[0],
            self.rgb[1] * rhs.rgb[1],
            self.rgb[2] * rhs.rgb[2],
        ] }
    }
}

impl Add<FColor> for FColor {
    type Output = FColor;

    fn add(self, rhs: FColor) -> Self::Output {
        FColor { rgb: [
            self.rgb[0] + rhs.rgb[0],
            self.rgb[1] + rhs.rgb[1],
            self.rgb[2] + rhs.rgb[2],
        ] }
    }
}

impl AddAssign for FColor {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

#[derive(Copy, Clone, Debug)]
pub struct PBRMaterial {
    pub color: FColor,
    //specular_exponent: f64,
    pub metallic_factor: f64, 
    pub ior: f64,
    pub transmissive: f64
}

impl From<[u8; 3]> for PBRMaterial {
    fn from(value: [u8; 3]) -> Self {
        PBRMaterial { color: Color::from(value).to_fcolor(), metallic_factor: 0.0, ior: 1.0, transmissive: 0.0 }
    }
}

// ISBN: 978-1-4842-7185-8 page 105
pub fn reflect_light(incoming: &Ray, normal: Vec3, point: Vec3) -> Ray {
    let dir = incoming.dir - 2. * incoming.dir.dotp(normal) * normal;
    Ray {
        pos: point,
        dir,
        min: incoming.min,
        max: incoming.max,
    }
}

const IOR_AIR: f64 = 1.; // roughly that shit

// ISBN: 978-1-4842-7185-8 page 106 onwards
pub fn refract_light(
    incoming: &Ray,
    normal: Vec3,
    point: Vec3,
    ior: f64
) -> Option<Ray> {
    let eta = ior; // relative ior
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
