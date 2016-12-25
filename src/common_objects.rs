extern crate nalgebra as na;

#[macro_use]
use scad_macros::*;
use scad_element::ScadElement::*;
use scad_object::*;


/**
  An scad cube object that supports various convenience features like
  centering.
*/
pub struct Cube
{
    size: na::Vector3<f32>,

    offset: na::Vector3<f32>
}

impl Cube
{
    pub fn new(size: na::Vector3<f32>) -> Cube
    {
        Cube {
            size: size,

            offset: na::zero()
        }
    }

    /**
        Centers the cube on the specified axis
    */
    pub fn center(mut self, x: bool, y: bool, z: bool) -> Cube
    {
        self.offset.x = if x {self.size.x} else {0.};
        self.offset.y = if y {self.size.y} else {0.};
        self.offset.z = if z {self.size.z} else {0.};

        self.offset = -self.offset / 2.;

        self
    }

    /**
        Converts to scad and destroys the object
    */
    pub fn scad(self) -> ScadObject
    {
        let mut translation = ScadObject::new(Translate(self.offset));
        translation.add_child(ScadObject::new(Cube(self.size)));
        translation
    }
}



#[cfg(test)]
mod tests
{
    extern crate nalgebra as na;

    use super::*;

    #[test]
    fn cube_center_x()
    {
        let obj = Cube::new(na::one())
                    .center(true, false, false)
                    .scad();

        assert_eq!(obj.get_code(), "translate([-0.5,0,0])\n{\n\tcube([1,1,1]);\n}");
    }
    #[test]
    fn cube_center_yz()
    {
        let obj = Cube::new(vec3(1., 2., 4.))
                    .center(false, true, true)
                    .scad();

        assert_eq!(obj.get_code(), "translate([0,-1,-2])\n{\n\tcube([1,2,4]);\n}");
    }
}
