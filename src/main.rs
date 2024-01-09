mod http;
mod io;
mod language;
mod error;

use http::server::Server;
use log::error;


fn main() {
    pretty_env_logger::init();

    let server = match Server::new("127.0.0.1", 3000, "./serve"){
        Ok(server) => server,
        Err(e) => {
            error!("Server error: {}", e);
            std::process::exit(1);
        }
    };

    server.run();
}
