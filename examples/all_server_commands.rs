extern crate xmz_client;

use xmz_client::client::Client;



fn main() {
    let mut client = Client::new();

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
        client.request(msg);
        ::std::thread::sleep(::std::time::Duration::new(1, 0));
    }
}
