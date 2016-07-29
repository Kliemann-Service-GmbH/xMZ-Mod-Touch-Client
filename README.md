Komandozeilen Client für die 'xMZ-Mod-Touch'-Platform

Mit diesem Command Line Interface (Befehlszeilen Schnittstelle :) können Befehle
an den Server Prozess der 'xMZ-Mod-Touch' gesendet werden.

# Build, Compilation auf der 'xMZ-Mod-Touch'-Hardware
Die folgenden Befehle gehen davon aus das das Meta Git Repository
['xMZ-Mod-Touch-Software'][1] im HOME Verzeichnis ausgecheckt wurde.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cargo build --release
```

# Installation
## Programmdateien installieren

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cp target/release/xmz-client-bin /usr/bin/xmz-client
```

# Tests

Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cargo test
```

# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
* Command Line Parser https://github.com/kbknapp/clap-rs

[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
