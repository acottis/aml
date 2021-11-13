//! This is a very basic library to read my AML (Adam Markup Language) files. The syntax is basic, but very strict, for simplicity.
//! This is a way to use "secret" variable files in Rust.
//! 
//! The library reads a .aml file and returns a dictionary of key/value pairs. This library can also create an .aml from a HashMap
//! 
//! Here is a file example.aml for reference
//! ```aml
//! key1===data
//! key2===254235!fdjsfhksjd£$%
//! key3===Data with spaces
//! ```
//! 
//! Syntax is:
//! 
//! `key===value`
//! 
//! No additional whitespace is needed. Any whitespace used will be considered as part of the key/value.
//! 
//! # Usage Examples
//! 
//! - Load File
//! ```rust
//!     use aml::load;    
//! 
//!     let filename = "example.aml";
//!     let data = load(filename.to_string());
//! 
//!     println!("{:?}", data);
//! ```
//! The above returns `{"key2": "254235!fdjsfhksjd£$%", "key3": "Data with spaces", "key1": "data"}`
//! 
//! - Save file
//! ```rust
//!     use aml::save;
//! 
//!     let filename = "test_save.aml";
//!     let data = std::collections::HashMap::from([
//!        ("key1".to_string(), "data".to_string()),
//!        ("key2".to_string(), "254235!fdjsfhksjd£$%".to_string()),
//!        ("key3".to_string(), "Data with spaces".to_string()),
//!     ]);
//!
//!     save(filename.to_string(), data).ok();
//! ```
//! Creates a file called "test_save.aml" with contents
//! ```aml
//! key1===data
//! key2===254235!fdjsfhksjd£$%
//! key3===Data with spaces
//! ```
//! 
//! # Limitations
//! - Value or key cannot contain three consecutive equal signs `===`
//! - Key cannot end with 2 consecutive equal signs `==`

use std::fs::{File, read_to_string};
use std::collections::HashMap;
use std::io::prelude::Write;

static DELIMITER: &str = "===";
static CRLF: &str = "\r\n";

/// Takes in a string of the filename you want to load into a dictionary and returns the dictionary.
/// Example at [crate] index
pub fn load(filename: String) -> HashMap<String, String>{

    let mut dict: HashMap<String, String> = HashMap::new();
    let file = read_to_string(filename).expect("Could not open file");
    
    let mut lines = file.split("\r\n").filter(|&x| x != "");

    while let Some(line) = lines.next(){
        let mut pair = line.split(DELIMITER);
        let key = pair.next().unwrap_or("").to_string();
        let value = pair.next().unwrap_or("").to_string();

        dict.insert(key, value);
    }
    dict
}

/// Takes in a string of the filename you want to save to and the dictionary you want to write to it.
/// Example at [crate] index
pub fn save(filename: String, dict: HashMap<String, String>) -> Result<(), std::io::Error> {

    let file = File::open(filename.to_owned()).unwrap_or_else(|_| {
        //println!("Creating {} as it did not exist", filename);
        File::create(filename.to_owned()).unwrap()
    });

    let mut writer = std::io::BufWriter::new(file);

    for (key, value) in &dict {
        let data = format!("{}{}{}{}", key, DELIMITER, value, CRLF);
        writer.write(data.as_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod test;