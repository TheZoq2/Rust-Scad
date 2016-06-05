#![allow(dead_code)]
#[macro_use]

extern crate nalgebra as na;

mod scad_element;
mod scad_statement;

use scad_element::*;
use scad_statement::*;
use scad_element::ScadElement::*;
use scad_element::CircleType::*;

pub fn main()
{
    let mut translation = ScadStatement::new(Translate(na::Vector3::new(5.0, 5.0, 3.0)));
    let cube = ScadStatement::new(Cube(na::Vector3::new(5.0, 5.0, 5.0)));

    translation.add_child(cube);

    println!("{}", translation.get_code());
}

