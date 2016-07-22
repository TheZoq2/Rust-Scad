Rust scad generator
===

A library for creating openscad models using rust. I created this project partly because I wanted to learn
more about rust and partly because openscad handles variables in a weird way. Variables in a single scope get
assigned the last value they had in that scope at all points in that scope. This means that the following
code

```OpenScad
current_value = 0;
for( i = [0: 3])
{
    current_value = current_value + i;
    echo(current_value);
}
echo(current_value);
```
would print

```
0
1
2
3
0
```
instead of
```
0
1
3
5
5
```
as you would expect in most programming languages.

My library works around this issue by using rust to do all logic and generate simple 
scad code which accomplishes the same thing as more complex scad code would. 

##Usage

First, let's look at a simple example of the crate being used.

```Rust
//"Import" the module along with the macros
#[macro_use]
extern crate scad_generator;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

pub fn main()
{
    //Create an scad file object for storing the scad objects. This
    //allows us to set things like the detail level ($fn) for the models.
    let mut sfile = ScadFile::new();

    //Sets the $fn variable in scad which controls the detail level of things
    //like spheres. Look at the scad wiki for details
    sfile.set_detail(50);

    //Create an scad object
    let cube = scad!(Translate(vec3(2.0, 2.0, 3.0));
            {
                scad!(Cube(vec3(2.0,1.0,4.0)))
            });

    //Create a sphere with a height of 10 and a diameter of 3 mm
    let sphere = scad!(Sphere(10., Diameter(3.)));

    //Add the sphere to the cubes translation.
    cube.add_child(sphere);
    
    //Add the cube object to the file
    sfile.add_object(cube);
    //Save the scad code to a file
    sfile.write_to_file(String::from("out.scad"));

    //You can also print the code for the object manually since it's just a string
    println!(cube.get_code());
}
```

Which will print the following openscad code
```OpenSCAD
translate([2,2,3])
{
	cube([2,1,4]);
    Sphere(h=10, r=3);
}
```

##The scad! macro
The most important part of the crate is the `scad!` macro. The first parameter
of the macro is the element type of the object we want to create which should be 
an instance of the `ScadElement` enum. If you only want to create a single scad
object, you can simply end the macro invocation after the parent like this:

```Rust
scad!(Cube(vec3(10., 10., 10.)))
```

A lot of times, you want to add more elements as children to an scad object. For example
when translating a cube. If you want to add children to the object, add a `;` after the
element type and surround all the children in `{}`. The children should be instances
of the `ScadObject`. The children should be separated by `;`.

```Rust
let child = scad!(Sphere(10., Radius(3.)));

scad!(Difference;{
    //A child can be another call to the scad! macro
    scad!(Cube(vec3(1., 2., 3.)));
    //or a variable that is an scad object
    child;
    //Or even a function that returns an scad objectj
    get_child();
})

fn get_child() -> ScadObject
{
...
}
```

##Object parameters
Almost all `ScadElements` take additional parameters that describes them.

###Vectors
The most common parameter is a vector. This library uses the nalgebra crate for vectors
but writing `na::Vector3::new(x, y, z)` each time you want a vector is tedious which is 
why I have created simple utility functions. `vec3(x, y, z)` and `vec2(x, y)` are simply
functions that call the equivalent nalgebra constructor.

###Circle radii and diameters.
Just like regular OpenSCAD, you can create round objects by either specifying the diameter
or radius of the circle. This is done using the `CircleType` enum which is either 
`Diameter(d)` or `Radius(r)`. 
