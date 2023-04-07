use std::f64::consts::FRAC_PI_4;

use crate::camera::Camera;
use crate::intersections::{
    Geometry, Intersect, IntersectionResult, Sphere, TriGeometry, Triangle,
};
use crate::maths::{Vec2, Vec3};
use crate::ray::{CameraFovDirection, PinholePerspective, RayGenerator};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;
use light_transport::{Color, PointLight, reflect_light};
use ray::Ray;
use rayon::prelude::*;

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground/therayclass
// https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix
// source for intersections: https://www.lighthouse3d.com/tutorials/maths/*
// https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing#understandable-raytracing-in-256-lines-of-bare-c
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html

mod camera;
mod intersections;
mod light_transport;
mod maths;
mod model;
mod ray;

const SAMPLES: usize = 4;

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

    let perspective = PinholePerspective {
        camera_fov: FRAC_PI_4,
        fov_dir: CameraFovDirection::Horizontal,
        image_size: Vec2([dim.0 as f64, dim.1 as f64]),
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
        col: [0xd3, 0x68, 0x7d],
    };

    let tris = model::load_from_gltf("models/sphere.gltf");
    let geom = TriGeometry { objects: tris };

    let lights = vec![PointLight {
        point: Vec3([-3., 0., -2.]),
        color: [255, 255, 255].into(),
        intensity: 1.,
    }];

    let lights = vec![PointLight {
        point: Vec3([1., 0., 1.]),
        color: [255, 255, 255].into(),
        intensity: 1.,
    }];

    for px_y in (0..height).progress() {
        for px_x in 0..width {
            let (x, y) = (px_x as f64, px_y as f64);
            let rays = generate_rays(Vec2([x, y]), &perspective, &cam, SAMPLES);

            let col = trace_and_shade_with(rays, &geom, &lights);

            img.put_pixel(px_x, px_y, Rgb(col.into()));
        }
    }

    println!("Finished :)\nSaving...");

    img.save("images/second_try.png")?;

    Ok(())
}

fn generate_rays<T>(pixel: Vec2, perspective: &T, cam: &Camera, samples: usize) -> Vec<Ray>
where
    T: RayGenerator,
{
    let rays = perspective.gen_samples(pixel, samples);

    //let ray_w = cam.ray_cam_to_world(ray.clone());
    //dbg!(x, y, &ray, &ray_w);

    rays.iter().map(|r| cam.ray_cam_to_world(r)).collect()
}

fn trace_and_shade_with<U>(rays: Vec<Ray>, geom: &U, lights: &Vec<PointLight>) -> Color
where
    U: Intersect,
{
    let rgb = rays
        .iter()
        .map(|r| (geom.intersect(&r), r))
        .map(|(res, ray)| match res {
            IntersectionResult::Hit {
                point,
                normal,
                t,
                color,
            } => {
                let mut col = color.map(|x| x as f64);

                let mut diffuse_intensity = 0.;
                let mut specular_intensity = 0.;
                let specular_exponent = 10.; // rubber
                // TODO: add albedo
                for l in lights {
                    let light_dir = (l.point - point).normalize();
                    //if let IntersectionResult::Miss = geom.intersect(&light_ray) {
                        // brightness
                    
                    //dbg!(light_dir);
                    //dbg!(normal, normal.length());

                        diffuse_intensity += (l.intensity * f64::max(0., light_dir.dotp(-normal)));
                        
                        // TODO: rwrok reflect func.
                        let reflected_ray = reflect_light(Ray {dir: light_dir, pos: point, min: 0., max: f64::INFINITY }, normal, point);
                        specular_intensity += f64::max(0., reflected_ray.dir.dotp(ray.dir)).powf(specular_exponent) * l.intensity;
                        //};
                }
                    
                let col = col.map(|x| x * diffuse_intensity);
                let specs = [255.; 3].map(|x| x * specular_intensity);
                ([col[0] + specs[0], col[1] + specs[1], col[2] + specs[2],]).map(|x| x as u8)


                //dbg!(normal);
                /*
                let n = normal.dotp(ray.dir) / ray.dir.length();
                let n = f64::max(0.0, n);
                let col = color.map(|x| x as f64 * n).map(|x| x as u8);
                */
                
            }
            IntersectionResult::Miss => [0xA3, 0xE4, 0xD7],
        })
        .map(|x| x.map(|y| y as f64))
        .fold([0., 0., 0.], |col_a, col_b| {
            let n = rays.len() as f64;
            [
                col_a[0] + col_b[0] / n,
                col_a[1] + col_b[1] / n,
                col_a[2] + col_b[2] / n,
            ]
        })
        .map(|y| y as u8);
    Color::from(rgb)
}
