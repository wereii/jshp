mod error;
mod http;
mod io;
mod language;

use http::server::Server;
use language::v8::V8State;
use log::{error, trace};

fn main() {
    pretty_env_logger::init();

    let mut v8 = V8State::new();
    v8.init();

    let mut do_cleanup = || {
        trace!("Cleaning up");
        v8.dispose();
    };

    let server = match Server::new("127.0.0.1", 3000, "./serve") {
        Ok(server) => server,
        Err(e) => {
            error!("Server error: {}", e);
            do_cleanup();
            std::process::exit(1);
        }
    };
    server.run();
    do_cleanup();
}
