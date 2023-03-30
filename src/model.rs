use std::path::Path;

use cgmath::{Matrix4, Vector4};
use gltf::mesh::util::ReadIndices;

use crate::{intersections::Triangle, maths::Vec3};

pub fn read_gltf_data() -> Vec<Triangle> {
    let path = Path::new("models/cubes.gltf");

    let (document, buffers, images) = gltf::import(path).unwrap();

    let (mut vertices, mut indices) = (Vec::new(), Vec::new());

    for mesh in document.meshes() {
        let transform = document.nodes().skip(mesh.index()).next().unwrap().transform().matrix();

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(it) = reader.read_positions() {
                for [x, y, z] in it {
                    let v = Vector4::from([x, y, z, 1.]);

                    let nv = Matrix4::from(transform) * v;

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

            /*
            if let Some(texture) = primitive.material().pbr_metallic_roughness().base_color_texture(){
                let image = &images[texture.texture().index()];

                let data: ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::from_raw(image.width, image.height, image.pixels.clone()).unwrap();

                data.save("test.jpg").unwrap();
            }*/

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
            .map(|v| (v.0, v.1, v.2))
            .map(|v| Vec3([v.0 as f64, v.1 as f64, v.2 as f64]))
        };
        tris.push(t);
    }

    dbg!(tris)
}