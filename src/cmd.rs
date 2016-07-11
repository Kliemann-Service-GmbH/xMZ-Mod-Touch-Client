
#[derive(Debug)]
pub enum Command {
    Invalid,
    Led,
    Relais,
    Module,
    Sensors,
    Execute { command: String, subcommand: String },
}

pub fn read_command() -> Command {
    let matches = clap_app!(xmz_client =>
        (version: "1.0.0")
        (author: "Stefan Müller <s.mueller@it.kls-glt.de>")
        (about: "Komandozeilen Client für die 'xMZ-Mod-Touch'-Platform")
        (@arg command: -c --command +required +takes_value "Komando")
        (@arg subcommand: -s --subcommand +required +takes_value "Subcommand")
    ).get_matches();

    unimplemented!();
}
