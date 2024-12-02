use std::convert::Infallible;

use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server, StatusCode};

async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(Body::from("Hello, Rust HTTP Server with Hyper v0.14!"))
        .unwrap();
    Ok(response)
}

#[tokio::main]
async fn main() {
    let make_svc =
        make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle_request)) });

    let address = ([127, 0, 0, 1], 8080).into();
    let server = Server::bind(&address).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}
