use scad_element::*;

use std::vec::*;

#[derive(Clone)]
pub struct ScadObject 
{
    element: ScadElement,

    children: Vec<ScadObject>
}

impl ScadObject 
{
    pub fn new(element: ScadElement) -> ScadObject 
    {
        ScadObject {
            element: element,

            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, statement: ScadObject) 
    {
        self.children.push(statement);
    }

    //Returns the code for the current statement
    pub fn get_code(&self) -> String 
    {
        let mut result: String;

        //Get the code for the current element
        result = self.element.clone().get_code();

        //Adding the code for all children, or ; if none exist
        result = result + &(match self.children.len()
        {
            0 => String::from(";"),
            _ => {
                    let mut child_code = String::from("\n{\n");
                    for stmt in &self.children 
                    {
                        //Add the children indented one line
                        child_code = child_code + "\t" + &(stmt.get_code().replace("\n", "\n\t"));
                        child_code = child_code + "\n";
                    }

                    //Add the final bracket and 'return' the result
                    child_code + "}"
                }
        });

        return result;
    }
}






#[cfg(test)]
mod statement_tests
{
    extern crate nalgebra as na;
    use scad_object::*;
    use scad_element::*;

    #[test]
    fn simple_stmt_test()
    {
        let mut test_stmt = ScadObject::new(ScadElement::Translate(na::Vector3::new(0.0, 0.0, 0.0)));

        assert_eq!(test_stmt.get_code(), "translate([0,0,0]);");

        test_stmt.add_child(ScadObject::new(ScadElement::Cube(na::Vector3::new(1.0, 1.0, 1.0))));
        assert_eq!(test_stmt.get_code(), "translate([0,0,0])\n{\n\tcube([1,1,1]);\n}");
    }
}
