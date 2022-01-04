use axum::response;
use crate::app::usecase::index_case::{ 
    IndexOutput, 
    execute
};

pub async fn root() -> &'static str {
    "Hello, World!"
}

pub async fn find() -> response::Json<IndexOutput> {
    response::Json(execute())
}