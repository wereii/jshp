mod node_worker;
mod parse;
mod v8;

use std::io;
use astra::{Body, Response, ResponseBuilder, Server};
use log::{error, info};
use pretty_env_logger;

fn locate_files(path: &str) -> io::Result<Vec<String>> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    dirs.push(path.to_string());
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir)? {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.to_str().unwrap().to_string());
            } else if path.as_path().ends_with(".jshp") {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }
    Ok(files)
}

fn server() {
    // Server::bind("localhost:3000")
    //     .serve(|_req, _info| {
    //         ResponseBuilder::new()
    //             .header("Content-Type", "text/html")
    //             .body(Body::new("Hi"))
    //             .unwrap()
    //     })
    //     .expect("serve failed");
    todo!("Implement server");
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
    for file in files {
        println!("{:?}", file);
    }
    server();
}



