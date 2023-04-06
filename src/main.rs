use crate::camera::Camera;
use crate::intersections::{
    Geometry, Intersect, IntersectionResult, Sphere, TriGeometry, Triangle,
};
use crate::maths::{Vec2, Vec3};
use crate::ray::{CameraFovDirection, PinholePerspective, RayGenerator};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;
use rayon::prelude::*;

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground/therayclass
// https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix
// source for intersections: https://www.lighthouse3d.com/tutorials/maths/*
// https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing#understandable-raytracing-in-256-lines-of-bare-c
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html

mod camera;
mod intersections;
mod maths;
mod model;
mod ray;

fn main() -> image::error::ImageResult<()> {
    // Create image and get the dimensions
    let mut img: RgbImage = ImageBuffer::new(2560 / 2, 1440 / 2);
    let dim @ (width, height) = img.dimensions();

    println!("Ray-tracing..");

    let cam = Camera {
        position: Vec3([0., 0., 0.]),
        direction: Vec3([0., 0., -1.]),
        up: Vec3([0., 1., 0.]),
        fov: 0.0,
    };

    let sphere = Sphere {
        radius: 0.3,
        position: Vec3([0.0, 0., -2.0]),
    };

    let spheres = (-2..=6)
        .map(|x| {
            let x = x as f64;
            Triangle {
                vertices: [
                    Vec3([-0.5, 0.5 + x, -15.]),
                    Vec3([-0.5, -0.5 + x, -15.]),
                    Vec3([0.5, 0. + x, -15.]),
                ],
                col: [0xd3, 0x68, 0x7d],
            }
        })
        .collect::<Vec<Triangle>>();

    let objects: Vec<_> = spheres.iter().map(|x| x as &dyn Intersect).collect();

    let tri = Triangle {
        vertices: [
            Vec3([0.2, 0.2, -1.8]),
            Vec3([0., 0.2, -2.0]),
            Vec3([0.2, 0., -2.2]),
        ],
        col: [0xd3, 0x68, 0x7d]
    };
    /*
    let geom = Geometry {
        objects: vec![&tri]
    };*/

    let tris = model::load_from_gltf("models/complex.gltf");
    let geom = TriGeometry { objects: tris };

    const SAMPLES: usize = 4;

    for px_y in (0..height).progress() {
        for px_x in 0..width {
            let (x, y) = (px_x as f64, px_y as f64);
            let perspective = PinholePerspective {
                camera_fov: 0.785398,
                fov_dir: CameraFovDirection::Horizontal,
                image_size: Vec2([dim.0 as f64, dim.1 as f64]),
            };
            let pixel = Vec2([x, y]); //.map(|i| ( i + 0.5) * 2. - 1.));
            let rays = perspective
                .gen_samples(pixel, SAMPLES);

            //let ray_w = cam.ray_cam_to_world(ray.clone());
            //dbg!(x, y, &ray, &ray_w);

            let res = rays
                .iter()
                .map(|r| cam.ray_cam_to_world(r))
                .map(|r| (geom.intersect(&r), r))
                .map(|(res, ray)|

            //let res = geom.intersect(&ray_w);

                match res {
                    IntersectionResult::Hit { point, normal, t, color } => {
                        //dbg!(normal);
                        let n = normal.scalar_mul(ray.dir) / ray.dir.length();

                        let col = color.map(|x| x as f64 * n).map(|x| x as u8);

                        let mut m = normal.0.map(|i| (i * 255.));
                        m[2] *= -1.;
                        m.map(|i| i as u8);

                        col
                    }
                    IntersectionResult::Miss => [0xA3, 0xE4, 0xD7],
                }
            )
            .map(|x| x.map(|y| y as f64))
            .fold([0., 0., 0.], |col_a, col_b| {
                let n = SAMPLES as f64;
                [
                    col_a[0] + col_b[0]/n,
                    col_a[1] + col_b[1]/n,
                    col_a[2] + col_b[2]/n,
                ]}
            ).map(|y| y as u8);

                      

            img.put_pixel(px_x, px_y, Rgb(res));
        }
    }

    println!("Finished :)\nSaving...");

    img.save("images/second_try.png")?;

    Ok(())
}
