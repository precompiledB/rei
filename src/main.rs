use std::f64::consts::PI;
use crate::IntersectionResult::{Hit, Miss};
use image::ImageFormat::Png;
use image::{GenericImage, GenericImageView, ImageBuffer, Rgb, RgbImage};
use std::ops::{Add, Mul, Sub};
use cgmath::{Matrix4, One, Point3, Vector3, Vector4};
use cgmath::num_traits::FloatConst;
use indicatif::ProgressIterator;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Vec3([f64; 3]);

impl Vec3 {
    fn length(&self) -> f64 {
        self.0.map(|x| x * x).iter().sum::<f64>().sqrt()
    }

    fn normalize(&self) -> Self {
        let length = self.length();
        Self(self.0.map(|x| {
            if length == 0. {
                0.
            } else {
                x / length
            }
        }))
    }

    fn abs(&self) -> Self {
        Self(self.0.map(|x| x.abs()))
    }

    fn scalar_mul(self, rhs: Vec3) -> f64 {
        self.0.iter().zip(rhs.0.iter()).map(|(a, b)| a * b).sum()
    }

    fn cross(self, rhs: Vec3) -> Vec3 {
        let v = cgmath::Vector3::from(self.0).cross(cgmath::Vector3::from(rhs.0));
        Vec3([v.x, v.y, v.z])
    }

    fn x(&self) -> f64 {self.0[0]}
    fn y(&self) -> f64 {self.0[1]}
    fn z(&self) -> f64 {self.0[2]}
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

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground/therayclass
// ray should be normalized
struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }
}

// point(t) = origin + t * direction

#[derive(Clone)]
struct Camera {
    aperture: Vec3,
    direction: Vec3,
    up: Vec3,
    fov: f64,
    /// place where the image is formed
    frame: RgbImage,
}

impl Camera {
    fn look_at(&self) -> Matrix4<f64> {
        if self.aperture.0 == [0., 0., 0.] {
            return Matrix4::one();
        }

        Matrix4::look_at_rh(
            Point3::from(self.aperture.0),
            Point3::from(self.direction.0),
            Vector3::from(self.up.0)
        )
    }
}

fn transform_by_look_at(look_at: Matrix4<f64>, vec: Vec3) -> Vec3 {
    let v = look_at * cgmath::Vector4::new(vec.x(), vec.y(), vec.z(), 1.);
    let [x,y,z] = [v.x, v.y, v.z];
    Vec3([x,y,z])
}

/* // DOES NOT WORK YET
#[derive(Debug)]
struct Matrix44([[f64;4]; 4]);

impl Camera {

    // https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix
    fn look_at(&self) -> Matrix44 {
        let z_axis = (self.direction - self.aperture).normalize();
        let x_axis = (self.up.cross(z_axis)).normalize();
        let y_axis = z_axis.cross(x_axis);

        dbg!(x_axis);dbg!(y_axis);dbg!(z_axis);

        Matrix44([
            [ x_axis.x(), y_axis.x(), z_axis.x(), 0. ],
            [ x_axis.y(), y_axis.y(), z_axis.y(), 0. ],
            [ x_axis.z(), y_axis.z(), z_axis.z(), 0. ],
            [ -x_axis.scalar_mul(self.aperture), -y_axis.scalar_mul(self.aperture), -z_axis.scalar_mul(self.aperture), 1.],
        ])
    }
}

fn transform_by_look_at(look_at: Matrix44, vec: Vec3) -> Vec3 {
    let vec = {
        let [x,y,z] = vec.0;
        [x, y, z, 1.0]
    };

    let mut out = [0.; 4];

    for i in 0..4 {
        out[i] = vec.iter().zip(look_at.0[i].iter()).map(|(x, y)| dbg!(x * y)).sum()
    }

    let [x,y,z,_] = out;
    Vec3([x, y, z])
}
*/

enum IntersectionResult {
    Hit([u8; 3]),
    Miss,
}

struct IntersectionCtx;

trait Intersectable {
    fn intersects(self, ray: Ray, ctx: IntersectionCtx) -> IntersectionResult;
}

#[derive(Copy, Clone)]
struct Sphere {
    radius: f64,
    position: Vec3,
}

#[derive(Copy, Clone)]
struct Triangle {
    vertices: [Vec3; 3],
}

