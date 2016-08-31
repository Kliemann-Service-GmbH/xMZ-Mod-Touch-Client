#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
#![feature(stmt_expr_attributes)]
#![recursion_limit = "1024"]


//! xMZ-Mod-Touch Client
//!
//! Befehlszeilen Client zur Steuerung der 'xMZ-Mod-Touch'-Platform
//!
//! Git Repository: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Client

#[macro_use] extern crate clap;
#[macro_use]extern crate error_chain;
#[macro_use] extern crate log;
extern crate nanomsg;


/// Implementation des Clients
pub mod client;
/// Befehlsverwaltung des Clienten
pub mod cmd;
/// Fehlerbehandlung mit dem `error-chain` crate https://github.com/brson/error-chain
mod errors;
