use http_body_util::{BodyExt, Full};
use hyper::body::Bytes;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{body::Incoming as IncomingBody, Method, Request, Response, StatusCode};
use hyper_util::rt::TokioIo;
use std::net::SocketAddr;
use tokio::net::TcpListener;

type GenericError = Box<dyn std::error::Error + Send + Sync>;
type Result<T> = std::result::Result<T, GenericError>;
type BoxBody = http_body_util::combinators::BoxBody<Bytes, hyper::Error>;

async fn index_response() -> Result<Response<BoxBody>> {
    let body = full(Bytes::from("Hello, Rust HTTP Server with v1.5!"));
    let response = Response::builder()
        .status(StatusCode::OK)
        .body(body)
        .unwrap();
    Ok(response)
}

async fn not_found_response() -> Result<Response<BoxBody>> {
    let response = Response::builder()
        .status(StatusCode::NOT_FOUND)
        .body(full("Not found!"))
        .unwrap();
    Ok(response)
}

async fn handle_request(req: Request<IncomingBody>) -> Result<Response<BoxBody>> {
    println!("{:?} {:?}", req.method().as_str(), req.uri());
    match (req.method(), req.uri().path()) {
        (&Method::GET, "/") | (&Method::GET, "/index.html") => index_response().await,
        _ => not_found_response().await,
    }
}

fn full<T: Into<Bytes>>(chunk: T) -> BoxBody {
    Full::new(chunk.into())
        .map_err(|never| match never {})
        .boxed()
}

#[tokio::main]
async fn main() -> Result<()> {
    let address = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = TcpListener::bind(address).await?;

    loop {
        let (stream, _) = listener.accept().await?;
        let io = TokioIo::new(stream);

        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(io, service_fn(handle_request))
                .await
            {
                eprintln!("server error: {:?}", err);
            }
        });
    }
}
