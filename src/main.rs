mod node_worker;
mod parse;
mod v8;

use std::io;
use astra::{Body, Response, ResponseBuilder, Server, Request};
use log::error;
use pretty_env_logger;

#[derive(Debug, Clone)]
struct ServerState {
    files: Vec<String>
}

fn locate_files(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    dirs.push(path.to_string());
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir)? {
            
            let entry = entry.unwrap();
            let entry_dir = entry.path();

            if entry_dir.is_dir() {
                dirs.push(entry_dir.to_str().unwrap().to_string());
            } else {
                let unix_path = format!("./{}/", path);
                let windows_path = format!("{}\\", path);
                let new_filepath = entry_dir
                    .to_str().unwrap()
                    .to_string()
                    .replace(&unix_path, "")
                    .replace(&windows_path, "");

                files.push(new_filepath);
            }
        }
    }
    Ok(files)
}

fn server(address: &str, port: i32, state: ServerState) {
    println!("Listening on http://{}:{}", "localhost", 3000);

     Server::bind(format!("{}:{}", address, port))
         .serve(move |req, _info| route(req, state.clone()))
         .expect("serve failed");
}


fn route(req: Request, state: ServerState) -> Response {

    println!("Requested path: {}", req.uri());

    let mut body = String::from("<h1>Available files</h1><br>");

    for file in &state.files {
        body += &format!("{}<br>", file);
    }

    ResponseBuilder::new()
        .header("Content-Type", "text/html")
        .body(Body::new(body))
        .unwrap()
}

fn main() {
    pretty_env_logger::init();

    let files = match locate_files("./serve") {
        Ok(v) => v,
        Err(e) => {
            error!("Failed to locate files: {}", e);
            std::process::exit(1);
        }
    };

    for file in &files {
        println!("{:?}", file);
    }

    server("127.0.0.1", 3000, ServerState{
        files
    });
}



