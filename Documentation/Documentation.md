# Raytracer in Rust - Documentation
##### Pascal Botzum & J. Leander Zellentin

## **Preproduction & Planning**

### **Rust**
We knew we wanted to use Rust for its speed, ergonomic library system, helpful compiler and foreknowledge in the language. 

### **GLTF File Format**
We also wanted to use the GLTF format, as it has support for a lot of features that we wanted to use in this project immediately within the file. It enables everything to be encapsulated within one file, making storage and reading of the information very simple. There is a Rust [library](https://lib.rs/crates/gltf) (crate) that makes working with GLTF files much simpler, as one can access all the information similarly to a markup document.

### **Structuring the Development**
We wanted to follow the structure of the lecture closely, as the chronology was similar to that of building a backward ray tracing engine. For times when there was not enough information or we didn't fully understand how to implement an aspect of the engine, we most frequently turned to the following sources:

 - [ScratchaPixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-ray-tracing/implementing-the-raytracing-algorithm.html) - A complete guide from scratch for writing your own RayTracer. This source also explains every step in detail, as well as the theory behind it.
 - [ScratchaPixel](https://www.scratchapixel.com/lessons/3d-basic-rendering/ray-tracing-generating-camera-rays/generating-camera-rays.html) - generating rays
 - [RayTracingInOneWeekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html#rays,asimplecamera,andbackground/therayclass) - creating rays and the camera model
 - [Stackoverflow](https://stackoverflow.com/questions/349050/calculating-a-lookat-matrix) - Creation of the lookup matrix
 - [Lighthouse3D](https://www.lighthouse3d.com/tutorials/maths/*) - source for intersections
 - [TinyRaytracer](https://github.com/ssloy/tinyraytracer/wiki/Part-1:-understandable-raytracing#understandable-raytracing-in-256-lines-of-bare-c) - reference implementation; see how others did it
 - [Khronos GLTF Github](https://github.com/KhronosGroup/glTF) - The Repo containing a lot of useful information about the GLTF standard, including this [handy graphic](https://raw.githubusercontent.com/KhronosGroup/glTF/main/specification/2.0/figures/gltfOverview-2.0.0b.png) that we used a lot.
 - [graphicscodex.com](https://graphicscodex.com/Sample2-RayTriangleIntersection.pdf) - Intersection equations
 - ISBN: 978-1-4842-7185-8 - Ray Tracing Gems II: Several parts from that book

### **Creating 3D Models to Render**

All of the models we used to test, debug and showcase our renderer were modelled, textured and pre-rendered in Blender. They were then exported to GLTF, together with the corresponding PBR materials and transformations. Some of the models showcase basic shapes, such as planes, or cubes and spheres.
There are three directories in this repo:

 - models are the models but exported to GLTF files
 - TriTestBlend contain basic models to be used
 - Raytracing test are supposed to contain final renders and models but alot of them took too much time to render.

## **Blender Animation**

We created a looping animation in blender of a spaceship travelling through some rocks to test our potential animation capabilities, however we never managed to implement animation support.

Here we modeled the spaceship and terrain, duplicated the terrain 3 times and made the transition between start and finish seamless. The animation works through the terrain moving and looping at the end, whilst the spaceship stays in position, moving only side to side. There is a directional light from above, and two point lights at the end of the engines.

There are Blenders' F-Curves to control the smooth movement of the spaceship, and keyframes to control the constant movement of the terrain behind it.

The animation was created with help from the [YouTube video tutorial](https://www.youtube.com/watch?v=PdBUYZHJh30) by Polygon Runway.

![Kestrel Image](../images/lp_kestrel.png)
![Spaceship Animation](../images/spaceship.gif)

# Development

## **Camera and Geometry**

### **Creating the Camera**
 - Setting FOV
 - Creating the image plane (focal length, adjustable size, origin)

Due to the camera default view direction being straight downwards, i.e. looking into the negative Y direction, we had problems using the look-at matrix. This was due to a special case where the direction vector generated for the viewing direction was created incorrectly, and nothing was rendered. This issue was overcome by rewriting the lookat matrix several times. 

### **Creating the First Objects**

As we do not have any explicit points to render, such as triangles and the surface inbetween their vertices, we relied on implicit spheres for the first few rendering tests. These were created by Triangles and Shapes, as seen in `intersetions.rs`. Using these shapes, we can see how the pixels are representing the 3D space, and apply color based on a ray hit/miss.

In the code a trait (i.e. an interface in other pro. langs) is defined for all implicit shapes:
```rust
pub trait Intersect {
    fn intersect(&self, ray: &Ray) -> IntersectionResult;
}
```
So any implicit shape that can be rendered should implement this trait. Any object in the code can test for intersections using the same function. This function takes in its own geometry and the incoming ray and outputs information whether it's a hit or not. The outputted type is seen as follows:

```rust
pub enum IntersectionResult {
    Hit {
        normal: _,
        point: _,
        .. // additional info
    },
    Miss,
}
```

At first we rendered the sphere as a "flat" object, assigning only one color based on a binary hit/miss, resulting in just a flat color circle. Displaying the multidimensionality of the sphere is difficult without directly simulating light, but we can assign a color to each pixel dependant on the normal at the intersection. We compared the direction of the normal to the direction of the ray. Using this to apply color, with each pixel showing a value in one of the 4 colors (we used RGB and black), we get a colourful circle with a 3D effect on the previously flat circle.

![Sphere](../images/sphere_col.jpeg)
![Sphere Normals](../images/sphere.png)

### **Loading Proper Geometry**

Now that we have a working camera and objects can be rendered, we tested if we could render a single triangle. This was done by manually creating three points in the camera space, and creating a surface between them by iterating counter-clockwise. This direction was chosen as it is the default for how GLTF stores the vertices.

Now that individual triangles can be loaded, we can also load complex meshes.
Reading from a GLTF is relatively easy compared to other 3D file formats. Here we read all the vertices from a list of indexes that the vertices are stored as, and the actual position out of the index buffer (at the end of the file). Originally we had a problem reading the vertex coordinates out of the buffer because we neglected to read it individually for each item, prohbiting us from reading more than one item. Splitting the reading of the buffer to be on a per-item basis resolved this problem.

Now we can load any 3D mesh and display it in our renderer!
The code for loading the models and its associated data can be seen in `model.rs`

![Spheres](../images/spheres2.png)
![Complex models](../images/complex.png)

## **Color and Shading effects**

At this time, we were still using basic colors applied using a hardcoded value within the program. The normals to calculate how bright the object using the view direction as comparison. Using the face normals of the geometry multiplied (dot-product) with the ray from the camera, we get a ray direction that we can multiply with the color to shade the object darker, simulating a shadow.

![Complex models with lambertian](../images/complex_lambertian.png)

### **Reading color**

The BSDF colors attatched to the models as created in blender can also be stored within the GLTF, so we created a color for each object and replaced the hardcoded color value with the correct one out of the GLTF. These colors can then be applied to each object when rendered. Now we have properly colored objects, using the BSDF assigned to it in Blender.

![Now with associated colour data](../images/complex_lambertian_msaa4.png)

### **MSAA**

To calculate the anti-ailiasing (AA), we generate multiple rays per pixel and take the average. This smoothes the image nicely, however multiplies the render times by the amount of multisampling chosen respectively. For testing purposes, we only used 4x MSAA to keep render times low. It is possible to choose between no AA, 2, 4, 8, and 16. We used Halton sequence to generate where within the pixel the ray should be generated from.

We modelled our code for the sampling offset arrays after the DirectX implementation as seen in Raytracing Gems II.

Compare the last two pictures to see the difference between using no MSAA and MSAA4.


### **Lighting**

To integrate physically based lighting, we also need to increase our number of bounces so that the ray can hit not only the object, but see if it can reach a light source afterwards. We created a single point light within the virtual space for our lighting. Brightness can be calculated using the direction the light is compared to the surface normal. Surfaces looking 'away' from the light are darker.


### **Specular**

Using the Phong reflection model, we modelled the small intense highlights that a shiny surface may have. We chose the material rubber for testing, which has a specular intensity of 10.

Just like with the lighting, the surface normals are used but this time multiplied exponentially for a small but very strong highlight effect.

![Sphere with specular](../images/sphere_MSAA16_PHONG.png)

### **Shadows**

Instead of using Lambertian or Phong models directly, we do a check to see if the ray can directly travel to the source of light. This is due to the raytracing being done backwards, so from the ray is generated from the camera. If the ray cannot go from an intersection directly to the light source, that area is in shadow and is given the appropriate darkness respectively.

![Now shadows!!](../images/complex_shadow.png)

## **More Complex Features**

Since our codebase was somewhat unstructured, we took some time to refactor some of the functions and improve code clarity. We also temporarily deactivated some features such as the specular reflections before implementing the multiple light sources for us to test the new features without bugs/weird interactions slowing development.

### **Multiple Light Sources**

For rendering with multiple light sources, we rendered the entire scene again for each light. This increases compute time by a lot (an entire extra scene for each additional light), but also delivers an accurate result. The implementation was done following the formulas given in the lecture.

The color mixing for the different lights is a very complex process, we are using a much simpler technique. This is due to time constraints as well as the fact that our current implementation of light is based off  RGB color calculations, so our best option is to add the colors together linearly. The [PBR-Book](https://www.pbr-book.org/) covers this topic extensively.

Our image library for creating and displaying the images, called image, works using three `int` values between 0 - 255 for the RGB values. Working with three `float64` is much more intuitive for the coding of color calculations, so we use those three floats and convert them back at the end of the render process.
Still at some points light can be a little weird. Using too many light sources can oversaturate the image or render it completely white, because lights are only added together linearly.

An extreme result can be seen here:

![Oversaturated image](../images/toomanylights.png)

An example with two different coloured light sources.

![Muliple Lights](../images/sphere_mulligh_specular.png)

### **Reflections**

We implemented reflections by adding more bounces to our rays. Our implementation works recursively, which we did not restrict initially, quickly leading to a stack overflow. Adding a counter to limit the maximum bounces solves this issue.

When a ray intersects with a specular surface, the ray is bounced again using the `shade_with` function in `main.rs`. This happens until the max. bounces is reached, a diffuse surface is intersected, or no object is intersected. When either of these conditions is met, the color is returned.

![Reflectins](../images/complex_with_everything.png)

### **Refractions**

Refraction is done the same way as reflection but adding the colors at the end did not work out completely.

## **Rendering Times**

As we are coding our raytracer in Rust with no special options, the code runs on the CPU, on one core. The most direct solution to this is to simply use more CPU cores, as the ray calculations can be parallelized without issue. This is because the different rays don't affect eachother. Multicore processing was implemented using the [Rayon](https://crates.io/crates/rayon) crate, which allows us to change the way our iterators are calculated. Thanks to the way Rust is set up, implementing Rayon is very easy to implement, as it was only a matter of replacing `iter()` to `par_iter()` to iterate through all the geometry. Doing this improves render times, although it is still slow. Additionally, using rayon we had to limit our way of rendering. While we still iterated sequentially we could use any object that implements the interface `Intersect`. But because of how dynamic dispatch works in this language, all geometry is restricted to only use triangles or to be a single sphere/triangle.

Rust automatically optimises the code at compile time, e.g. by embedding constants, simplifying the machine code etc. which lowers rendering times. Rust also has the feature of compiling under a "release mode", which implements much stronger optimization during compilation, increasing compile time but also significantly improving render speeds. 
We use the "debug mode" for stepping through the code with an debugger. `gdb` was used to set and watch breakpoints and analyze the code further that what simple `println` statements can do.
Executing it under "debug mode" still brought a heavy penalty.
For reference, rendering normally takes ~10 mins for a model with 1000 triangles at 8x MSAA. With the `release` mode, the same task takes ~1.5 mins.

# Conclusion

Overall we are very happy with our raytracing engine, although we wish we could have implemented some more features.

# Planned Features

These are some of the features we considered and structured our code around, but didn't have the time to implement.

### **Culling, Clipping, Occlusion, and Bounding Boxes**

Any kind of performance enhancement is a huge benefit with raytracers, and we didn't have time to implement any of these methods that would have saved some time with the rendering. Occlusion would not make a large difference, as we still need the backside of an object for more accurate renders, however the other methods would have saved on resources and therefore render time.

There exists a Rust library called [BVH](https://lib.rs/crates/bvh) which would allow us to easily generate bounding boxes for our objects very quickly. The library supports heirarchies and axis-alignment to make the boxes as efficient as possible.

### **Smooth Shading and Surface Materials**

 - **Smooth shading**: We didn't manage to implement the color interpolation for the Phong shading, so all of our objects are rendered in a blocky fashion.
 - **Metallics and Dielectrics**: would also be a feature we would really like to implement, as it would not be very complex but make different renders much more interesting.
 - **Texture Support**: Adding support for textures is significantly more complicated due to our implementation as getting the texture mapping correct with the vertices is difficult.

### **Animations**

Stored within the GLTF are also the transformations necessary to change objects in an animation, it would be possible to code an animation mode where multiple renders are done sucessively, using the linear or Bezier curve interpolation for the geometry transformations.

The multiple images could then be conjoined into a GIF or some other format, for example with the [GIF library](https://lib.rs/crates/gif).

### **GPU Acceleration**

Working with the Graphics Processing Unit (GPU) would be a huge performance boost to the image rendering. However, the recursive method of tracing the rays is not possible on the GPU hardware. Theoretically, the code could run on a GPU. As it can be observed in the git log. At the beginnig we tried implementing a raytracer through the `wgpu-rs` crate using the WebGPU interface.

As we strated wrongly we implemented the ray tracer using parallel code that runs on the CPU.
But we learned how to interact with the WebGPU interface and how to use WGSL as the shading language.

For leveraging hard work on the gpu, compute shaders can be used. The recursion problem with shaders can be overcome by giving the gpu data that doesnt have to be run recursively. For us a performance impact is significant when using alot of triangles.

Initial thought was to use a shader per ray and do the complete traceing and shading part. But the only real complexity lies in iterating through the triangles, to calculate if an intersection has occured.
So the idea is to have the recursive parts of the code be on the cpu and onl let the gpu calculate intersections. That would theoretically minimize performance impact.

Implementing this is done with the `gpgpu` caret using WGSL compute shaders on WebGPU.
