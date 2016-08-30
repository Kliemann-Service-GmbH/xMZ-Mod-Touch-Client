#![doc(html_logo_url = "https://raw.githubusercontent.com/zzeroo/xmz-snapshot/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/zzeroo/xmz-snapshot/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![recursion_limit = "1024"]

#[macro_use] extern crate clap;
#[macro_use]extern crate error_chain;
#[macro_use] extern crate log;
extern crate env_logger;
extern crate nanomsg;

use client::Client;

mod client;
mod cmd;
mod errors;


fn main() {
    env_logger::init().unwrap();

    let message = cmd::read_command();
    let mut client = Client::new();
    info!("{:?}", client.execute(message));
}
