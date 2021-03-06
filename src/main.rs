mod error;
mod protocol;
mod server;
mod utils;

use chrono::Local;
use env_logger::Builder;
use log::debug;
use log::info;
use log::warn;
use log::LevelFilter;
use std::io::Write;

fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                Local::now().format("%Y-%m-%dT%H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .filter(Some("ws"), LevelFilter::Error)
        .init();

    let server = match server::Server::new() {
        Ok(server) => server,
        Err(error) => {
            println!("Cannot start server because of error:");
            println!("{:?}", error);
            return;
        }
    };

    server.start();
}
