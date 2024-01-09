mod http;
mod language;

use std::path::PathBuf;

use crate::http::file_handler::FileHandler;
use crate::http::util;

use astra::{Body, Request, Response, ResponseBuilder, Server};
use log::{error, info};
use pretty_env_logger;

#[derive(Debug, Clone)]
struct ServerState {
    file_handler: FileHandler,
}

fn server(address: &str, port: i32, state: ServerState) {
    info!("Listening on http://{}:{}", address, port);

    Server::bind(format!("{}:{}", address, port))
        .serve(move |req, _info| route(req, state.clone()))
        .expect("serve failed");
}

fn route(req: Request, ctx: ServerState) -> Response {
    info!("{} {}", req.method(), req.uri().path());

    // TODO? Kinda butchered
    let mut stripped_path = req.uri().to_string();

    if stripped_path.ends_with("/") {
        stripped_path += "index.jshp";
    }

    stripped_path.remove(0);

    let buffer = match ctx.file_handler.read_bytes(&stripped_path) {
        Ok(data) => data,
        Err(_) => {
            return ResponseBuilder::new()
                .header("Content-Type", "text/html")
                .body(Body::new("<h1>404 File Not Found</h1>"))
                .unwrap();
        }
    };

    let file_path = PathBuf::from(&stripped_path);
    let content_type: String;

    if let Some(file_extension) = file_path.extension() {
        content_type = util::get_content_type(file_extension.to_str().unwrap()).to_owned();
    } else {
        content_type = String::from("text/plain");
    }

    ResponseBuilder::new()
        .header("Content-Type", content_type)
        .body(Body::new(buffer))
        .unwrap()
}

fn main() {
    pretty_env_logger::init();

    let file_handler = match FileHandler::new("./serve") {
        Ok(handler) => handler,
        Err(e) => {
            error!("FileHandler error: {}", e);
            std::process::exit(1);
        }
    };

    let state = ServerState { file_handler };

    server("127.0.0.1", 3000, state);
}
