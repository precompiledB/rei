use crate::maths::Vec3;

#[derive(Clone)]
struct Camera {
    pub aperture: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov: f64,
}
