//! This is a very basic library to read my AML (Adam Markup Language) files. The syntax is basic, but very strict, for simplicity.
//! This is a way to use "secret" variable files in Rust.
//! 
//! The library reads a .aml file and returns a dictionary of key/value pairs.
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
//! # Usage Example
//! 
//! ```rust
//!     use aml::load;    
//! 
//!     let filename = "example.aml";
//!     let data = load(filename.to_string());
//! 
//!     println!("{:?}", data);
//! ```
//! 
//! The above returns `{"key2": "254235!fdjsfhksjd£$%", "key3": "Data with spaces", "key1": "data"}`
//! 
//! # Limitations
//! - Value or key cannot contain three consecutive equal signs `===`
//! - Key cannot end with 2 consecutive equal signs `==`

use std::fs;
use std::collections::HashMap;

pub fn load(filename: String) -> HashMap<String, String>{

    let mut dict: HashMap<String, String> = HashMap::new();
    let file = fs::read_to_string(filename).expect("Could not open file");
    
    let mut lines = file.split("\r\n");

    while let Some(line) = lines.next(){
        let mut pair = line.split("===");
        let key = pair.next().unwrap_or("").to_string();
        let value = pair.next().unwrap_or("").to_string();

        dict.insert(key, value);
    }

    dict
}

#[cfg(test)]
mod test;