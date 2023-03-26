//use std::f64::consts::PI;
//use crate::IntersectionResult::{Hit, Miss};

use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};

//use std::ops::{Add, Mul, Sub};
//use cgmath::{Matrix4, One, Point3, Vector3, Vector4};
//use cgmath::num_traits::FloatConst;
use indicatif::ProgressIterator;

/*


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
    Hit {
        point: Vec3,
        normal: Vec3,
        t: f64,
    },
    Miss,
}

struct IntersectionCtx {
    //t_min: f64,
    //t_max: f64, TODO: Add
    hit_record: Vec<Box<dyn Intersectable>>
}

trait Intersectable {
    fn intersect(self, ray: Ray, ctx: IntersectionCtx) -> IntersectionResult;
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
    fn intersect(self, ray: Ray, _ctx: IntersectionCtx) -> IntersectionResult {
        let a = ray.direction.scalar_mul(ray.direction); // D^2
        let b = 2.0 * ray.direction.scalar_mul(ray.origin - self.position); // 2D(O-C)

        let tmp = (ray.origin - self.position).abs();
        let c = (tmp.scalar_mul(tmp)) - (self.radius * self.radius); // |O-C|^2 - R^2

        let delta = b * b - 4.0 * a * c;

        let t = f64::max((-b + delta.sqrt())/2.0*a, (-b - delta.sqrt())/2.0*a);

        const THRESHOLD: f64 = 0.03;
        match delta {
            // TODO: re-add functionality for edge detection
            /*x if x > -THRESHOLD && x < THRESHOLD && t > 0. => Hit(
                [156, 255, 120]
            ), // hit in one point;  green
            x if x > THRESHOLD && t > 0. => {
                let n = (ray.at(t) - self.position).normalize();
                let n = 255. * (0.5 * (Vec3([1.,1.,1.]) + n));
                Hit(n.0.map(|x| x as u8))
            }, // intersect in two point; normal colouring
            x if x < -THRESHOLD => Miss, // hit in no points*/
            x if x > 0. && t > 0.0 => {
                let point = ray.at(t);
                Hit {
                    point,
                    normal: (point - self.position).normalize(),
                    t,
                }
            }
            _ => Miss
        }
    }
}

impl Intersectable for Triangle {
    fn intersect(self, ray: Ray, _ctx: IntersectionCtx) -> IntersectionResult {
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
            Hit {
                t,
                point: ray.at(t),
                normal: plane_normal
            }
        } else {
            Miss
        }
    }
}

// source for intersections: https://www.lighthouse3d.com/tutorials/maths/*

fn main() {
    println!("Hello, world!");



    let mut cam = Camera {
        aperture: Vec3([0., 0., 70.]).normalize(),
        direction: Vec3([0.0, 1.0, -1.]).normalize(),
        up: Vec3([0., 0., 1.]),
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

            let ctx = IntersectionCtx {
                hit_record: vec![],
            };

            let mut col = match object1.intersect(ray, ctx) {
                Hit{ normal, .. } => {
                    (0.5 * (Vec3([1.,1.,1.]) + normal)).0
                        .map(|x: f64| x * 255.)
                        .map(|x| x as u8)
                },
                Miss => {/*print!("*"); */[54, 81, 94]},
            };
            /*
            let ray = Ray { origin, direction };
            col = match object1.intersects(ray, IntersectionCtx) {
                Hit(col) => {/*print!("#");*/ col},
                Miss => {col},
            };
            */
            //print!("{} ", pixel.0[0]);
            cam.frame.put_pixel(x, y, Rgb(col));
        }
    }

    println!("\nFinished with rendering:)");

    // ppm as output is faster than png
    cam.frame.save("images/sphere_col.png").unwrap();
}
*/
*/

mod camera;
mod maths;
mod ray;

fn main() -> image::error::ImageResult<()> {
    // Create image and get the dimensions
    let mut img: RgbImage = ImageBuffer::new(2560/2, 1440/2);
    let dim @ (width, height) = img.dimensions();

    println!("Ray-tracing..");

    for px_y in (0..height).progress() {
        for px_x in 0..width {
            let (x, y) = (px_x as f64, px_y as f64);
            let color = Rgb(
                [x/y, y/x, 0.]
                    .map(|i| f64::max(0., f64::min(1., i)) * 255.)
                    .map(|i| i as u8));


            img.put_pixel(px_x, px_y, color);
        }
    }

    println!("Finished :)\nSaving...");

    img.save("images/second_try.png")?;

    Ok(())
}