# Raytracer in Rust - Documentation
##### Pascal Botzum & J. Leander Zellentin

## **Preproduction & Planning**

### **Rust**
We knew we wanted to use Rust for its speed, ergonomic library system, helpful compiler and foreknowledge in the language. 

### **GLTF File Format**
We also wanted to use the GLTF format, as it has support for a lot of features that we wanted to use in this project immediately within the file, so that everything is in one file. There is a Rust library (crate) that makes working with GLTF files much simpler, as one can access all the information similarly to a markup document.

### **Structuring the Development**
We wanted to follow the structure of the lecture closely, as the chronology was similar to that of building a raytracing engine. For times when there was not enough information or we didn't fully understand how to implement an aspect of the engine, we most frequently turned to the following sources:

 - [ScratchaPixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing/implementing-the-raytracing-algorithm.html) - A complete guide from scratch for writing your own RayTracer. This source also explains every step in detail, as well as the theory behind it.
 - [[[some other source]]]
 - [Khronos GLTF Github](https://github.com/KhronosGroup/glTF) - The Repo containing a lot of useful information about the GLTF standard, including this [handy graphic](https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/figures/gltfOverview-2.0.0b.png).

# Development

## **Camera and Geometry**

### **Creating the Camera**
 - Setting FOV
 - Creating the image plane (focal length, adjustable size, origin)

Due to the camera default view direction being straight downwards, i.e. looking into the negative Y direction, we had problems using the look-at matrix. This was due to a special case where the direction vector generated for the viewing direction was created incorrectly, and nothing was rendered. This issue was overcome by [[[lol]]]. 

### **Creating the First Objects**

As we do not have any explicit points to render, such as triangles and the surface inbetween their verticies, we relied on implicit spheres for the first few rendering tests. These were created by [[[lmao]]], as seen in [[[this code snippet]]]. Using these shapes, we can see how the pixels are representing the 3D space, and apply color based on a ray hit or miss.

At first we rendered the sphere as a "flat" object, assigning only one color based on a binary hit or miss, resulting in just a single color circle. Displaying the multidimensionality of the sphere is difficult without directly simulating light, but we can assign a color to each pixel dependant on the normal at the intersection. This gives us a colourful circle, with each pixel showing a value in one of the 4 directions and also giving a 3D effect on the previously flat circle.

### **Loading Proper Geometry**

Now that we have a working camera and objects can be rendered, we tested if we could render a single triangle. This was done by manually creating three points in the camera space, and creating a surface between them by iterating counter-clockwise. This direction was chosen as it is the default for how GLTF stores the verticies.

Now that individual triangles can be loaded, we can also load complex meshes. Reading from a GLTF is relatively easy compared to other 3D file formats. Here we read all the verticies from a list of indexes that the verticies are stored as, and the actual position out of the index buffer (at the end of the file). Originally we had a problem reading the vertex coordinates out of the buffer because we neglected to read it individually for each item, prohbiting us from reading more than one item. Splitting the reading of the buffer to be on a per-item basis resolved this problem.

Now we can load any 3D mesh and display it in our renderer!

## **Color and Shading effects**

For now, we are still using basic colors applied statically within the program. The normals to calculate how bright the object using the view direction as comparison. This uses the Lambertion equation, creating shadows as if the camera were the point of light.

### **Reading color**

The BRDF colors attatched to the models as created in blender can also be stored within the GLTF, so we created a color for each object and replaced the hardcoded color value with the correct one out of the GLTF. These colors can then be applied to each object when rendered. Now we have properly colored objects! 

### **MSAA**

To calculate the AA, we generate multiple rays per pixel and take the average. This smoothes the image nicely, however multiplies the render times by the amount of multisampling chosen respectively. For testing purposes, we only used 4x MSAA to keep render times low. It is possible to choose between no AA, 2, 4, 8, and 16. We used Halton sequence to generate where within the pixel the ray should be generated from.

### **Specular**


### **Shadows**

### **Multiple Light Points**

## **Rendering Times**

As we are coding our raytracer in Rust with no special options, the code runs on the CPU, on one core. The most direct solution to this is to simply use more CPU cores, as the ray calculations can be parallelized without issue. This is because the different rays don't affect eachother. Multicore processing was implemented using the [Rayon](https://crates.io/crates/rayon) library (crate), which allows us to change the way our iterators are calculated. Thanks to the way Rust is set up, implementing Rayon is very easy to implement. Doing this improves render times, although it is still slow.

Rust automatically optimises the code at compile time, e.g. by embedding constants, simplifying the machine code etc. which lowers rendering times. Rust also has the feature of compiling under a "release mode", which implements much stronger optimization during compilation, increasing compile time but also significantly improving render speeds. Rust allows us to employ multiple compilation profiles, we have one one under `gdb` for debugging, and one under `release` for faster rendering. For reference, rendering normally takes ~10 mins for a model with 1000 triangles at 8x MSAA. With the `release` mode, the same task takes ~1.5 mins.