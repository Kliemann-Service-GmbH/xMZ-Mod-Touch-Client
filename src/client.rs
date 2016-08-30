use errors::*;
use nanomsg::{Socket, Protocol};
use std::io::{Read, Write};

pub struct Client {
    socket: Socket,
}

impl Client {
    /// Erzeugt eine neue Client Instanz
    ///
    /// Jeder Client wird mit einem Empfangs-Timeout (receive) von 2 Sek. (2000ms) initalisiert.
    /// Wird auch ein Sende-Timeout benötigt muss die Funktion `new_with_send_timeout` anstelle
    /// von `new()` verwendet werden.
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
        // socket.set_send_timeout(1000);
        info!("Aktiviere Socket Receive Timeout von 2 Sekunden");
        socket.set_receive_timeout(2000).unwrap();

        Client {
            socket: socket,
        }
    }

    /// Erzeugt eine neue Client Instanz **mit einem Send Timeout**
    ///
    /// Alternative Client Initialisierung mit einem Sende Timeout (1. Parameter in Millisekunden)
    ///
    /// # Parameters
    ///
    /// `send_timeout`  - Send Timeout in Millisekunden
    ///
    /// # Examples
    ///
    /// ```
    /// use xmz_client::client::Client;
    ///
    /// let client = Client::new_with_send_timeout(2000);
    /// ```
    pub fn new_with_send_timeout(send_timeout: isize) -> Self {
        let mut client = Client::new();
        client.socket.set_send_timeout(send_timeout);

        client
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
    /// assert_eq!(client.execute("server version"), Some(""));
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
}
