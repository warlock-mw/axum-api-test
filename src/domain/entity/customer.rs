use serde::{ Serialize };
use crate::adapter::transfer::customers_transfer::AddRow;

#[derive(Debug, Serialize)]
pub struct Customer {
    pub id: u32,
    pub name: String
}

impl From<AddRow> for Customer {
    fn from(row: AddRow) -> Self {
        Self { id: 1, name: row.name }
    }
}