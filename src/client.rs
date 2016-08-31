use errors::*;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};

pub struct Client {
    /// Nanomsg Socket
    socket: Socket,
    /// Counter der Kommunikationsfehler
    pub error_communication: u32,
}

impl Client {
    /// Erzeugt eine neue Client Instanz
    ///
    /// ... Werden spezielle
    /// Timeouts benötigt müssen die Funktionen `set_socket_send_timeout()` und
    /// `set_socket_receive_timeout()` verwendet werden.
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        trace!("Erzeuge neuen Client");
        let mut socket = Socket::new(Protocol::Req).unwrap();
        let _endpoint = socket.connect("ipc:///tmp/xmz-server.ipc").unwrap();
        socket.set_send_timeout(100);
        socket.set_receive_timeout(100).unwrap();

        Client {
            socket: socket,
            error_communication: 0,
        }
    }

    /// Setzt den Send Timeout Wert des Sockets
    ///
    /// # Params
    ///
    /// `timeout`   - der zu setzende Timeout
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let mut client = Client::new();
    /// assert_eq!(client.set_socket_send_timeout(1000).unwrap(), ());
    /// ```
    ///
    pub fn set_socket_send_timeout(&mut self, timeout: isize) -> Result<()> {
        let _ = try!(self.socket.set_send_timeout(timeout).chain_err(|| "Socket Send Timeout konnte nicht gesetzt werden"));

        Ok(())
    }

    /// Setzt den Empfangs (receive) Timeout Wert des Sockets
    ///
    /// # Params
    ///
    /// `timeout`   - der zu setzende Timeout
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let mut client = Client::new();
    /// assert_eq!(client.set_socket_receive_timeout(1000).unwrap(), ());
    /// ```
    ///
    pub fn set_socket_receive_timeout(&mut self, timeout: isize) -> Result<()> {
        try!(self.socket.set_receive_timeout(timeout).chain_err(|| "Socket Send Timeout konnte nicht gesetzt werden"));

        Ok(())
    }

    /// Führt ein Befehl aus (1. Parameter) und liefert das Ergebnis als Result<()>
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
    /// //assert_eq!(client.execute("server version"), Some(""));
    /// ```
    pub fn execute<T: AsRef<str>>(&mut self, message: T) -> Result<String> {
        trace!("Führe Befehl aus");
        info!("Befehl: {}", message.as_ref());
        let mut reply = String::new();
        let request = format!("{}", message.as_ref());

        try!(self.socket.write_all(request.as_bytes()).chain_err(|| "Konnte Nachricht nicht schreiben"));
        try!(self.socket.read_to_string(&mut reply).chain_err(|| "Nachricht konnte nicht gelesen werden"));

        Ok(reply)
    }
}


#[cfg(tests)]
mod test {
    fn get_version() -> String {
        format!("{}.{}.{}{}",
                        env!("CARGO_PKG_VERSION_MAJOR"),
                        env!("CARGO_PKG_VERSION_MINOR"),
                        env!("CARGO_PKG_VERSION_PATCH"),
                        option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""))
    }

    #[test]
    fn execute_parameter_str() {
        let version = get_version();
        let mut client = Client::new();

        assert_eq!(version, client.execute("server version"));
    }

    #[test]
    fn execute_parameter_string() {
        let mut client = Client::new();
        client.execute("server version".to_string());
    }

    #[test]
    fn set_socket_send_timeout() {
        let mut client = Client::new();

        assert_eq!(client.set_socket_send_timeout(1000).unwrap(), ());
    }

    #[test]
    fn set_socket_receive_timeout() {
        let mut client = Client::new();

        assert_eq!(client.set_socket_receive_timeout(1000).unwrap(), ());
    }
}
