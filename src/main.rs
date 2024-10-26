use axum::{extract::Path, routing::get, Router};

async fn add(Path((a, b)): Path<(u32, u32)>) -> String {
    (a + b).to_string()
}

async fn mult(Path((a, b)): Path<(u32, u32)>) -> String {
    (a * b).to_string()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/add/:a/:b", get(add))
        .route("/mult/:a/:b", get(mult));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
