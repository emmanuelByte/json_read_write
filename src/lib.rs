use serde::{Deserialize, Serialize};
use std::{error::Error, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Person {
    pub name: String,
    pub age: u8,
    pub city: String,
}

impl Person {
    pub fn new(name: &str, age: u8, city: &str) -> Self {
        Self {
            name: name.to_string(),
            age,
            city: city.to_string(),
        }
    }
    pub fn read_json(filename: &str) -> Result<Vec<Self>, Box<dyn Error>> {
        // Read the file to a string
        let contents = fs::read_to_string(filename)?;

        // Deserialize the string into the Person struct
        let people: Result<Vec<Self>, serde_json::Error> = serde_json::from_str(&contents);

        // Match on the result and convert the error type if needed
        people.map_err(|e| Box::new(e) as Box<dyn Error>)
    }
    pub fn write_to_json(data: Vec<Self>, filename: &str) -> Result<(), Box<dyn Error>> {
        let serialized_data = serde_json::to_string(&data)?;

        fs::write(filename, serialized_data)?;

        Ok(())
    }
}
