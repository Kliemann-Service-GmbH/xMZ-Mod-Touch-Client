extern crate xmz_client;
extern crate xmz_server;
extern crate rustc_serialize;

use rustc_serialize::json;
use xmz_client::client::Client;
use xmz_server::module::{Module, ModuleType};


fn main() {
    let mut client = Client::new();
    let module = Module::new(ModuleType::RAGAS_CO_NO2);

    let module: Vec<Module> = json::decode(&client.execute("module list").unwrap()).unwrap();

    println!("{:#?}", module);
}
