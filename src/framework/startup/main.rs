use axum::{
    Router, 
    routing::get,
    routing::post
};
use axum_api_test_lib::adapter::controller::{
    index_controller,
    customers_controller
};

#[tokio::main]
async fn main() {
    let customers_router = Router::new()
        .route("/", get(customers_controller::root))
        .route("/", post(customers_controller::add));

    let app = Router::new()
        .route("/", get(index_controller::root))
        .route("/get", get(index_controller::find))
        .nest("/customers", customers_router);

    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
