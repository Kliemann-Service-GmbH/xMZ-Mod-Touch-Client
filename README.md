Komandozeilen Client für die 'xMZ-Mod-Touch'-Platform

# Build
Die folgenden Befehle gehen davon aus das das Meta Git Repository
['xMZ-Mod-Touch-Software'][1] im HOME Verzeichnis ausgecheckt wurde.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cargo build --release
```

# Installation

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cp target/release/xmz-client /usr/bin/
```

# Tests

Optional können auch die Tests aufgerufen werden.

```bash
cd
cd xMZ-Mod-Touch-Software/xMZ-Mod-Touch-Client
cargo build --release
```

# Links

* https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software


[1]: https://github.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Software
