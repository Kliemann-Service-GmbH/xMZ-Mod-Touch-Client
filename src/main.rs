/// Bei dieser Lösung ist die Socket Endpoint Initalisierung schon mal sehr cool.

extern crate nanomsg;

use nanomsg::{Socket, Protocol, Error};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

struct Client {
    name: String,
}

impl Client {
    fn new(name: String) -> Self {
        Client {
            name: name
        }
    }

    pub fn run(&self) {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        let mut endpoint = socket.connect("ipc:///tmp/client.ipc").unwrap();
        let client_name = self.name.clone();

        let client_thread = thread::spawn(move || {

            let mut reply = String::new();

            loop {
                let request = format!("{}: led set 1", client_name);

                match socket.write_all(request.as_bytes()) {
                    Ok(..) => {
                        println!("Send {}", request);
                    }
                    Err(err) => { println!("Failed to send: {}", request); }
                }

                match socket.read_to_string(&mut reply) {
                    Ok(_) => {
                        println!("{} hat '{}' empfangen", client_name, reply);
                        reply.clear();
                    }
                    Err(err) => {
                        println!("{} konnte Reply nicht empfangen: {}", client_name, err);
                        break
                    }
                }

                thread::sleep(Duration::new(1, 0));
            }
        });
        // client_thread.join();
    }

}



fn server() {
    let mut socket = Socket::new(Protocol::Rep).unwrap();
    let mut endpoint = socket.connect("ipc:///tmp/srv.ipc").unwrap();

    let mut request = String::new();

    let server_thread = thread::spawn(move || {
        println!("Server ist bereit!");

        loop {
            match socket.read_to_string(&mut request) {
                Ok(_) => {
                    println!("Server Empfang: {}", request);

                    match socket.write_all("OK".as_bytes()) {
                        Ok(..) => { println!("Server sendet: OK"); }
                        Err(err) => {
                            println!("Server konnte kein OK senden");
                            break
                        }
                    }
                    thread::sleep(Duration::from_millis(400));
                    request.clear();
                }
                Err(err) => {
                    println!("Server konnte Anfrage: '{}' nicht verarbeiten!", err);

                }
            }
        }
    });
    server_thread.join();
    endpoint.shutdown();
}

fn device() {
    let mut front_socket = Socket::new_for_device(Protocol::Rep).unwrap();
    let mut front_endpoint = front_socket.bind("ipc:///tmp/client.ipc").unwrap();
    let mut back_socket = Socket::new_for_device(Protocol::Req).unwrap();
    let mut back_endpoint = back_socket.bind("ipc:///tmp/srv.ipc").unwrap();

    let device_thread = thread::spawn(move || {
        println!("Device ist bereit.");
        Socket::device(&front_socket, &back_socket);
        println!("Device beendet!");

        front_endpoint.shutdown();
        back_endpoint.shutdown();
    });
    // device_thread.join();
}


fn main() {
    for i in 1..101 {
        let name = format!("Client {}", i);
        let client = Client::new(name);
        client.run();
    }

    device();

    server();
}
