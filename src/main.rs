extern crate protobuf;
extern crate nia_protocol_rust;
extern crate nia_interpreter_core;

mod error;
mod commands;
mod server;

fn main() {
    let mut s = server::Server::new();

    s.start_listening();

    // let device_info = commands::get_device_info(
    //     "/dev/input/event6"
    // ).expect("");
    //
    // println!("{:?}", device_info);
}
