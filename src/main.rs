#![allow(dead_code)]
#[macro_use]

extern crate nalgebra as na;

mod scad_element;
mod scad_statement;

#[macro_use]
mod scad_macros;

use scad_element::*;
use scad_statement::*;
use scad_element::ScadElement::*;
use scad_element::CircleType::*;
use scad_macros::*;


pub fn main()
{
    let translation = scad!(Translate(vec3([2.0, 2.0, 3.0]));
            {
                scad!(Cube(na::Vector3::new(2.0,1.0,4.0)))
            }
        );



    println!("{}", translation.get_code());
}

