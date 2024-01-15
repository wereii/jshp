use std::{path::PathBuf, sync::Arc};

use crate::{error::Error, io::FileHandler, server::types::MimeType};
use astra::{Body, Request, Response, ResponseBuilder, Server as AstraServer};
use log::info;

#[derive(Debug, Clone)]
struct ServerState {
    file_handler: FileHandler,
}

#[derive(Debug, Clone)]
pub struct Server {
    address: String,
    port: i32,
    state: Arc<ServerState>,
}

impl Server {
    pub fn new(address: &str, port: i32, serve_dir: &str) -> Result<Server, Error> {
        let file_handler = match FileHandler::new(serve_dir) {
            Ok(handler) => handler,
            Err(e) => {
                return Err(e.into());
            }
        };

        Ok(Self {
            address: address.to_string(),
            port,
            state: Arc::new(ServerState { file_handler }),
        })
    }

    pub fn run(&self) {
        info!("Listening on http://{}:{}", self.address, self.port);
        let state = self.state.clone();
        AstraServer::bind(format!("{}:{}", self.address, self.port))
            .serve(move |req, _info| route(req, state.clone()))
            .expect("serve failed");
    }
}

fn route(req: Request, ctx: Arc<ServerState>) -> Response {
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
    let content_type = MimeType::from(file_path.extension()).to_content_type_string();

    ResponseBuilder::new()
        .header("Content-Type", content_type)
        .body(Body::new(buffer))
        .unwrap()
}
