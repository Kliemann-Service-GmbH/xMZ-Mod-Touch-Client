#[macro_use] extern crate log;
extern crate env_logger;
extern crate rustc_serialize;
extern crate xmz_client;
extern crate xmz_server;

use rustc_serialize::json;
use xmz_client::client::Client;
use xmz_server::module::{Module};


fn main() {
    trace!("Initialisiere den Logger");
    env_logger::init().unwrap();

    let mut module: Vec<Module> = vec![];
    let mut client = Client::new();
    info!("Setze Empfangs Timeout auf 1sek");
    let _ = client.set_socket_receive_timeout(1000);

    match client.execute("module list") {
        Ok(client) => { module = json::decode(&client).unwrap_or(vec![]); }
        Err(err) => {
            // println!("{:#?}", err);
            println!("description: {:#?}", ::std::error::Error::description(&err));
            println!("cause: {:#?}", ::std::error::Error::cause(&err).unwrap());
        }
    }

    println!("Module: {:#?}", module);
}
