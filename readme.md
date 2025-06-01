# Projekt-Ora

Ein moderner, plattformübergreifender Browser, entwickelt in Rust.

## Plattformunterstützung

- ✅ **Windows 10/11** - Vollständig unterstützt mit WebView2
- ✅ **Linux** - Unterstützt mit GTK3 und WebKit2GTK
- 🚧 **macOS** - In Entwicklung

## Schnellstart

### Windows

```powershell
# Repository klonen
git clone https://github.com/your-username/projekt-ora.git
cd projekt-ora

# Build und ausführen
cargo run
```

### Linux

```bash
# Repository klonen
git clone https://github.com/your-username/projekt-ora.git
cd projekt-ora

# Automatisches Build-Script verwenden
chmod +x scripts/build-linux.sh
./scripts/build-linux.sh --release

# Oder manuell bauen
cargo build --release
./target/release/projekt-ora
```

Für detaillierte Linux-Installationsanweisungen siehe [Linux Build Guide](docs/linux_build_guide.md).

## Abhängigkeiten

### Windows
- Rust 1.70.0+
- WebView2 Runtime (meist vorinstalliert)
- Visual Studio Build Tools

### Linux
- Rust 1.70.0+
- GTK3 Development Libraries
- WebKit2GTK Development Libraries
- Build-Essential Tools

### Alle Plattformen
- anyhow 1.0.98
- tokio 1.0 (mit "full" Features)

## Projektstruktur

```
projekt-ora/
├── src/
│   ├── main.rs              # Haupteinstiegspunkt
│   ├── lib.rs               # Bibliotheksmodul
│   └── webview.rs           # Cross-Platform WebView
├── crates/
│   ├── core/                # Kernfunktionalität
│   ├── platform/            # Plattformspezifischer Code
│   ├── devtools/            # Entwicklertools
│   └── ui/                  # Benutzeroberfläche
├── docs/
│   ├── linux_build_guide.md # Linux Build-Anleitung
│   └── ...
├── scripts/
│   └── build-linux.sh      # Linux Build-Script
└── .github/workflows/
    └── deploy.yml           # Multi-Platform CI/CD
```

## Build-Optionen

### Debug-Build
```bash
cargo build
```

### Release-Build (empfohlen)
```bash
cargo build --release
```

### Tests ausführen
```bash
cargo test
```

### Plattformspezifische Builds
```bash
# Für Linux
cargo build --target x86_64-unknown-linux-gnu

# Für Windows
cargo build --target x86_64-pc-windows-msvc

# Für macOS
cargo build --target x86_64-apple-darwin
```

## Continuous Integration

Das Projekt unterstützt automatische Builds für alle Plattformen über GitHub Actions:

- **Linux**: Ubuntu Latest mit GTK3/WebKit2GTK
- **Windows**: Windows Latest mit WebView2
- **macOS**: macOS Latest (in Entwicklung)

## Entwicklung

### Lokale Entwicklung

1. Repository klonen
2. Abhängigkeiten installieren (siehe Plattform-spezifische Anleitungen)
3. `cargo run` für schnelle Entwicklung
4. `cargo test` für Tests

### Cross-Platform-Entwicklung

Das Projekt verwendet bedingte Kompilierung für plattformspezifische Features:

```rust
#[cfg(windows)]
// Windows-spezifischer Code

#[cfg(target_os = "linux")]
// Linux-spezifischer Code

#[cfg(target_os = "macos")]
// macOS-spezifischer Code
```

## Dokumentation

- [Linux Build Guide](docs/linux_build_guide.md) - Detaillierte Linux-Installationsanweisungen
- [Installation Guide](docs/installation_guide_for_project_ora.md) - Allgemeine Installationsanleitung
- [Common Issues](docs/common_issues_and_solutions.md) - Häufige Probleme und Lösungen

## Lizenz

[Lizenzinformationen hier einfügen]

## Mitwirken

Wir freuen uns über Beiträge zur Verbesserung von Projekt-Ora. Bitte lesen Sie unsere Contribution Guidelines, bevor Sie einen Pull Request erstellen.

### Plattform-spezifische Beiträge

- **Windows**: WebView2-Integration und Windows-spezifische Features
- **Linux**: GTK/WebKit2GTK-Integration und Linux-Desktop-Integration
- **macOS**: Cocoa/WebKit-Integration (Hilfe erwünscht!)

## Kontakt

[Kontaktinformationen hier einfügen]