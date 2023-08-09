use json_read_write::Person;

fn main() {
    let person = Person::new("John", 30, "New York");
    let filename = "user.json";
    match Person::write_to_json(vec![person], filename) {
        Ok(_) => println!("Data written successfully"),
        Err(e) => println!("Error: {}", e),
    }

    let contents = Person::read_json(filename);
    match contents {
        Ok(content) => println!("{:?}", content),
        Err(e) => println!("Error: {}", e),
    }
}
