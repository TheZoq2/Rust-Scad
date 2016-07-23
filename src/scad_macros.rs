extern crate nalgebra as na;

/**
    Creates an scad object with optional children

    #Examples
    ```
    # #[macro_use]
    # extern crate scad_generator;

    # use scad_generator::*;

    # fn main(){
         //No children
         let cube = scad!(Cube(vec3(1., 1., 1.)));

         //One parent with several children
         scad!(Difference;
         {
             cube,
             scad!(Cube(vec3(2., 1., 1.))),
         });
     # }
    ```
*/

#[macro_export]
macro_rules! scad {
    ($parent:expr) => {ScadObject::new($parent)};

    ($parent:expr;{$($child:expr),*$(),+}) => {
        {
            let mut tmp_stmt = ScadObject::new($parent);

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
    use scad_object::*;
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

    #[test]
    fn many_children_test()
    {
        assert_eq!(scad!(Translate(vec3(0.0,0.0,0.0));{
                scad!(Cube(vec3(1.0,1.0,1.0))),
                scad!(Cube(vec3(1.0,1.0,1.0)))
            }).get_code(), "translate([0,0,0])\n{\n\tcube([1,1,1]);\n\tcube([1,1,1]);\n}"
        );
        //Test trailing edge ,
        assert_eq!(scad!(Translate(vec3(0.0,0.0,0.0));{
                scad!(Cube(vec3(1.0,1.0,1.0))),
                scad!(Cube(vec3(1.0,1.0,1.0))),
            }).get_code(), "translate([0,0,0])\n{\n\tcube([1,1,1]);\n\tcube([1,1,1]);\n}"
        );
    }
}
