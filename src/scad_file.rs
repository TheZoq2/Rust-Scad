use crate::scad_object::*;
use crate::scad_type::ScadType;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::string::String;
use std::vec::Vec;

/**
    Object that stores scad objects along with global parameters for
    the objects. Also has methods for writing the  data to files.
*/
pub struct ScadFile {
    objects: Vec<ScadObject>,

    detail: i32,
}

impl ScadFile {
    pub fn new() -> ScadFile {
        ScadFile {
            objects: Vec::new(),

            detail: 0,
        }
    }

    /**
        Returns the code for the global parameters as well as all the
        children in the file
    */
    pub fn get_code(&self) -> String {
        let mut result = String::from("");

        if self.detail != 0 {
            result = result + "$fn=" + &self.detail.to_string() + ";\n";
        }

        for object in &self.objects {
            result = result + &object.get_code() + "\n";
        }

        result
    }

    pub fn add_object(&mut self, object: ScadObject) {
        self.objects.push(object);
    }

    /**
     Sets the $fn variable for the whole file. This varibale defines  the detail
     amount for cylindrical objects
    */
    pub fn set_detail(&mut self, detail: i32) {
        self.detail = detail;
    }

    /**
     Writes the resulting code to a file

     ##Arguments

       path: The path to the file where we want to write relative to the current
       working directory.

     ##Returns
     The function will return false and print a message to the console if
     writing fails.
    */
    pub fn write_to_file(&self, path: String) -> bool {
        //Writing the result to file
        let path = Path::new(&path);

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(_) => {
                println!("Couldn't open file for writing");
                return false;
            }
            Ok(file) => file,
        };

        if file.write(self.get_code().as_bytes()).is_err() {
            println!("Failed to write to output file");
            return false;
        };

        true
    }
}

impl Default for ScadFile {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod file_tests {
    use crate::scad_element::*;

    use super::*;
    use std::fs;
    use std::fs::File;

    #[test]
    fn detail_test() {
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        assert_eq!(sfile.get_code(), "$fn=30;\n");

        let obj = ScadObject::new(ScadElement::Union);
        sfile.add_object(obj);

        let obj = ScadObject::new(ScadElement::Difference);
        sfile.add_object(obj);

        assert_eq!(sfile.get_code(), "$fn=30;\nunion();\ndifference();\n")
    }

    #[test]
    fn file_test() {
        let mut sfile = ScadFile::new();

        sfile.detail = 30;

        let write_success = sfile.write_to_file(String::from("test.scad"));

        let mut correct_content = false;
        //Read the content of the file
        match File::open("test.scad") {
            Ok(mut f) => {
                let mut file_content = String::new();
                match f.read_to_string(&mut file_content) {
                    Ok(_) => {
                        if file_content == sfile.get_code() {
                            correct_content = true;
                        } else {
                            println!("Expected {}, Found {}", file_content, sfile.get_code());
                        }
                    }
                    Err(_) => {
                        println!("Failed to read content from output file");
                    }
                };
            }
            Err(_) => {
                println!("Failed to open file for reading");
            }
        };

        //Remove the file we created
        match fs::remove_file("test.scad") {
            Ok(_) => {}
            Err(_) => {}
        };

        assert!(write_success);
        assert!(correct_content);
    }
}
