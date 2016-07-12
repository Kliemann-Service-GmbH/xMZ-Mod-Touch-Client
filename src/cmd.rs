use clap::{Arg, App, SubCommand};

/// Liest die Befehlszeilen Parameter und formt ein entsprechenden String
///
/// Dieser wird dann an den Server gesendet. Der wiederum parst dann diesen.
pub fn read_command() -> String {
    let mut command = "";
    let mut subcommand = "";
    let mut options: String = String::new();

    // Pull version information out of Cargo.toml
    let version = format!("{}.{}.{}{}",
                    env!("CARGO_PKG_VERSION_MAJOR"),
                    env!("CARGO_PKG_VERSION_MINOR"),
                    env!("CARGO_PKG_VERSION_PATCH"),
                    option_env!("CARGO_PKG_VERSION_PRE").unwrap_or(""));

    let matches = clap_app!(xmz_client =>
        (@setting SubcommandRequiredElseHelp)
        (version: version.as_str())
        (author: env!("CARGO_PKG_AUTHORS"))
        (about: env!("CARGO_PKG_DESCRIPTION"))
        (@subcommand led =>
            (about: "Steuert das LED System der 'xMZ-Mod-Touch'")
            (@subcommand set =>
                (about: "schaltet die gegebenen LED")
                (@arg num: +takes_value +required "LED Nummer(n)")
            )
        )
        (@subcommand relais =>
            (about: "Steuert die RELAIS der 'xMZ-Mod-Touch'")
            (@subcommand set =>
                (about: "schaltet die gegebenen Relais")
                (@arg num: +takes_value +required "Relais Nummer(n)")
            )
        )
        (@subcommand module =>
            (about: "Konfiguration und Information der Sensor Module")
        )
    ).get_matches();

    if let Some(ref matches) = matches.subcommand_matches("led") {
        command = "led";
        if let Some(ref matches) = matches.subcommand_matches("set") {
            subcommand = "set";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches.to_string();
            }
        }
    }

    if let Some(ref matches) = matches.subcommand_matches("relais") {
        command = "relais";
        if let Some(ref matches) = matches.subcommand_matches("set") {
            subcommand = "set";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches.to_string();
            }
        }
    }

    format!("{} {} {}", command, subcommand, options)


}
