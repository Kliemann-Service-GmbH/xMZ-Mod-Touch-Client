#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]

//! xMZ-Mod-Touch Client
//!
//! Befehlszeilen Client zur Steuerung der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Client

#[macro_use] extern crate clap;
extern crate nanomsg;

pub mod client;

pub mod cmd;

pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
