extern crate nalgebra as na;

///Creates an scad module with optional children with the following syntax:
///scad!(parent);
///
///or
///
///scad!(parent;{child1 ... });
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

    ($parent:expr;$($child:expr),*) => {
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

#[macro_export]
macro_rules! qstruct
{
    ($name:ident
    ($($param_name:ident: $param_type:ty),*$(),+)
    {
        $($mem_name:ident : $mem_type:ty = $mem_value:expr),*$(),+
    }) 
    =>
    {
        struct $name
        {
            $(
                pub $mem_name : $mem_type
            ),*
        }

        impl $name
        {
            pub fn new($( $param_name: $param_type ),*) -> $name
            {
                $name{
                    $(
                        $mem_name: $mem_value,
                    )*
                }
            }
        }
    }
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

        assert_eq!(scad!(Cube(vec3(1.0,3.0,4.0)); scad!(Cylinder(5.0, Radius(3.0)))).get_code(), "cube([1,3,4])\n{\n\tcylinder(h=5,r=3);\n}");
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

    #[test]
    fn qstruct_test() 
    {
    }
}

#[cfg(test)]
mod qstruct_test
{
    extern crate nalgebra as na;

    use scad_macros::*;

    qstruct!{
        Test1(param1: f32, param2: u16)
        {
            var1: f32 = param1 + 5.,
            var2: u16 = 3 + param2,
        }
    }

    //qstruct!{Test2()
    //{
    //    var1: f32 = 1.3,
    //    var2: f32 = var1 * 2.,
    //}}


    #[test]
    fn new_test()
    {
        let instance = Test1::new(1.3, 100);

        assert_eq!(instance.var1, 1.3 + 5.);
        assert_eq!(instance.var2, 100+3);
    }
}
