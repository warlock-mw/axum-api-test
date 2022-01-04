use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct AddRow {
    pub name: String
}

#[derive(Debug, Deserialize)]
pub struct AddRequest {
    pub items: Vec<AddRow>
}