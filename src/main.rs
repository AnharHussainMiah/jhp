use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
    response::Response,
    routing::any,
};

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

mod guru;
mod jhp;

use crate::jhp::compile;
use tokio::fs;

const ROOT: &str = "./public";

#[tokio::main]
async fn main() {
    let app = Router::new().fallback(any(handle_request));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    println!("Serving {} on http://{}", ROOT, addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    let uri_path = req.uri().path().trim_start_matches('/');

    let path = Path::new(uri_path); // note: prevent ../ path traversal.

    if path
        .components()
        .any(|component| matches!(component, std::path::Component::ParentDir))
    {
        return Err(StatusCode::FORBIDDEN);
    }

    let mut file_path: PathBuf = Path::new(ROOT).join(path);

    if fs::metadata(&file_path)
        .await
        .map(|metadata| metadata.is_dir())
        .unwrap_or(false)
    {
        file_path.push("index.html");
    }

    if file_path
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("jhp")
    {
        return handle_jhp(&file_path).await;
    }

    let data = fs::read(&file_path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let mime = mime_guess::from_path(&file_path).first_or_octet_stream();

    Response::builder()
        .header("content-type", mime.as_ref())
        .body(Body::from(data))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn handle_jhp(path: &Path) -> Result<Response<Body>, StatusCode> {
    let source = fs::read_to_string(path)
        .await
        .map_err(|_| StatusCode::NOT_FOUND)?;

    let processed = process_jhp(&source).await?;

    Response::builder()
        .header("content-type", "text/html; charset=utf-8")
        .body(Body::from(processed))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn process_jhp(source: &str) -> Result<String, StatusCode> {
    let compiled = compile(source);

    let jhp = jhp::js(compiled.clone()).unwrap_or_else(|e| {
        let error = guru::error(&e.to_string());
        println!("DEBUG: JHP Source:\n{}\n", compiled);
        error
    });

    Ok(jhp)
}
