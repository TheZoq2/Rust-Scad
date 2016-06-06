Rust scad generator
===

A library for creating openscad models using rust. I created this project partly because I wanted to learn
more about rust and partly because openscad handles variables in a weird way. Variables in a single scope get
assigned the last value they had in that scope at all points in that scope. This means that the following
code

```
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

```
//"Import" the module along with the macros
#[macro_use]
extern crate scad_generator;

//Avoid having to write scad_generator:: everywhere
use scad_generator::*;

pub fn main()
{
    //Create an scad object
    let translation = scad!(Translate(vec3(2.0, 2.0, 3.0));
            {
                scad!(Cube(vec3(2.0,1.0,4.0)))
            }
        );


    //Print the result
    println!("{}", translation.get_code());
}
```

Which will print the following openscad code
```
translate([2,2,3])
{
	cube([2,1,4]);
}
```

The interesting part is the `scad!` macro

###The scad! macro
The `scad!` macro creates a new scad object with optional children. An scad 
object consists of a number of a parent with 0 or more children. The scad! macro 
takes one parent "object" followed by a `;` optionally followed by `{}`. The expression
returned by the macro is an instance of ScadObject.

The children should be instances of ScadObjects aswell which means that you can create
children directly inside the macro using another call to the `scad!` macro or use 
a variable created earlier.

The parent part of the macro takes an instance from the ScadElement enum which specifies
what type of scad object should be created. 


