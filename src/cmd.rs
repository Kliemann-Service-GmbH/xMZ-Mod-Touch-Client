use clap::{Arg, App, SubCommand};

/// Liest die Befehlszeilen Parameter und formt ein entsprechenden String
///
/// Dieser wird dann an den Server gesendet. Der wiederum parst dann diesen.
pub fn read_command() -> String {
    let mut fullcommand: String = String::new();   // Return Value

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
            (@subcommand get =>
                (about: "Zustand der gegebenen LED abfragen")
                (@arg num: +takes_value +required "LED Nummer(n)")
            )
            (@subcommand clear =>
                (about: "löscht die gegebenen LED")
                (@arg num: +takes_value +required "LED Nummer(n)")
            )
            (@subcommand toggle =>
                (about: "wechselt der Zustand die gegebenen LED")
                (@arg num: +takes_value +required "LED Nummer(n)")
            )
        )
        (@subcommand relais =>
            (about: "Steuert die RELAIS der 'xMZ-Mod-Touch'")
            (@subcommand set =>
                (about: "schaltet das gegebenen Relais")
                (@arg num: +takes_value +required "Relais Nummer(n)")
            )
            (@subcommand get =>
                (about: "Zustand des gegebenen Relais abfragen")
                (@arg num: +takes_value +required "Relais Nummer(n)")
            )
            (@subcommand clear =>
                (about: "löscht das gegebenen Relais")
                (@arg num: +takes_value +required "Relais Nummer(n)")
            )
            (@subcommand toggle =>
                (about: "wechselt der Zustand des gegebenen Relais")
                (@arg num: +takes_value +required "Relais Nummer(n)")
            )
        )
        (@subcommand server =>
            (about: "Konfiguration des Servers Lesen und Ändern")
            (@subcommand set =>
                (about: "Konfiguration des Servers (1. Parameter) durch den Wert (2. Parameter) ersetzen")
                (@arg config_entry: +takes_value +required "Konfigurations Parameter")
                (@arg value: +takes_value +required "zu setzender Wert")
            )
            (@subcommand get =>
                (about: "Konfigurations Parameter abfragen")
                (@arg config_entry: +takes_value +required "Konfigurations Parameter")
            )
        )
        (@subcommand module =>
            (about: "Konfiguration und Information der Sensor Module")
        )
    ).get_matches();

    // LED Command
    if let Some(ref matches) = matches.subcommand_matches("led") {
        let command = "led";
        let mut subcommand = "";
        let mut options = "";

        if let Some(ref matches) = matches.subcommand_matches("set") {
            subcommand = "set";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("get") {
            subcommand = "get";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("clear") {
            subcommand = "clear";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("toggle") {
            subcommand = "toggle";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        fullcommand = format!("{} {} {}", command, subcommand, options);
    }

    // Relais Command
    if let Some(ref matches) = matches.subcommand_matches("relais") {
        let command = "relais";
        let mut subcommand = "";
        let mut options = "";

        if let Some(ref matches) = matches.subcommand_matches("set") {
            subcommand = "set";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("get") {
            subcommand = "get";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("clear") {
            subcommand = "clear";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        if let Some(ref matches) = matches.subcommand_matches("toggle") {
            subcommand = "toggle";
            if let Some(ref matches) = matches.value_of("num") {
                options = matches;
            }
        }
        fullcommand = format!("{} {} {}", command, subcommand, options);
    }

    // Server Command
    if let Some(ref matches) = matches.subcommand_matches("server") {
        let command = "server";
        let mut subcommand = "";
        let mut config_entry = "";
        let mut value = "";

        if let Some(ref matches) = matches.subcommand_matches("set") {
            subcommand = "set";
            if let Some(ref matches) = matches.value_of("config_entry") {
                config_entry = matches;
            }
            if let Some(ref matches) = matches.value_of("value") {
                value = matches;
            }
        }
        fullcommand = format!("{} {} {} {}", command, subcommand, config_entry, value);

        if let Some(ref matches) = matches.subcommand_matches("get") {
            subcommand = "get";
            if let Some(ref matches) = matches.value_of("config_entry") {
                config_entry = matches;
            }
        }
        fullcommand = format!("{} {} {}", command, subcommand, config_entry);
    }

    // Return value senden
    fullcommand

}
