use scad_object::*;
use std::vec::{Vec};
use std::string::{String};
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;


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

    pub fn write_to_file(&self, path: String) -> bool
    {
        //Writing the result to file
        let path = Path::new(&path);

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(_) => 
                {
                    println!("Couldn't open file for writing");
                    return false;
                },
            Ok(file) => file,
        };

        match file.write(self.get_code().as_bytes()) {
            Err(_) => 
                {
                    println!("Failed to write to output file");
                    return false;
                },
            Ok(_) => {}
        };

        return true;
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

    use std::fs;
    use std::fs::File;
    use std::io::prelude::*;

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

    #[test]
    fn file_test()
    {
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        let write_success = sfile.write_to_file(String::from("test.scad"));
        
        let mut correct_content = false;
        //Read the content of the file
        match File::open("test.scad")
        {
            Ok(mut f) => 
            {
                let mut file_content = String::new();
                match f.read_to_string(&mut file_content)
                {
                    Ok(_) => {
                        if file_content == sfile.get_code()
                        {
                            correct_content = true;
                        }
                        else
                        {
                            println!("Expected {}, Found {}", file_content, sfile.get_code());
                        }
                    },
                    Err(_) => {println!("Failed to read content from output file");}
                };
            },
            Err(_) => {println!("Failed to open file for reading");}
        };

        //Remove the file we created
        match fs::remove_file("test.scad")
        {
            Ok(_) => {},
            Err(_) => {}
        };

        assert!(write_success);
        assert!(correct_content);
    }
}
