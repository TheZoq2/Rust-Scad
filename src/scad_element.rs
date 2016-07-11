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
        String::from("[") + &self.x.get_code() + "," + &self.y.get_code() + "," + &self.z.get_code() + "]"
    }
}
impl ScadType for na::Vector2<f32>
{
    fn get_code(&self) -> String
    {
        String::from("[") + &self.x.get_code() + "," + &self.y.get_code() + "]"
    }
}

impl ScadType for f32
{
    fn get_code(&self) -> String 
    {
        self.to_string()
    }
}
impl ScadType for i32
{
    fn get_code(&self) -> String 
    {
        self.to_string()
    }
}
impl ScadType for bool
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


/////////////////////////////////////////////////////////////////////////////
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

//Openscad modules or functions
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

    //2D stuff
    Square(na::Vector2<f32>),
}

impl ScadElement
{
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

    #[test]
    fn type_test()
    {
        //No more tests needed for now. I assume the to_string() function works
        //as expected
        assert_eq!(na::Vector3::new(0.0, 0.0, 0.0).get_code(), "[0,0,0]");
        assert_eq!(na::Vector3::new(-5.0, 0.0, 0.0).get_code(), "[-5,0,0]");
        assert_eq!(na::Vector3::new(1.0,2.0,3.0).get_code(), "[1,2,3]");

        assert_eq!(na::Vector2::new(1.0, 3.3).get_code(), "[1,3.3]");
    }

    #[test]
    fn simple_enum_test()
    {
        assert_eq!(ScadElement::Translate(na::Vector3::new(0.5,0.5,0.5)).get_code(), "translate([0.5,0.5,0.5])");

        assert_eq!(ScadElement::Rotate(90.0, na::Vector3::new(1.0, 0.0, 0.0)).get_code(), "rotate(90,[1,0,0])");

        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Radius(7.0)).get_code(), "cylinder(h=5,r=7)");
        assert_eq!(ScadElement::Cylinder(5.0, CircleType::Diameter(7.0)).get_code(), "cylinder(h=5,d=7)");

        assert_eq!(ScadElement::Square(na::Vector2::new(1.,2.)).get_code(), "square([1,2])");
    }

    #[test]
    fn lin_extrude_test()
    {
        assert_eq!(LinExtrudeParams::default().get_code(), "height=1,center=true,convecity=10,twist=0,slices=20");

        assert_eq!(LinExtrudeParams{twist:720., .. Default::default()}.get_code(), "height=1,center=true,convecity=10,twist=720,slices=20");
    }
}
