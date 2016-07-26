use nanomsg::{Socket, Protocol, Error};
use std::io::{Read, Write};
use std::thread;
use std::time::Duration;

pub struct Client {
    socket: Socket,
}

impl Client {
    /// Erzeugt eine neue Client Instanz
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        let mut socket = Socket::new(Protocol::Req).unwrap();
        let mut endpoint = socket.connect("ipc:///tmp/xmz-server.ipc").unwrap();
        // socket.set_send_timeout(1000);
        socket.set_receive_timeout(2000);

        Client {
            socket: socket,
        }
    }

    /// Führt ein Befehl aus (1. Parameter) und liefert das Ergebnis als Result<String, Error>
    ///
    /// # Params
    ///
    /// `message`   - Zu sendende Nachricht als String oder &str
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let mut client = Client::new();
    /// client.execute("server version");
    /// ```
    pub fn execute<T: AsRef<str>>(&mut self, message: T) -> Result<String, Error> {
        let mut reply = String::new();
        let request = format!("{}", message.as_ref());

        try!(self.socket.write_all(request.as_bytes()));
        try!(self.socket.read_to_string(&mut reply));

        Ok(reply)
    }
}

#[cfg(tests)]
mod test {

        #[test]
        fn execute_parameter_str() {
            let mut client = Client::new();
            client.execute("server version");
        }

        #[test]
        fn execute_parameter_string() {
            let mut client = Client::new();
            client.execute("server version".to_string());
        }
}
