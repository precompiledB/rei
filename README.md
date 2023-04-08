# Raytracer in Rust
##### Pascal Botzum & J. Leander Zellentin
Following the guidelines of both the lecture, the Scratchapixel walkthrough and other resources, we created a Raytracing Engine with the focus of reading scenes out of GLTF files. We chose to narrow our focus onto GLTF as it has support for storing all the different necessary aspects of a scene, such as animations, textures and bones. GLTF is also open source, regularly updated and can be used in many different 3D Software.

## Features

The raytracing engine has the following features:

 - Support for **shape equations**: spheres and triangles.
 - Support for **geometry**: triangle based meshes from GLTF files
 - Support for **materials**: diffuse, specular, transmission, ior from GLTF files.
 - Support for **variable** anti-aliasing, 1 to 16x MSAA.
 - Support for **parallel** processing using the Rayon library (later gpu).

## Code Structure

The code is organized into several modules:

 - `main.rs` : the entry point of the application. Here you can customize which model to load and how many samples per pixel are used
 - `intersections.rs`:  defines triangle, sphere equations and defines a common interface for more equations
 - `light_transport.rs`: defines color and how color reacts to reflection/refraction
 - `maths.rs`: defines the Vec2/3 structs that are used for point and (geometrical) vector operations
 - `model.rs`: handles gltf loading
 - `ray.rs`: defines how  rays are created using MSAA
 - `camera.rs`: a module that defines the Camera struct and its interactions with camera rays


## How to Launch 

All you need for the code to run is the Rust Programming Language tools.
This code was programmed and tested on version 1.68.2

In order to run the code type in `cargo run`. This compiles and runs the `rei` executable.
Not that in order to use gltf models alot of patience or a beefy cpu is need as at the time of writing BVHs and other optimizations are not added.

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