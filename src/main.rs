mod error;
mod protocol;
mod server;
mod utils;

fn main() {
    server::Server::new().start();
}
