use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};

async fn handle_request(_: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    let response = Response::new(Body::from("Hello, Rust Web Server!"));
    Ok(response)
}

#[tokio::main]
async fn main() {
    let addr = ([127, 0, 0, 1], 8080).into();

    let make_service = make_service_fn(|_conn| {
        async { Ok::<_, hyper::Error>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_service);

    println!("Rust Web Server running on http://{}", addr);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}