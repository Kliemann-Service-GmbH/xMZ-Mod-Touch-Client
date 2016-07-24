use nanomsg::{Socket, Protocol, Error};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub struct Client {
    socket: Socket,
}

impl Client {
    pub fn new() -> Self {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        let mut endpoint = socket.connect("ipc:///tmp/xmz-server.ipc").unwrap();
        // socket.set_send_timeout(1000);
        socket.set_receive_timeout(2000);

        Client {
            socket: socket,
        }
    }

    /// Führt ein Befehl aus (1. Parameter) und liefert das Ergebnis als Option<String>
    ///
    pub fn execute<T: AsRef<str>>(&mut self, message: T) -> Option<String> {
        let mut reply = String::new();
        let request = format!("{}", message.as_ref());

        match self.socket.write_all(request.as_bytes()) {
            Ok(..) => { println!("Sende: {}", request); }
            Err(err) => { println!("Fehler beim Senden des Requests: {}", request); }
        }

        match self.socket.read_to_string(&mut reply) {
            Ok(_) => {  }
            Err(err) => { println!("Konnte Reply nicht empfangen: {}", err); }
        }

        Some(reply)
    }

    pub fn request<T: AsRef<str>>(&mut self, message: T) {
        let mut reply = String::new();
        let request = format!("{}", message.as_ref());

        match self.socket.write_all(request.as_bytes()) {
            Ok(..) => {
                println!("Sende: {}", request);
            }
            Err(err) => { println!("Fehler beim Senden des Requests: {}", request); }
        }

        match self.socket.read_to_string(&mut reply) {
            Ok(_) => {
                println!("'{}' empfangen", reply);
                reply.clear();
            }
            Err(err) => {
                println!("Konnte Reply nicht empfangen: {}", err);
            }
        }
    }

}
