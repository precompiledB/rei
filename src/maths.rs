use std::ops::{Add, Mul, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2(pub [f64; 2]);

impl Vec2 {
    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Vec2) -> Self::Output {
        Vec2([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
        ])
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3(pub [f64; 3]);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3([x, y, z])
    }

    pub fn length(&self) -> f64 {
        self.0.map(|x| x * x).iter().sum::<f64>().sqrt()
    }

    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0.map(|x| if length == 0. { 0. } else { x / length }))
    }

    pub fn abs(&self) -> Self {
        Self(self.0.map(|x| x.abs()))
    }

    pub fn scalar_mul(self, rhs: Vec3) -> f64 {
        self.0.iter().zip(rhs.0.iter()).map(|(a, b)| a * b).sum()
    }

    pub fn cross(self, rhs: Vec3) -> Vec3 {
        let v = cgmath::Vector3::from(self.0).cross(cgmath::Vector3::from(rhs.0));
        Vec3([v.x, v.y, v.z])
    }

    pub fn x(&self) -> f64 {
        self.0[0]
    }
    pub fn y(&self) -> f64 {
        self.0[1]
    }
    pub fn z(&self) -> f64 {
        self.0[2]
    }
}

impl Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] - rhs.0[0],
            self.0[1] - rhs.0[1],
            self.0[2] - rhs.0[2],
        ])
    }
}

impl Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3([
            self.0[0] + rhs.0[0],
            self.0[1] + rhs.0[1],
            self.0[2] + rhs.0[2],
        ])
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self(self.0.map(|x| x * rhs))
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0.map(|x| x * self))
    }
}
