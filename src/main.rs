use std::f64::consts::FRAC_PI_4;

use crate::camera::Camera;
use crate::intersections::{
    Geometry, Intersect, IntersectionResult, Sphere, TriGeometry, Triangle,
};
use crate::maths::{Vec2, Vec3};
use crate::ray::{CameraFovDirection, PinholePerspective, RayGenerator};
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use indicatif::ProgressIterator;
use light_transport::{reflect_light, Color, PointLight, FColor, refract_light, PBRMaterial};
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

const SAMPLES: usize = 2;
const LIGHT_PATHS: usize = 8;

fn main() -> image::error::ImageResult<()> {
    // Create image and get the dimensions
    let mut img: RgbImage = ImageBuffer::new(2560/2, 1440/2);
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
                pbr_mat: [0xd3, 0x68, 0x7d].into(),
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
        pbr_mat: [0xd3, 0x68, 0x7d].into(),
    };

    let tris = model::load_from_gltf("models/complex2.gltf");
    let geom = TriGeometry { objects: tris };

    let lights = vec![PointLight {
        point: Vec3([-3., 0., -2.]),
        color: [255, 255, 255].into(),
        intensity: 1.,
    }];

    let lights = vec![
        /*PointLight { // pink
            point: Vec3([-2., 0., 2.]),
            color: [214, 2, 112].into(),
            intensity: 1.,
        },
        PointLight { // blue
            point: Vec3([1., 0., 1.]),
            color: [0, 56, 168].into(),
            intensity: 1.,
        }, */
        PointLight { //white
            point: Vec3([3., 0., 0.5]),
            color: [255, 255, 255].into(),
            intensity: 1.,
        },
    ];

    for px_y in (0..height).progress() {
        for px_x in 0..width {
            let (x, y) = (px_x as f64, px_y as f64);
            let rays = generate_rays(Vec2([x, y]), &perspective, &cam, SAMPLES);

            let col = trace_with(rays, &geom, &lights);

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

fn trace_with<U>(rays: Vec<Ray>, geom: &U, lights: &Vec<PointLight>) -> Color
where
    U: Intersect,
{
    let rgb = rays
        .iter()
        .map(|r| (geom.intersect(&r), r))
        .map(|(res, ray)| match res {
            IntersectionResult::Hit {
                idx,
                point,
                normal,
                t,
                color,
            } => {
                shade_with(color.into(), point, normal, ray, lights, geom, 0)
                    .to_color().into()
            }
            IntersectionResult::Miss => [0x01, 0x01, 0x01],
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

fn shade_with<U>(mat_col: PBRMaterial, hit_point: Vec3, hit_normal: Vec3, hit_from: &Ray, lights: &Vec<PointLight>, geom: &U, light_path_num: usize) -> FColor
where U: Intersect
{
    if light_path_num == LIGHT_PATHS {
        return FColor{ rgb: [1.,1.,1.] };
    }

    let specular_exponent = 50.; // rubber
    let mut hit_color = FColor{ rgb: [0.,0.,0.] };

    for light in lights {
        let light_dir = (light.point - hit_point).normalize();
        let light_color = light.color.to_fcolor();
        
        let shadow_orig = {
            let vf = if light_dir.dotp(hit_normal) < 0. {
                -1.
            } else {
                1.
            };
            hit_point + vf * hit_normal * 1e-4
        };
        let light_ray = Ray {
            dir: light_dir,
            pos: shadow_orig,
            min: 0.,
            max: f64::INFINITY,
        };
        let light_distance = (light.point - hit_point).length();

        // reflection
        let reflected_ray = reflect_light(&hit_from, hit_normal, hit_point);
        let reflected_ray = {
            let vf = if reflected_ray.dir.dotp(hit_normal) < 0. {
                -1.
            } else {
                1.
            };
            let pos = hit_point + vf * hit_normal * 1e-4;
            Ray {
                pos,
                ..reflected_ray
            }
        };
        let reflected_col = {
            if let IntersectionResult::Hit {
                color,
                normal,
                point,
                ..
            } = geom.intersect(&reflected_ray) {
                Some(shade_with(color.into(), point, normal, &reflected_ray, lights, geom, light_path_num + 1))
            } else {
                None
            }
        };

        if let Some(col) = reflected_col {
            hit_color += col;
        }

        // refraction
        let refracted_ray = refract_light(hit_from, hit_normal, hit_point, mat_col.ior);
        if refracted_ray.is_some() {
            let refracted_ray = refracted_ray.unwrap();
            let refracted_ray = {
                let vf = if refracted_ray.dir.dotp(hit_normal) < 0. {
                    -1.
                } else {
                    1.
                };
                let pos = hit_point - vf * hit_normal * 1e-4;
                Ray {
                    pos,
                    ..refracted_ray
                }
            };
            let refracted_col = {
                if let IntersectionResult::Hit {
                    color,
                    normal,
                    point,
                    ..
                } = geom.intersect(&refracted_ray) {
                    Some(shade_with(color.into(), point, normal, &refracted_ray, lights, geom, light_path_num + 1))
                } else {
                    None
                }
            };
    
            if let Some(col) = refracted_col {
                hit_color += col * 0.1;
            }
        }

        // shadows
        let light_intersect_res = geom.intersect(&light_ray);
        if let IntersectionResult::Hit { point, .. } = light_intersect_res
        {
            if (point - shadow_orig).length() < light_distance {
                continue;
            }
        }

        // diffuse
        let albedo = FColor::from([0.18,0.18,0.18]);
        let color = /*albedo **/ std::f64::consts::FRAC_1_PI * light.intensity * f64::max(0., hit_normal.dotp(-light_dir));
        let color = light_color * color * (mat_col.color * 2.);
        hit_color += color;

        // specular
        let reflected_ray = reflect_light(
            &light_ray,
            hit_normal,
            hit_point,
        );
        hit_color += FColor{ rgb: [1., 1., 1.] } * f64::max(0., reflected_ray.dir.dotp(hit_from.dir))
            .powf(specular_exponent) * mat_col.metallic_factor
            * light.intensity * 0.2;
    }

    hit_color
    //c_shaded.into()
}