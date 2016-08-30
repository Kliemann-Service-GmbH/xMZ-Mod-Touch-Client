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

    let mut client = Client::new();
    // let module = Module::new(ModuleType::RAGAS_CO_NO2);

    let module: Vec<Module> = json::decode(&client.execute("module list").unwrap()).unwrap();

    info!("{:#?}", module);
}