impl Intersectable for Sphere {
    fn intersects(self, ray: Ray, _ctx: IntersectionCtx) -> IntersectionResult {
        let a = ray.direction.scalar_mul(ray.direction); // D^2
        let b = 2.0 * ray.direction.scalar_mul(ray.origin - self.position); // 2D(O-C)

        let tmp = (ray.origin - self.position).abs();
        let c = (tmp.scalar_mul(tmp)) - (self.radius * self.radius); // |O-C|^2 - R^2

        let delta = b * b - 4.0 * a * c;

        const THRESHOLD: f64 = 0.03;

        let t = f64::max((-b + (b*b-4.0*a*c).sqrt())/2.0*a, (-b - (b*b-4.0*a*c).sqrt())/2.0*a);

        let t = t / 10.;

        match delta {
            x if x > -THRESHOLD && x < THRESHOLD && t > 0. => Hit([156, 255, 120]), // hit in one point;  green
            x if x > THRESHOLD && t > 0. => Hit([255, 0, 120].map(|x| (x as f64 * t / 3.) as u8 * 6)), // intersect in two point; red
            x if x < -THRESHOLD => Miss, // hit in no points
            _ => Miss
        }
    }
}

impl Intersectable for Triangle {
    fn intersects(self, ray: Ray, _ctx: IntersectionCtx) -> IntersectionResult {
        let plane_normal = {
            let a = self.vertices[1] - self.vertices[0];
            let b = self.vertices[2] - self.vertices[0];
            let c = a.cross(b);
            c.normalize()
        };

        let distance = plane_normal.scalar_mul(self.vertices[0]);

        let t = - ((plane_normal.scalar_mul(ray.origin) + distance) /
            plane_normal.scalar_mul(ray.direction));

        let p = ray.origin + (ray.direction * t);


        let edges = [
            self.vertices[1] - self.vertices[0],
            self.vertices[2] - self.vertices[1],
            self.vertices[0] - self.vertices[2]
        ];

        let c = [
            p - self.vertices[0],
            p - self.vertices[1],
            p - self.vertices[2]
        ];

        let q = [
            edges[0].cross(c[0]),
            edges[1].cross(c[1]),
            edges[2].cross(c[2])
        ].map(|x| plane_normal.scalar_mul(x));

        if q.iter().all(|x: &f64| x > &0.0) {
            Hit([23, 60, 60].map(|x| (x as f64 * t) as u8 * 4))
        } else {
            Miss
        }
    }
}

// source for intersections: https://www.lighthouse3d.com/tutorials/maths/*

fn main() {
    println!("Hello, world!");

    let img: RgbImage = ImageBuffer::new(2560/2, 1440/2);

    let dim @ (width, height) = img.dimensions();

    let mut cam = Camera {
        aperture: Vec3([0., 0., 0.]).normalize(),
        direction: Vec3([0.0, 0.0, -1.]),
        up: Vec3([1., 0., 0.]),
        fov: 60.0, // make arbitrary
        frame: img,
    };

    let look_at = cam.look_at();

    println!("{:#?}", cam.look_at());
    dbg!(transform_by_look_at(cam.look_at(), Vec3([0.,0.,0.]))); // testing; TODO: Remove

    // frame: (x - p) - n = 0; p = (0 0 1); n = (0 0 1); from (-1 -1) to (1 1); frame is squared
    let object0 = Sphere { radius: 0.6, position: Vec3([0.0, 0.0, -2.0]) };
    let object1 = Triangle { vertices: [
        Vec3([0.2, 0.2, 0.2]),
        Vec3([0.3, -0.5, 0.3]),
        Vec3([-0.2, -0.6, 0.4])
    ] };

    for x in (0..dim.0).progress() {
        for y in (0..dim.1) {
            // https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing#understandable-raytracing-in-256-lines-of-bare-c
            // https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html
            let origin = transform_by_look_at(look_at, Vec3([0., 0., 0.]));
            let direction = {
                let width = width as f64;
                let height = height as f64;
                let image_aspect_ratio = width / height;

                let deg2rad = |x| -> f64 { x * PI / 180.0 };
                let scale = f64::tan(deg2rad(cam.fov * 0.5));

                // P: direction target point
                let p_x = (2. * ((x as f64) + 0.5) / width - 1.) * image_aspect_ratio * scale;
                let p_y = (1. - 2. * ((y as f64) + 0.5) / height) * scale;

                transform_by_look_at(look_at, Vec3([p_x, p_y, -1.0]).normalize())
            };

            let ray = Ray { origin, direction };
            let mut col = match object0.intersects(ray, IntersectionCtx) {
                Hit(col) => {/*print!("#");*/ col},
                Miss => {/*print!("*"); */[54, 81, 94]},
            };
            let ray = Ray { origin, direction };
            col = match object1.intersects(ray, IntersectionCtx) {
                Hit(col) => {/*print!("#");*/ col},
                Miss => {col},
            };

            //print!("{} ", pixel.0[0]);
            cam.frame.put_pixel(x, y, Rgb(col));
        }
    }

    println!("\nFinished with rendering:)");

    // ppm as output is faster than png
    cam.frame.save("images/tmp.ppm").unwrap();
}
