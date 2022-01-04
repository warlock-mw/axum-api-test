use serde::{ Serialize };
use crate::domain::entity::customer::Customer;
use crate::app::repository::customer_repository::{ 
    get_all,
    add_all 
};
use crate::adapter::transfer::customers_transfer::{ AddRequest };

#[derive(Debug, Serialize)]
pub struct RootOutput {
    pub items: Vec<Customer>
}

#[derive(Debug)]
pub struct AddInput {
    pub items: Vec<Customer>
}

impl From<AddRequest> for AddInput {
    fn from(req: AddRequest) -> Self {
        Self { items: req.items.into_iter().map(|x| Customer::from(x)).collect() }
    }
}

#[derive(Debug, Serialize)]
pub struct AddOutput {
    pub count: usize
}

pub fn root_handle() -> RootOutput {
    RootOutput {
        items: get_all()
    }
}

pub fn add_handle(input: AddInput) -> AddOutput {
    let c = add_all(input.items);
    AddOutput { count: c }
}
