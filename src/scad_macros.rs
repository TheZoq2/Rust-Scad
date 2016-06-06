extern crate nalgebra as na;

///Creates an scad module with optional children with the following syntax:
///scad!(parent);
///
///or
///
///scad!(parent;{child1 ... });
#[macro_export]
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

pub fn vec3(x: f32, y: f32, z:f32) -> na::Vector3<f32>
{
    na::Vector3::new(x,y,z)
}

#[allow(unused_imports)]
#[allow(unused_attributes)]
#[cfg(test)]
mod macro_test
{
    extern crate nalgebra as na;
    
    use scad_element::*;
    use scad_statement::*;
    use scad_element::ScadElement::*;
    use scad_element::CircleType::*;

    #[macro_use]
    use scad_macros::*;

    #[test]
    fn vec3_test()
    {
        assert_eq!(vec3(0.0, 1.0, 2.0), na::Vector3::new(0.0, 1.0, 2.0));
    }

    #[test]
    fn scad_macro_test()
    {
        assert_eq!(scad!(Cube(vec3(1.0,3.0,4.0))).get_code(), "cube([1,3,4]);");
    }
}
