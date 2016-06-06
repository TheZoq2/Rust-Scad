use std::string::*;

extern crate nalgebra as na;

//Trait for converting from rust types to strings compatible with openscad
pub trait ScadType 
{
    fn get_code(&self) -> String;
}

impl ScadType for na::Vector3<f32>
{
    fn get_code(&self) -> String 
    {
        String::from("[") + &self.x.get_code() + "," + &self.y.get_code() + "," + &self.y.get_code() + "]"
    }
}
impl ScadType for f32
{
    fn get_code(&self) -> String 
    {
        self.to_string()
    }
}

//Since scad allows creation of circle like objects using either radius or diameter,
//this enum specifies which format to use
#[derive(Clone)]
pub enum CircleType {
    Radius(f32),
    Diameter(f32),
}

//Openscad modules or functions
#[derive(Clone)]
pub enum ScadElement {
    //Transformation stuff
    Translate(na::Vector3<f32>),
    Scale(na::Vector3<f32>),
    Rotate(f32, na::Vector3<f32>),

    Difference,
    Union,

    //Object stuff
    Cube(na::Vector3<f32>),
    Cylinder(f32, CircleType)
}

impl ScadElement
{
    pub fn get_code(self) -> String 
    {
        let code: String;

        code = match self
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


            //Combination constructs
            ScadElement::Difference => String::from("difference()"),
            ScadElement::Union => String::from("union()"),
        };

        return code;
    }
}



#[cfg(test)]
mod scad_tests
{
    extern crate nalgebra as na;

    use scad_element::*;

    #[test]
    fn type_test()
    {
        //No more tests needed for now. I assume the to_string() function works
        //as expected
        assert_eq!(na::Vector3::new(0.0, 0.0, 0.0).get_code(), "[0,0,0]");
        assert_eq!(na::Vector3::new(-5.0, 0.0, 0.0).get_code(), "[-5,0,0]");
    }

    #[test]
    fn simple_enum_test()
    {
        assert_eq!(ScadElement::Translate(na::Vector3::new(0.5,0.5,0.5)).get_code(), "translate([0.5,0.5,0.5])");

        assert_eq!(ScadElement::Rotate(90.0, na::Vector3::new(1.0, 0.0, 0.0)).get_code(), "rotate(90,[1,0,0])");

        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Radius(7.0)).get_code(), "cylinder(h=5,r=7)");
        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Diameter(7.0)).get_code(), "cylinder(h=5,d=7)");
    }
}
