mod error;
mod http;
mod io;
mod javascript;

use http::server::Server;
use javascript::v8::V8State;
use log::{error, trace};

fn main() {
    pretty_env_logger::init();

    let v8handle = V8State::init();

    let server = match Server::new("127.0.0.1", 3000, "./serve") {
        Ok(server) => server,
        Err(e) => {
            error!("Server error: {}", e);
            V8State::dispose();
            std::process::exit(1);
        }
    };
    server.run();
    V8State::dispose();
}
