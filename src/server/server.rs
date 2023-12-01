use http_body_util::combinators::BoxBody;
use http_body_util::{BodyExt, Full};
use hyper::body::{Bytes, Incoming};
use hyper::client::conn::http1::Builder;
use hyper::server::conn::http1;
use hyper::service::service_fn;
use hyper::{Request, Response};
use hyper_util::rt::TokioIo;
use std::env;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};

pub async fn main_server(addr: SocketAddr) {
    let listener = TcpListener::bind(addr)
        .await
        .expect("failed to bind to port");
    println!("Listening on http://{}", addr);
    loop {
        let (stream, _) = listener
            .accept()
            .await
            .expect("failed to accept connection");
        let io = TokioIo::new(stream);
        tokio::task::spawn(async move {
            if let Err(err) = http1::Builder::new()
                .serve_connection(
                    io,
                    service_fn(move |req| async move {
                        match req
                            .uri()
                            .path()
                        {
                            "api" => api_handler(req).await,
                            "ui" => frontend_ssr_handler(req).await,
                            "entry" => frontend_ssr_handler(req).await,
                            _ => frontend_ssr_handler(req).await,
                        }
                    }),
                )
                .await
            {
                println!("Error serving connection: {:?}", err);
            }
        });
    }
}

async fn api_handler(
    _: Request<impl hyper::body::Body>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::builder()
        .status(200)
        .body(
            Full::new("Hello, World!".into())
                .map_err(|e| match e {})
                .boxed(),
        )
        .expect("Failed to build response"))
}
async fn frontend_ssr_handler(
    req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    match std::env::var("MODE") {
        Ok(mode) => match mode.as_str() {
            "DEV" => Ok(dev_server_handler(req)
                .await
                .expect("Failed to handle request")),
            "PROD" => Ok(prod_server_handler(req)
                .await
                .expect("Failed to handle request")),
            _ => Ok(dev_server_handler(req)
                .await
                .expect("Failed to handle request")),
        },
        Err(_) => Ok(no_mode_handler(req)
            .await
            .expect("Failed to handle request")),
    }
}
async fn dev_server_handler(
    main_req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    let dev_server_url = format!(
        "http://localhost:{}{}",
        env::var("DEV_PORT").expect("Failed to get dev port"),
        main_req.uri().path()
    );
    let url = dev_server_url
        .parse::<hyper::Uri>()
        .expect("Failed to parse dev server url");
    let host = url.host().expect("uri has no host");
    let port = url.port_u16().unwrap_or(80);
    let stream = TcpStream::connect((host, port)).await
        .expect("Failed to connect to dev server");
    let io = TokioIo::new(stream);
    let (mut sender, conn) = Builder::new()
        .preserve_header_case(true)
        .title_case_headers(true)
        .handshake(io)
        .await
        .expect("Failed to handshake with dev server");
    tokio::task::spawn(async move {
        if let Err(err) = conn.await {
            println!("Error serving connection: {:?}", err);
        }
    });
    let resp = sender
        .send_request(main_req)
        .await
        .expect("Failed to send request to dev server");
    Ok(resp.map(|body| body.boxed()))
}

async fn prod_server_handler(
    req: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::new(req.boxed()))
}

async fn no_mode_handler(
    _: Request<Incoming>,
) -> Result<Response<BoxBody<Bytes, hyper::Error>>, hyper::Error> {
    Ok(Response::builder()
        .status(200)
        .body(
            Full::new("Hello, World!".into())
                .map_err(|e| match e {})
                .boxed(),
        )
        .expect("Failed to build response"))
}