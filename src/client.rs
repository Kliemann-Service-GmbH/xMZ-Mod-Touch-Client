use nanomsg::{Socket, Protocol, Error};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub struct Client {
    name: String,
    socket: Socket,
}

impl Client {
    pub fn new(name: String) -> Self {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        let mut endpoint = socket.connect("ipc:///tmp/xmz-client.ipc").unwrap();

        Client {
            name: name,
            socket: socket,
        }
    }

    pub fn run(&mut self) {
        let mut reply = String::new();

        let request = format!("{}: led set 1", self.name);

        match self.socket.write_all(request.as_bytes()) {
            Ok(..) => {
                println!("Send {}", request);
            }
            Err(err) => { println!("Failed to send: {}", request); }
        }

        match self.socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("{} hat '{}' empfangen", self.name, reply);
                reply.clear();
            }
            Err(err) => {
                println!("{} konnte Reply nicht empfangen: {}", self.name, err);
            }
        }
    }
}
