/// Bei dieser Lösung ist die Socket Endpoint Initalisierung schon mal sehr cool.

extern crate nanomsg;
#[macro_use] extern crate clap;

use client::Client;

mod client;
mod cmd;


fn main() {
    println!("{:?}", cmd::read_command());
    // let mut client = Client::new("A client".to_string());
    // client.run();
}
