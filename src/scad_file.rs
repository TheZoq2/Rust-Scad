use scad_object::*;
use std::vec::{Vec};
use std::string::{String};


pub struct ScadFile 
{
    objects: Vec<ScadObject>,

    detail: i32,
}

impl ScadFile 
{
    pub fn new() -> ScadFile 
    {
        ScadFile {
            objects: Vec::new(),

            detail: 0,
        }
    }

    pub fn get_code(&self) -> String 
    {
        let mut result = String::from("");

        if self.detail != 0
        {
            result = result + "$fn=" + &self.detail.to_string() + ";\n";
        }

        for object in &self.objects
        {
            result = result + &object.get_code() + "\n";
        }

        result
    }

    pub fn add_object(&mut self, object: ScadObject) 
    {
        self.objects.push(object);
    }

    pub fn set_detail(&mut self, detail: i32) 
    {
        self.detail = detail;
    }
}



#[cfg(test)]
mod file_tests
{
    use scad_object::*;
    use scad_element::*;

    use scad_file::{ScadFile};

    #[macro_use]
    use scad_macros::*;

    #[test]
    fn detail_test()
    {
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        assert_eq!(sfile.get_code(), "$fn=30;\n");

        let obj = ScadObject::new(ScadElement::Union);
        sfile.add_object(obj);

        let mut obj = ScadObject::new(ScadElement::Difference);
        sfile.add_object(obj);

        assert_eq!(sfile.get_code(), "$fn=30;\nunion();\ndifference();\n")
    }
}
