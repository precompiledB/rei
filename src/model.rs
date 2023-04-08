use std::path::Path;

use cgmath::{Matrix4, SquareMatrix, Vector4};
use gltf::mesh::util::ReadIndices;

use crate::{intersections::Triangle, maths::Vec3, light_transport::{PBRMaterial, FColor}};

pub fn load_from_gltf<T: AsRef<str>>(path: T) -> Vec<Triangle> {
    let path = Path::new(path.as_ref());

    let (document, buffers, _images) = gltf::import(path).unwrap();

    let mut tris = Vec::new();

    dbg!(document.meshes().len());

    for (idx, mesh) in document.meshes().enumerate() {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        dbg!(idx);

        let current_node = document.nodes().nth(mesh.index());
        let transform = current_node.clone().unwrap().transform();

        if Matrix4::from(current_node.unwrap().transform().matrix()).determinant() > 0. {
            println!("CCW");
        } else {
            println!("CW");
        }
        dbg!(mesh.name());
        dbg!(mesh.primitives().len());

        for primitive in mesh.primitives() {
            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(it) = reader.read_positions() {
                for [x, y, z] in it {
                    let v = Vector4::from([x, y, z, 1.]);

                    let transf = Matrix4::from(transform.clone().matrix());

                    let nv = transf * v;
                    // NOTE: z and y swapped
                    let nv = Vec3([nv.x, nv.z, nv.y].map(|a| a as f64));

                    vertices.push((nv.x(), nv.y(), nv.z()));

                    let output = format!("vtx {} {} {}\n", x, y, z);
                    dbg!(&output);
                }
            }

            // every indices element represents one colour: associate color attributes here

        let color = {
            let color = primitive
            .material()
            .pbr_metallic_roughness()
            .base_color_factor();
        let metallic_factor = primitive
            .material()
            .pbr_metallic_roughness()
            .metallic_factor() as f64;
        let ior = primitive.material().ior().unwrap_or(1.0) as f64;
        let transmissive = primitive
            .material()
            .transmission()
            .and_then(|x| Some(x.transmission_factor()))
            .unwrap_or(0.0) as f64;

            PBRMaterial {
            color: FColor { rgb: [color[0] as f64, color[1] as f64, color[2] as f64] }, // ignore alpha
            metallic_factor,
            ior,
            transmissive,
        }};

            let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));
            if let Some(it) = reader.read_indices() {
                if let ReadIndices::U16(it) = it {
                    let chunks = it.collect::<Vec<_>>();
                    let chunks = chunks.chunks_exact(3);
                    for c in chunks.clone() {
                        let output = format!("idx {} {} {}\n", c[0], c[1], c[2]);
                        dbg!(&output);
                        indices.push((c[0], c[1], c[2], color));
                    }
                    assert!(chunks.remainder().is_empty());
                }
            } else {
                dbg!("NO");
            }

            //dbg!(colour);
            
            /*
            if let Some(texture) = primitive.material().pbr_metallic_roughness().base_color_texture(){
                let image = &images[texture.texture().index()];
                
                let data: ImageBuffer<Rgb<u8>, _> = image::ImageBuffer::from_raw(image.width, image.height, image.pixels.clone()).unwrap();
                
                data.save("test.jpg").unwrap();
            }*/
        }

        for i in indices {
            let t = Triangle {
                vertices: [
                    vertices[i.0 as usize],
                    vertices[i.1 as usize],
                    vertices[i.2 as usize],
                ]
                .map(|v| (v.0, v.1, v.2))
                .map(|v| Vec3([v.0 as f64, v.1 as f64, v.2 as f64])),
                pbr_mat: i.3,
            };
            tris.push(t);
        }
    }

    //dbg!(tris)
    tris
}
