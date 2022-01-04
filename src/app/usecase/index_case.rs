use serde::{ Serialize };

#[derive(Serialize)]
pub struct IndexOutput {
    name: String,
    age: u32
}

pub fn execute() -> IndexOutput {
    IndexOutput {
        name: String::from("Test"),
        age: 20
    }
}

