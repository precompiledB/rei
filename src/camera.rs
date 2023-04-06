use crate::maths::Vec3;
use crate::ray::Ray;
use cgmath::{Matrix3, Matrix4, One, Vector3};

#[derive(Clone)]
pub struct Camera {
    pub position: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov: f64,
}

impl Camera {
    // ISBN: 978-1-4842-7185-8, page 43
    pub fn ray_cam_to_world(&self, ray: &Ray) -> Ray {
        let right = self.direction.cross(self.up);
        let right = right.normalize();
        let up = self.up.normalize();
        let forward = up.cross(right); // should be normalized as well

        //dbg!(right, up, forward);

        let right = Vector3::from(right.0);
        let up = Vector3::from(up.0);
        let forward = Vector3::from(forward.0);

        // lecture : i need from rows
        // todo: re-visit the camera, look how OpenGl does it (lectures: there's a link)
        let cam_rot = Matrix3::from_cols(right, up, -forward);
        let cam_translation = -cam_rot * Vector3::from(self.position.0);

        let dir = cam_rot * Vector3::from(ray.dir.0);
        let pos = cam_translation + Vector3::from(ray.pos.0);

        Ray {
            pos: Vec3::new(pos.x, pos.y, pos.z),
            dir: Vec3::new(dir.x, dir.y, dir.z),
            min: ray.min,
            max: ray.max,
        }
    }
}
