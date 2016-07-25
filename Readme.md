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

Add the following to your cargo.toml.
```
[dependencies]
scad_generator = {git = "https://github.com/TheZoq2/Rust-Scad.git"}
```

And import the crate.
```Rust
#[macro_use]
extern crate scad_generator;
use scad_generator::*;
```

See the documentation for a tutorial.

##Documentation

I will publish the documentation somewhere in a few days when I get home and have access
to a non data capped internet connection. For now you have to build the documentation 
yourself.

The documentation can be built using `cargo doc`. Then you can open the documentation
in `target/doc/scad_generator/index.html`

You can also have a look [https://github.com/TheZoq2/Z-Plane](https://github.com/TheZoq2/Z-Plane) at my RC plane repo for 
an example project that is using the library.

##Utility repo
Since including dependencies in rust projects using cargo is so simple, I have a repository
with some standard objects like nuts, screws and some RC things like servos in a separate
repo which you can also include using cargo. For now it's undocumented but I will get 
around to that eventually.

[https://github.com/TheZoq2/Rust-scad-util](https://github.com/TheZoq2/Rust-scad-util)
