#[macro_use] extern crate log;
extern crate env_logger;
extern crate xmz_client;

use xmz_client::client::Client;



fn main() {
    trace!("Initialisiere den Logger");
    env_logger::init().unwrap();

    trace!("Initialisiere den Client");
    let mut client = Client::new_with_send_timeout(1000);
    // Alle möglichen Befehle
    let messages: Vec<_> = vec![
        "led list",
        "led test",
        "led set 1",
        "led get 1",
        "led toggle 1",
        "led clear 1",

        "relais list",
        "relais test",
        "relais set 1",
        "relais get 1",
        "relais toggle 1",
        "relais clear 1",

        "server get modbus_device",
        "server set modbus_device /dev/ttyUSB0",
        "server get modbus_device",

        "module new",
        "module list",
        "module show 1",
        "module get modbus_slave_id 1",
        "module set modbus_slave_id 100 1",
    ];

    for msg in messages {
        client.execute(msg);
        ::std::thread::sleep(::std::time::Duration::new(1, 0));
    }
}
