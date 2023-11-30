use axum::{
    routing::get,
    Router, http::{Response, HeaderMap},
};
use url::Url;
use std::env;

pub async fn main_server() {
    let handler;
    if env::var("MODE") == Ok("DEV".to_string()) {
          async fn dev_server_handler (req: axum::extract::Request) {
            let dev_server = Url::parse(format!("http://localhost:{}", env::var("DEV_PORT").expect("Failed to read DEV_PORT from .env file")).as_str()).expect("Failed to parse Dev Server URL");
                let client = reqwest::Client::new();
                let formatted_url = format!("{}://{}:{}{}", dev_server.scheme(),
                 dev_server.host_str().expect("Failed to get Dev Server host"), 
                 dev_server.port().expect("Failed to get Dev Server port"),
                 req.uri().path());
                println!("Requesting {}", formatted_url);
                let res = client.get(formatted_url).send().await.expect("Failed to send request to Dev Server");
                let content_type = res.headers().get("Content-Type").expect("Failed to get Content-Type header from Dev Server").to_str().expect("Failed to convert Content-Type header to string").to_string();
                let res_status = res.status().as_u16();
                let body = res.text().await.expect("Failed to get response body from Dev Server").to_string();
                let mut res = Response::new(body);
                let mut headers = res.headers_mut();
                for (key, value) in headers.iter_mut() {
                    headers.insert(key, value.to_owned());
                }
                
                      }
    } else if env::var("MODE") == Ok("PROD".to_string()) {
        handler = get(|| async { "Hello, World!" });
        } else {
            panic!("MODE not set in .env file");
    }
    let app = Router::new().route("/*wildcard", handler);



    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Listening at http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
