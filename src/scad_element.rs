use std::string::*;
use scad_type::*;
extern crate nalgebra as na;

use std::vec::Vec;


///Since scad allows creation of circle like objects using either radius or diameter,
///this enum specifies which format to use
#[derive(Clone)]
pub enum CircleType {
    Radius(f32),
    Diameter(f32),
}


/////////////////////////////////////////////////////////////////////////////

///Parameters for the linear extrude function.
///
///These are in a struct because  there are so many of them and
///most of them  can have a default value.
#[derive(Clone)]
pub struct LinExtrudeParams 
{
    pub height: f32,
    pub center: bool,
    pub convexity: i32,
    pub twist: f32,
    pub slices: i32,
}

impl Default for LinExtrudeParams
{
    fn default() -> LinExtrudeParams
    {
        LinExtrudeParams
        {
            height: 1.,
            center: true,
            convexity: 10,
            twist: 0.,
            slices: 20,
        }
    }
}

impl ScadType for LinExtrudeParams
{
    fn get_code(&self) -> String 
    {
        String::from("height=") + &self.height.get_code() + ",center=" + &self.center.get_code() +
            ",convecity=" + &self.convexity.get_code() + ",twist=" + &self.twist.get_code() + 
            ",slices=" + &self.slices.get_code()
    }
}
/////////////////////////////////////////////////////////////////////////////

///Different kinds of scad modules and function. These are parameters
///for `ScadObjects`.
///
///Most of these have  the same name as the openscad counterparts so see
///their documentation for details
#[derive(Clone)]
pub enum ScadElement {
    //Transformation stuff
    Translate(na::Vector3<f32>),
    Scale(na::Vector3<f32>),
    Rotate(f32, na::Vector3<f32>),
    Mirror(na::Vector3<f32>),
    LinearExtrude(LinExtrudeParams),

    Difference,
    Union,
    Hull,
    Intersection,

    //Object stuff
    Cube(na::Vector3<f32>),
    Cylinder(f32, CircleType),
    Sphere(CircleType),
    Cone(f32, CircleType, CircleType),
    
    Polyhedron(Vec<na::Vector3<f32>>, Vec<Vec<i32>>),
    Import(String),

    //2D stuff
    Square(na::Vector2<f32>),
}

impl ScadElement
{
    ///Returns scad code for each of the elements
    pub fn get_code(self) -> String 
    {
        match self
        {
            //Transformation things
            ScadElement::Translate(value) => {
                String::from("translate(") + &value.get_code() + ")"
            },
            ScadElement::Scale(value) => {
                String::from("scale(") + &value.get_code() + ")"
            },
            ScadElement::Rotate(angle, vector) => {
                String::from("rotate(") + &angle.get_code() + "," + &vector.get_code() + ")"
            },
            ScadElement::Mirror(vector) => {
                String::from("mirror(") + &vector.get_code() + ")"
            },
            ScadElement::LinearExtrude(params) => {
                String::from("linear_extrude(") + &params.get_code() + ")"
            }

            //Primitive objects
            ScadElement::Cube(value) => {
                String::from("cube(") + &value.get_code() + ")"
            },
            ScadElement::Cylinder(height, width) => {
                let width_str = match width
                {
                    CircleType::Radius(val) => String::from("r=") + &val.get_code(),
                    CircleType::Diameter(val) => String::from("d=") + &val.get_code(),
                };

                String::from("cylinder(h=") + &height.get_code() + "," + &width_str + ")"
            },
            ScadElement::Sphere(size) => {
                let size_str = match size
                {
                    CircleType::Radius(val) => String::from("r=") + &val.get_code(),
                    CircleType::Diameter(val) => String::from("d=") + &val.get_code(),
                };

                String::from("sphere(") + &size_str + ")"
            },
            ScadElement::Cone(height, size1, size2) => {
                let size1_str = match size1
                {
                    CircleType::Radius(val) => String::from("r1=") + &val.get_code(),
                    CircleType::Diameter(val) => String::from("d1=") + &val.get_code(),
                };
                let size2_str = match size2
                {
                    CircleType::Radius(val) => String::from("r2=") + &val.get_code(),
                    CircleType::Diameter(val) => String::from("d2=") + &val.get_code(),
                };

                String::from("cylinder(h=") + &height.get_code() + "," + &size1_str + "," + &size2_str + ")"
            }

            ScadElement::Polyhedron(points,faces) => {
                String::from("polyhedron(points=") + &points.get_code() +",faces=" + &faces.get_code() + ")"
            },
            ScadElement::Import(path) => {
                String::from("import('") + &path + "')"
            },

            //primitive 2d objects
            ScadElement::Square(value) => {
                String::from("square(") + &value.get_code() + ")"
            },


            //Combination constructs
            ScadElement::Difference => String::from("difference()"),
            ScadElement::Union => String::from("union()"),
            ScadElement::Hull => String::from("hull()"),
            ScadElement::Intersection => String::from("intersection()"),
        }
    }
}
/////////////////////////////////////////////////////////////////////////////



#[cfg(test)]
mod scad_tests
{
    extern crate nalgebra as na;

    use scad_element::*;
    use scad_type::*;

    #[test]
    fn simple_enum_test()
    {
        assert_eq!(ScadElement::Translate(na::Vector3::new(0.5,0.5,0.5)).get_code(), "translate([0.5,0.5,0.5])");

        assert_eq!(ScadElement::Rotate(90.0, na::Vector3::new(1.0, 0.0, 0.0)).get_code(), "rotate(90,[1,0,0])");

        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Radius(7.0)).get_code(), "cylinder(h=5,r=7)");
        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Diameter(7.0)).get_code(), "cylinder(h=5,d=7)");

        assert_eq!(ScadElement::Square(na::Vector2::new(1.,2.)).get_code(), "square([1,2])");

        assert_eq!(ScadElement::Sphere(CircleType::Radius(7.)).get_code(), "sphere(r=7)");
        assert_eq!(ScadElement::Sphere(CircleType::Diameter(7.)).get_code(), "sphere(d=7)");

        assert_eq!(ScadElement::Cone(5., CircleType::Radius(7.), CircleType::Radius(14.)).get_code(), "cylinder(h=5,r1=7,r2=14)");
        assert_eq!(ScadElement::Cone(5., CircleType::Diameter(7.), CircleType::Diameter(14.)).get_code(), "cylinder(h=5,d1=7,d2=14)");

        assert_eq!(ScadElement::Import("hello_world.stl".to_string()).get_code(), "import('hello_world.stl')");
    }

    #[test]
    fn lin_extrude_test()
    {
        assert_eq!(LinExtrudeParams::default().get_code(), "height=1,center=true,convecity=10,twist=0,slices=20");

        assert_eq!(LinExtrudeParams{twist:720., .. Default::default()}.get_code(), "height=1,center=true,convecity=10,twist=720,slices=20");
    }
}
