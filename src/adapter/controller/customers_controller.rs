use axum::{
    response,
    extract
};
use crate::app::usecase::customers_case::{ 
    RootOutput, 
    root_handle,
    AddInput,
    AddOutput,
    add_handle
};
use crate::adapter::transfer::customers_transfer::{
    AddRequest
};

pub async fn root() -> response::Json<RootOutput> {
    response::Json(root_handle())
}

pub async fn add(extract::Json(req): extract::Json<AddRequest>) -> response::Json<AddOutput> {
    response::Json(add_handle(AddInput::from(req)))
}