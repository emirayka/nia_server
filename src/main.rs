mod error;
mod protocol;
mod server;
mod utils;

fn main() {
    let mut s = server::Server::new();

    s.start();
}
