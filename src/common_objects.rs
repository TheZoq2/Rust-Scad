extern crate nalgebra as na;

use scad_macros::*;
use scad_element::ScadElement::*;
use scad_object::*;



/**
  Creates a cube that is centered on the specified axis
*/
pub fn centered_cube(size: na::Vector3<f32>, centering: (bool, bool, bool)) -> ScadObject
{
    let (x,y,z) = centering;

    let mut offset = vec3(0.,0.,0.);
    offset.x = if x {size.x} else {0.};
    offset.y = if y {size.y} else {0.};
    offset.z = if z {size.z} else {0.};

    offset /= -2.; 

    let mut translation = ScadObject::new(Translate(offset));
    translation.add_child(ScadObject::new(Cube(size)));
    translation
}



#[cfg(test)]
mod tests
{
    extern crate nalgebra as na;

    use super::*;

    #[test]
    fn cube_center_x()
    {
        let obj = centered_cube(vec3(1., 1., 1.), (true, false, false));

        assert_eq!(obj.get_code(), "translate([-0.5,0,0])\n{\n\tcube([1,1,1]);\n}");
    }
    #[test]
    fn cube_center_yz()
    {
        let obj = centered_cube(vec3(1., 2., 4.), (false, true, true));

        assert_eq!(obj.get_code(), "translate([0,-1,-2])\n{\n\tcube([1,2,4]);\n}");
    }
}
