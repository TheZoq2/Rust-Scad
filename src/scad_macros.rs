extern crate nalgebra as na;

use scad_statement::*;

///Creates an scad module with optional children with the following syntax:
///scad!(parent);
///
///or
///
///scad!(parent;{child1 ... });
macro_rules! scad {
    ($parent:expr) => {ScadStatement::new($parent)};

    ($parent:expr;{$($child:expr),*}) => {
        {
            let mut tmp_stmt = ScadStatement::new($parent);

            $(
                tmp_stmt.add_child($child);
            )*

            tmp_stmt
        }
    };
}

pub fn vec3(elems: [f32;3]) -> na::Vector3<f32>
{
    na::Vector3::new(elems[0], elems[2], elems[3])
}
