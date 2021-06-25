/*!
    This crate is used to generate openscad models using rust.

    ## Usage

    First, let's look at a simple example of the crate being used.

    ```
    //Avoid having to write scad:: everywhere
    use scad::*;

    pub fn main() {
        //Create an scad file object for storing the scad objects. This
        //allows us to set things like the detail level ($fn) for the models.
        let mut scad_file = ScadFile::new();

        //Sets the $fn variable in scad which controls the detail level of things
        //like spheres. Look at the scad wiki for details
        scad_file.set_detail(50);

        //Create an scad object
        let mut cube = scad!(Translate(vec3(2.0, 2.0, 3.0)); {
            scad!(Cube(vec3(2.0,1.0,4.0)))
        });

        //Create a cylinder with a height of 10 and a diameter of 3 mm
        let cylinder = scad!(Cylinder(10., Diameter(3.)));

        //Add the cylinder to the cubes translation.
        cube.add_child(cylinder);

        //Add the cube object to the file
        scad_file.add_object(cube.clone());

        //Save the scad code to a file
        scad_file.write_to_file(String::from("out.scad"));

        # // remove the created file
        # drop(std::fs::remove_file("out.scad"));

        //You can also print the code for the object manually since it's just a string
        println!("{}", scad!(Cube(vec3(5., 3.,  2.))).get_code());
    }
    ```

    Which will print the following openscad code
    ```OpenSCAD
    translate([2,2,3])
    {
        cube([2,1,4]);
        Cylinder(h=10, r=3);
    }
    ```

    ## The `scad!` macro
    The most important part of the crate is the `scad!` macro. The first parameter
    of the macro is the element type of the object we want to create which should be
    an instance of the `ScadElement` enum. If you only want to create a single scad
    object, you can simply end the macro invocation after the parent like this:


    ```
    use scad::*;

    fn main(){
      scad!(Cube(vec3(10., 10., 10.)));
    }
    ```

    A lot of times, you want to add more elements as children to an scad object. For example
    when translating a cube. If you want to add children to the object, add a `;` after the
    element type and surround all the children in `{}`. The children should be instances
    of the `ScadObject`. The children should be separated by `;`.

    ```
    use scad::*;

    fn main() {
        let child = scad!(Cylinder(10., Radius(3.)));

        scad!(Difference;{
            //A child can be another call to the scad! macro
            scad!(Cube(vec3(1., 2., 3.))),
            //or a variable that is an scad object
            child,
            //Or even a function that returns an scad object
            get_child(),
        });

        fn get_child() -> ScadObject {
            //...
            scad!(Union)
        }
    }
    ```

    ## Object parameters
    Almost all `ScadElements` take additional parameters that describe them. They
    are enum parameters so you specify them as you would with enums. Some parameters
    are regular built in types like `f32` but there are some special ones which are
    described below.

    ### Vectors
    The most common parameter is a vector. This library uses the nalgebra crate for vectors
    but writing `na::Vector3::new(x, y, z)` each time you want a vector is tedious which is
    why the library contains the functions `vec3(x, y, z)` and `vec2(x, y)`. They are simply
    functions that call the equivalent nalgebra constructor.

    ### Circle radii and diameters.
    Just like regular OpenSCAD, you can create round objects by either specifying the diameter
    or radius of the circle. This is done using the `CircleType` enum which is either
    `Diameter(d)` or `Radius(r)`.

    ## Creating objects in loops
    In most cases, the `scad!` macro should be good enoough to create objects, but one
    case where it is not,  is when you want to create several objects in a loop and
    add them as children to a specific object. In this case, you have to use the
    `add_child` method of the `ScadObject`  struct manually

    ```
    use scad::*;

    fn main() {
        //Create the parent and make sure its mutable
        let mut parent = scad!(Union);

        for i in 0..3 {
            parent.add_child(scad!(Cube(vec3(0., i as f32, 0.))));
        }
    }
    ```
*/

pub mod common_objects;
mod scad_element;
mod scad_file;
mod scad_object;
mod scad_type;

#[macro_use]
pub mod scad_macros;

pub use scad_element::CircleType::*;
pub use scad_element::ScadElement::*;
pub use scad_element::*;
pub use scad_macros::*;
pub use scad_object::*;

pub use scad_file::*;
pub use scad_type::*;

pub use common_objects::*;
