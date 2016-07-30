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

/**
  Utility function for creating nalgebra vectors without having
  to write `na::Vector3::new(x,y,z)`
*/
pub fn vec3(x: f32, y: f32, z:f32) -> na::Vector3<f32>
{
    na::Vector3::new(x,y,z)
}

/**
  Utility function for creating nalgebra vectors without having
  to write `na::Vector2::new(x,y)`
*/
pub fn vec2(x: f32, y: f32) -> na::Vector2<f32>
{
    na::Vector2::new(x, y)
}

/**
    Used to create structs with ::new functions that set default values
    without having to write an impl for new. 

    This exists because a lot of times when making scad models, you want to 
    have a lot of parameters with fixed values. `qstruct!` also supports adding
    parameters to the `new` function of the struct for the members that should
    be changeable.

    #Usage
    The qstruct starts with the name of the resulting struct, followed  by a
    list of parameters to the `new()` function inside `()`. After that, the member
    variables are listed inside `{}` and separated by `,`. Each member should 
    have a name, followed by the  
    type followed by the value. The value is any valid rust expression and can contain
    any variables that are in the struct, or parameters to the new function.

    ```
    # #[macro_use]
    # extern crate scad_generator;
    # use scad_generator::*;

    qstruct!(Demo(inner_width: f32)
    {
        //Constant value
        shell_thickness: f32 = 1.,

        //Value based on function parameter
        outer_width: f32 = inner_width + shell_thickness,

        //Value that depends on another member
        outer_height: f32 = outer_width / 2.,
    });

    //Add your own functions to the struct
    impl Demo
    {
        pub fn get_outer(&self) -> ScadObject 
        {
            scad!(Cube(vec3(self.outer_width, self.outer_width, self.outer_height)))
        }
    }
    
    # fn main(){}
    ```

*/
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
        //Create the struct itself
        struct $name
        {
            $(
                pub $mem_name : $mem_type
            ),*
        }

        //Implement the new function
        impl $name
        {
            pub fn new($( $param_name: $param_type ),*) -> $name
            {
                //Create variables for the values first so we can use previous variables when
                //selecting the value of future variables
                $(let $mem_name = $mem_value;)*

                //Create the actual struct itself
                $name{
                    $(
                        $mem_name: $mem_name,
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

    qstruct!{
        Test1(param1: f32, param2: u16)
        {
            var1: f32 = param1 + 5.,
            var2: u16 = 3 + param2,
        }
    }



    #[test]
    fn new_test()
    {
        let instance = Test1::new(1.3, 100);

        assert_eq!(instance.var1, 1.3 + 5.);
        assert_eq!(instance.var2, 100+3);
    }

    //Testing the ability to select values of variables in the struct based on other variables
    //present in it
    qstruct!{Test2()
    {
        var1: f32 = 1.3,
        var2: f32 = var1 * 2.,
    }}

    #[test]
    fn dependent_variables_test()
    {
        let instance = Test2::new();

        assert_eq!(instance.var1, 1.3);
        assert_eq!(instance.var2, 1.3*2.);
    }

    qstruct!{
        Test3()
        {
            var1: f32 = 1.,
            var2: u32 = 3,
        }
    }

    impl Test3
    {
        pub fn get_sum(&self) -> f32
        {
            self.var1 + self.var2 as f32
        }
    }

    #[test]
    fn impl_test()
    {
        assert_eq!(Test3::new().get_sum(), 4.);
    }
}
