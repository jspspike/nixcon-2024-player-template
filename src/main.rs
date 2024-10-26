use axum::{
    extract::Path, routing::get, Router
};

async fn handle(
    Path((a, b)): Path<(u32, u32)>,
) -> String {
    (a + b).to_string()
}

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello, World!" }))
.route("/add/:a/:b", get(handle));
        ;

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
