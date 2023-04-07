# Raytracer in Rust
##### Pascal E. Botzum & J. Leander Zellentin
Following the guidelines of both the lecture and the Scratchapixel walkthrough, we created a Raytracing Engine with the focus of reading scenes out of GLTF files. We chose to narrow out focus GLTF as it has support for storing all the different necessary aspects of a scene, such as animations, textures and bones. GLTF is also open source, regularly updated and can be used in many different 3D Software.

## Features

The raytracing engine has the following features:

 - Support for **implicit shapes**: spheres and triangles.
 - Support for **geometry**: triangle based meshes from GLTF files
 - Support for **materials**: diffuse, [[[hmm ]]].
 - Support for **specular** surfaces [[[something missing?]]]
 - Support for **variable** anti-aliasing, 1 to 16x MSAA.
 - Support for **parallel** processing using the Rayon library.

## Code Structure

The code is organized into several modules:

 - `main.rs` : the entry point of the application.
 - `intersections.rs`: 
 - `light_transport.rs`: 
 - `maths.rs`: 
 - `model.rs`: 
 - `ray.rs`: 
 - `camera.rs`: a module that defines the Camera struct.


## How to Launch 

[[[pabo please help]]]

## Configuration

Changing parts of the images in the scene is done best in a 3D Software, such as the open source software Blender.

Changes to make in Blender:

 - Changes to geometry (position, shape, scale)
 - Changes to the BRDF, such as color, specularity, IoR etc.

Changes to make in Rust:

 - Image size: `main.rs`
 - The GLTF you wish to load: in `main.rs`
 - Implicit shapes and their location: `main.rs`
 - Where and how to save your resulting PNG `main.rs`
 - Implicit shapes and their position
 - MSAA sample count