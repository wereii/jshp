mod node_worker;
mod parse;

use astra::{Body, Response, ResponseBuilder, Server};

fn locate_files(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    let mut dirs = Vec::new();
    dirs.push(path.to_string());
    while let Some(dir) = dirs.pop() {
        for entry in std::fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                dirs.push(path.to_str().unwrap().to_string());
            } else {
                files.push(path.to_str().unwrap().to_string());
            }
        }
    }
    files
}


fn main() {
    Server::bind("localhost:3000")
        .serve(|_req, _info| {
            ResponseBuilder::new()
                .header("Content-Type", "text/html")
                .body(Body::new("Hi"))
                .unwrap()
        })
        .expect("serve failed");
}
