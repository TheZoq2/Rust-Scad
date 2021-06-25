use crate::scad_element::{CircleType, ScadElement::*};
use crate::scad_macros::*;
use crate::scad_object::*;
use nalgebra as na;

/**
  Creates a cube that is centered on the specified axis
*/
pub fn centered_cube(size: na::Vector3<f32>, centering: (bool, bool, bool)) -> ScadObject {
    let (x, y, z) = centering;

    let mut offset = vec3(0., 0., 0.);
    offset.x = if x { size.x } else { 0. };
    offset.y = if y { size.y } else { 0. };
    offset.z = if z { size.z } else { 0. };

    offset /= -2.;

    let mut translation = ScadObject::new(Translate(offset));
    translation.add_child(ScadObject::new(Cube(size)));
    translation
}

pub fn centered_square(size: na::Vector2<f32>, centering: (bool, bool)) -> ScadObject {
    let (x, y) = centering;
    let offset = vec2(if x { size.x } else { 0. }, if y { size.y } else { 0. }) / -2.;

    let mut translation = ScadObject::new(Translate2d(offset));
    translation.add_child(ScadObject::new(Square(size)));
    translation
}

/**
  Creates a cylinder that is centered along all axis
*/
pub fn centered_cylinder(height: f32, size: CircleType) -> ScadObject {
    let mut translation = ScadObject::new(Translate(vec3(0., 0., -height / 2.)));
    let cylinder = ScadObject::new(Cylinder(height, size));

    translation.add_child(cylinder);
    translation
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::scad_type::ScadType;

    #[test]
    fn cube_center_x() {
        let obj = centered_cube(vec3(1., 1., 1.), (true, false, false));

        assert_eq!(
            obj.get_code(),
            "translate([-0.5,-0,-0])\n{\n\tcube([1,1,1]);\n}"
        );
    }
    #[test]
    fn cube_center_yz() {
        let obj = centered_cube(vec3(1., 2., 4.), (false, true, true));

        assert_eq!(
            obj.get_code(),
            "translate([-0,-1,-2])\n{\n\tcube([1,2,4]);\n}"
        );
    }

    #[test]
    fn square_center() {
        let obj = centered_square(vec2(2., 4.), (true, true));

        assert_eq!(obj.get_code(), "translate([-1,-2])\n{\n\tsquare([2,4]);\n}");
    }

    #[test]
    fn cylinder_center() {
        let obj = centered_cylinder(10., CircleType::Radius(5.));

        assert_eq!(
            obj.get_code(),
            "translate([0,0,-5])\n{\n\tcylinder(h=10,r=5);\n}"
        );
    }
}
