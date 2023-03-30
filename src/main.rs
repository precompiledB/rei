use std::path::Path;

use gltf::mesh::util::ReadIndices;
use image::{GenericImage, GenericImageView, ImageBuffer, Pixel, Rgb, RgbImage};
use crate::camera::Camera;
use crate::maths::{Vec2, Vec3};
use crate::ray::{CameraFovDirection, PinholePerspective, RayGenerator};
use indicatif::ProgressIterator;
use crate::intersections::{Geometry, Intersect, IntersectionResult, Sphere, Triangle};

// https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground/therayclass
// https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix
// source for intersections: https://www.lighthouse3d.com/tutorials/maths/*
// https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing#understandable-raytracing-in-256-lines-of-bare-c
// https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html

mod camera;
mod intersections;
mod maths;
mod ray;

fn read_gltf_data() -> Vec<Triangle> {
    let path = Path::new("models/plane.gltf");

    let (document, buffers, images) = gltf::import(path).unwrap();

    let (mut vertices, mut indices) = (Vec::new(), Vec::new());

    for mesh in document.meshes() {
        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(it) = reader.read_positions() {
                for [x, y, z] in it {
                    vertices.push((x, y, z));

                    let output = format!("vtx {} {} {}\n", x, y, z);
                    dbg!(&output);
                }
            }
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(it) = reader.read_indices() {
                if let ReadIndices::U16(it) = it {
                    let chunks = it.collect::<Vec<_>>();
                    let chunks = chunks.chunks_exact(3);
                    for c in chunks.clone().into_iter() {
                        let output = format!("idx {} {} {}\n", c[0], c[1], c[2]);
                        dbg!(&output);
                        indices.push((c[0], c[1], c[2]));
                    }
                    assert!(chunks.remainder().is_empty());
                }
            } else {
                dbg!("NO");
            }

            let colour = primitive.material().pbr_metallic_roughness().base_color_factor();
            dbg!(colour);

            if let Some(texture) = primitive.material().pbr_metallic_roughness().base_color_texture(){
                let image = &images[texture.texture().index()];

                let data: ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::from_raw(image.width, image.height, image.pixels.clone()).unwrap();

                data.save("test.jpg").unwrap();
            }

            dbg!(primitive.material().ior());
            dbg!(primitive.material().transmission().and_then(|x| Some(x.transmission_factor())));
        }
    }

    let mut tris = Vec::new();
    for i in indices {
        let t = Triangle {
            vertices: [
                vertices[i.0 as usize],
                vertices[i.1 as usize],
                vertices[i.2 as usize],
            ]
            .map(|v| (v.0, v.1 - 0.1, v.2))
            .map(|v| Vec3([v.0 as f64, v.1 as f64, v.2 as f64]))
        };
        tris.push(t);
    }

    tris
}

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
        position: Vec3([0.0,0.,-2.0]),
    };

    let spheres = (1..9).map(|x| {
            let x = x as f64;
            Sphere {
                radius: 0.3,
                position: Vec3([x / 3., 0.,(-2. - x / 4.)])
            }
        }).collect::<Vec<Sphere>>();

    let objects: Vec<_> = spheres.iter().map(|x| x as &dyn Intersect).collect();

    let tri = Triangle {
        vertices: [
            Vec3([0.2, 0.2, -1.8]),
            Vec3([0., 0.2, -2.0]),
            Vec3([0.2, 0., -2.2]),
        ],
    };
    /*
    let geom = Geometry {
        objects: vec![&tri]
    };*/

    let tris = read_gltf_data();
    let geom = Geometry {
        objects: tris.iter().map(|t| t as &dyn Intersect).collect()
    };

    for px_y in (0..height).progress() {
        for px_x in 0..width {
            let (x, y) = (px_x as f64, px_y as f64);
            let color = [x / y, y / x, 0.]
                .map(|i| f64::max(0., f64::min(1., i)) * 255.)
                .map(|i| i as u8);

            let perspective = PinholePerspective {
                camera_fov: 0.785398,
                fov_dir: CameraFovDirection::Horizontal,
                image_size: Vec2([dim.0 as f64, dim.1 as f64]),
            };

            let pixel = Vec2([x, y]);//.map(|i| ( i + 0.5) * 2. - 1.));

            let ray = perspective.gen_ray(pixel);

            let ray_w = cam.ray_cam_to_world(ray.clone());
            //dbg!(x, y, &ray, &ray_w);

            let res = geom.intersect(ray_w);

            let col = match res {
                IntersectionResult::Hit { point, normal, t } => {
                    //dbg!(normal);
                    let m = normal.0.map(|i| (i * 255.));
                    
                    m.map(|i| i as u8)
                },
                IntersectionResult::Miss => {
                    [0xA3, 0xE4, 0xD7]
                },
            };

            img.put_pixel(px_x, px_y, Rgb(col));
        }
    }

    println!("Finished :)\nSaving...");

    img.save("images/second_try.png")?;

    Ok(())
}
