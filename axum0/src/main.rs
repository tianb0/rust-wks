use axum::{Router, routing};

#[tokio::main]
async fn main() {
    // build our appapplication with a single route
    let app = Router::new()
        .route("/", routing::get(|| async { "Hello, me"}));

    // run it with hyper on localhost:3000
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
