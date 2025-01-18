use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;


// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {

    let path = "/home/jbc/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html";

    let stdlib_docs = tauri::async_runtime::spawn(static_site(path, true));

    let project_docs = tauri::async_runtime::spawn(async {
        // Empty implementation
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    drop(project_docs);
    drop(stdlib_docs);
}

async fn static_site(url: &str, live_reload: bool) -> Result<(), Box<dyn std::error::Error>> {

    async fn handle_request(_req: Request<Body>) -> Result<Response<Body>, Infallible> {
        Ok(Response::new(Body::from("Hello, World!")))
    }

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| {
        async { Ok::<_, Infallible>(service_fn(handle_request)) }
    });

    let server = Server::bind(&addr).serve(make_svc);

    println!("Listening on http://{}", addr);

    server.await?;


    Ok(())
}