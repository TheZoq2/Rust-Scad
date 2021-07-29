use scad::*;

#[test]
pub fn simple_usage() {
    let translation = scad!(Translate(vec3(2.0, 2.0, 3.0));
        {
            scad!(Cube(vec3(2.0,1.0,4.0)))
        }
    );

    //println!("{}", translation.get_code());
    assert_eq!(
        "translate([2,2,3])\n{\n\tcube([2,1,4]);\n}",
        translation.get_code()
    );
}

#[test]
pub fn variable_children() {
    let mut parent = scad!(Translate(vec3(2.0, 2.0, 3.0));
        {
            scad!(Cube(vec3(2.0,1.0,4.0)))
        }
    );

    let child = scad!(Cylinder(3.0, Radius(5.0)));

    parent.add_child(child);

    assert_eq!(
        "translate([2,2,3])\n{\n\tcube([2,1,4]);\n\tcylinder(h=3,r=5);\n}",
        parent.get_code()
    );
}

#[test]
fn scad_file_test() {
    let mut file = ScadFile::new();

    let mut parent = scad!(Translate(vec3(2.0, 2.0, 3.0));
        {
            scad!(Cube(vec3(2.0,1.0,4.0)))
        }
    );

    let child = scad!(Cylinder(3.0, Radius(5.0)));

    parent.add_child(child);

    file.add_object(parent);
    file.set_detail(40);

    assert_eq!(
        "$fn=40;\ntranslate([2,2,3])\n{\n\tcube([2,1,4]);\n\tcylinder(h=3,r=5);\n}\n",
        file.get_code()
    );
}
