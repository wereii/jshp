mod node_worker;
mod parse;
mod v8;
mod file_handler;

use std::path::{Path, PathBuf};

use astra::{Body, Response, ResponseBuilder, Server, Request};
use file_handler::{FileHandler, FileHandlerError};
use log::{error, info};
use pretty_env_logger;

#[derive(Debug, Clone)]
struct ServerState {
    file_handler: FileHandler,
}


fn server(address: &str, port: i32, state: ServerState) {
    println!("Listening on http://{}:{}", "localhost", 3000);

     Server::bind(format!("{}:{}", address, port))
         .serve(move |req, _info| route(req, state.clone()))
         .expect("serve failed");
}


fn route(req: Request, ctx: ServerState) -> Response {

    info!("Requested page: {}", req.uri());
    println!("Requested page: {}", req.uri());

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
                .body(Body::new("<h1>404 File not found</h1>"))
                .unwrap();
        }
    };


    let file_path = PathBuf::from(&stripped_path);
    let content_type: String;

    if let Some(extension) = file_path.extension() {
        let extenstion = extension.to_str().expect("Should be convertible from OsStr to Str");

        content_type = match extenstion {
            "html" => "text/html",
            "jpg" => "image/jpeg",
            "webp" => "image/webp",
            "png" => "image/png",
            "svg" => "image/svg+xml",
            _ => "text/plain",
        }.to_string();
    }
    else {
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

    let state = ServerState {
        file_handler
    };

    server("127.0.0.1", 3000, state);
}



