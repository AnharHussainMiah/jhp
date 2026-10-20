use axum::{
    Router,
    body::Body,
    body::to_bytes,
    http::{Request, StatusCode},
    response::Response,
    routing::any,
};

use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
};

use std::collections::HashMap;

mod guru;
mod host;
mod includes;
mod jhp;
mod logo;

use crate::jhp::{JHPResponse, compile};
use tokio::fs;

const ROOT: &str = "./public";

#[tokio::main]
async fn main() {
    println!("{}", logo::render());

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
        file_path.push("index.jhp"); // todo: may need to think about when an actual index.html exists e.g SPA
    }

    // if the file path does not exist but we have a "global.jhp" on disk
    // we should instead over ride the file_path with the global.jhp to handle it instead

    let global_jhp: PathBuf = Path::new(ROOT).join("global.jhp");

    if !file_path.exists() && global_jhp.exists() {
        file_path = global_jhp;
    }

    if file_path
        .extension()
        .and_then(|extension| extension.to_str())
        == Some("jhp")
    {
        return handle_jhp(&file_path, req).await;
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

async fn handle_jhp(path: &Path, req: Request<Body>) -> Result<Response<Body>, StatusCode> {
    if !path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let resolved = includes::resolve(&format!("{}", path.display()));
    let processed = process_jhp(&resolved, req).await?;

    let mut builder = Response::builder();

    for (key, value) in processed.headers {
        builder = builder.header(key, value);
    }

    builder
        .status(StatusCode::from_u16(processed.status).unwrap())
        .body(Body::from(processed.body))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

async fn process_jhp(source: &str, req: Request<Body>) -> Result<JHPResponse, StatusCode> {
    let compiled = compile(&source);

    let mut json = serde_json::Map::new();

    let headers = req.headers();
    let uri = req.uri();

    for (name, value) in headers {
        json.insert(
            name.to_string(),
            serde_json::Value::String(value.to_str().unwrap_or("").to_string()),
        );
    }

    let header_data = serde_json::to_string(&serde_json::Value::Object(json)).unwrap();

    let env_map: serde_json::Map<String, serde_json::Value> = std::env::vars()
        .map(|(key, value)| (key, serde_json::Value::String(value)))
        .collect();

    let env = serde_json::to_string(&env_map).unwrap_or_default();

    let compiled = format!("METHOD = '{}';\n", req.method()) + &compiled;
    let compiled = format!("URI = '{}';\n", uri) + &compiled;
    let compiled = format!("PATH = '{}';\n", uri.path()) + &compiled;
    let compiled = format!("QUERYSTRING = '{}';\n", uri.query().unwrap_or("")) + &compiled;
    let compiled = format!("ENV = {};\n", env) + &compiled;
    let compiled = format!("HEADERS = {};\n", header_data) + &compiled;

    // ugh some nasty stuff, will need to clean up later

    let bytes = to_bytes(req.into_body(), 1024 * 1024).await.unwrap(); // max body is 1MB
    let body = String::from_utf8(bytes.to_vec()).unwrap();

    let compiled = format!("BODY = `{}`;\n", escape(&body)) + &compiled;

    let jhp = jhp::js(compiled.clone()).unwrap_or_else(|e| {
        let error = guru::error(&e.to_string());
        println!("DEBUG: JHP Source:\n{}\n", compiled);
        JHPResponse {
            status: 501,
            headers: HashMap::new(),
            body: error,
        }
    });

    Ok(jhp)
}

fn escape(s: &str) -> String {
    s.replace('\\', "\\\\")
        .replace('`', "\\`")
        .replace("${", "\\${")
}
