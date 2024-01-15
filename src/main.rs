mod error;
mod io;
mod javascript;
mod server;

use crate::io::FileHandler;
use crate::javascript::parse::process_file;
use crate::javascript::v8::{CodeMetadata, V8Handle};
use astra::{Body, ConnectionInfo, Request, Response, ResponseBuilder, Server, Service};
use javascript::v8::V8State;
use log::{error, info, trace};
use std::env::{set_var, var};
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

struct RequestHandler {
    v8: V8Handle,
    file_handler: FileHandler,
}

impl RequestHandler {
    pub fn new(v8: V8Handle, serve_dir: String) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            v8,
            file_handler: FileHandler::new(serve_dir.as_str())?,
        })
    }

    fn handle_jshp(&self, path: PathBuf) -> Result<Response, Box<dyn Error>> {
        // optimally this would be all done iteratively (read file/buffer, parse then execute spans)
        let preprocessed_file = process_file(path)?;
        let mut buffer = preprocessed_file.raw.clone();

        for code_span in preprocessed_file.code_spans {
            let result = self
                .v8
                .evaluate(
                    code_span.code.as_str(),
                    Some(CodeMetadata {
                        file_name: preprocessed_file.file_path.to_str().unwrap().to_string(),
                        start_line: code_span.start_position.0 as i32,
                        start_offset: code_span.start_position.1 as i32,
                    }),
                )
                .map_or_else(|e| e.to_string(), |s| s);

            trace!("result: {}", result);

            buffer.replace_range(
                code_span.start_position.1..code_span.stop_position.1,
                result.as_str(),
            );
        }

        Ok(self
            .base_response()
            .header("Content-Type", "text/html")
            .body(Body::new(buffer))
            .unwrap())
    }

    fn base_response(&self) -> ResponseBuilder {
        ResponseBuilder::new().header("server", "jshp-0.1")
    }

    fn response_error(&self, _error: String) -> Response {
        self.base_response()
            .header("Content-Type", "text/html")
            .status(500)
            .body(Body::new("<h1>500 Internal Server Error</h1>"))
            .unwrap()
    }

    fn response_not_found(&self) -> Response {
        self.base_response()
            .header("Content-Type", "text/html")
            .status(404)
            .body(Body::new("<h1>404 File Not Found</h1>"))
            .unwrap()
    }
}

impl Service for RequestHandler {
    fn call(&self, request: Request, conn_info: ConnectionInfo) -> Response {
        info!(
            "{} - {} {}",
            request.method(),
            request.uri().path(),
            conn_info
                .peer_addr()
                .map_or("unknown".to_string(), |addr| addr.ip().to_string())
        );
        let req_path = request.uri().path().to_string(); // we should probably escape the path

        // TODO: Separate this, jshp/static files
        return match self.file_handler.get_file(req_path.as_str()) {
            Ok(file_path) => {
                trace!("open file: {}", file_path.to_str().unwrap());
                if file_path.extension().unwrap() == "jshp" {
                    self.handle_jshp(file_path).unwrap_or_else(|e| {
                        error!("Error processing jshp file: {}", e);
                        return self.response_error(e.to_string());
                    })
                } else {
                    let fd = File::open(file_path).unwrap();
                    return self
                        .base_response()
                        .header("Content-Type", "text/html")
                        .body(Body::wrap_reader(fd))
                        .unwrap();
                }
            }
            Err(e) => {
                error!("Error getting file: {}", e);
                self.response_not_found()
            }
        };
    }
}

fn main() {
    if cfg!(debug_assertions) {
        if var("RUST_LOG").is_err() {
            set_var("RUST_LOG", "trace");
        };
    };
    pretty_env_logger::init_timed();

    let v8handle = V8State::init();

    let handler = match RequestHandler::new(v8handle, "./tests/test_files".to_string()) {
        Ok(handler) => handler,
        Err(e) => {
            error!("Startup error: {}", e);
            V8State::dispose();
            std::process::exit(1);
        }
    };

    let address = "127.0.0.1";
    let port = 8080;

    info!("Listening on http://{}:{}", address, port);
    Server::bind(format!("{}:{}", address, port))
        .serve(handler)
        .expect("failed to start server");

    V8State::dispose();
}
