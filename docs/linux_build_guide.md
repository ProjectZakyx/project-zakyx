# Linux Build Guide für Projekt-Ora

## Systemvoraussetzungen

### Unterstützte Linux-Distributionen
- Ubuntu 20.04 LTS oder neuer
- Debian 11 oder neuer
- Fedora 35 oder neuer
- Arch Linux (aktuell)
- openSUSE Leap 15.4 oder neuer

### Erforderliche Software
- Rust 1.70.0 oder neuer
- Git
- Build-Essential Tools
- GTK3 Development Libraries
- WebKit2GTK Development Libraries

## Installation der Abhängigkeiten

### Ubuntu/Debian

```bash
# System aktualisieren
sudo apt update && sudo apt upgrade -y

# Rust installieren (falls nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build-Tools installieren
sudo apt install -y build-essential pkg-config git

# GTK3 und WebKit2GTK Entwicklungsbibliotheken installieren
sudo apt install -y \
    libgtk-3-dev \
    libwebkit2gtk-4.1-dev \
    libjavascriptcoregtk-4.1-dev \
    libsoup-3.0-dev \
    libglib2.0-dev \
    libcairo-gobject2 \
    libgtk-3-0 \
    libwebkit2gtk-4.1-0 \
    libjavascriptcoregtk-4.1-0 \
    libsoup-3.0-0 \
    libgdk-pixbuf2.0-dev \
    libpango1.0-dev \
    libatk1.0-dev \
    libcairo-dev \
    libappindicator3-dev \
    librsvg2-dev

# Optional: Zusätzliche Multimedia-Unterstützung
sudo apt install -y \
    gstreamer1.0-plugins-base \
    gstreamer1.0-plugins-good \
    gstreamer1.0-plugins-bad \
    gstreamer1.0-plugins-ugly \
    gstreamer1.0-libav
```

### Fedora/RHEL/CentOS

```bash
# System aktualisieren
sudo dnf update -y

# Rust installieren (falls nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build-Tools installieren
sudo dnf groupinstall -y "Development Tools"
sudo dnf install -y pkg-config git

# GTK3 und WebKit2GTK Entwicklungsbibliotheken installieren
sudo dnf install -y \
    gtk3-devel \
    webkit2gtk4.1-devel \
    libsoup3-devel \
    glib2-devel \
    cairo-gobject-devel \
    gdk-pixbuf2-devel \
    pango-devel \
    atk-devel \
    cairo-devel \
    libappindicator-gtk3-devel \
    librsvg2-devel

# Optional: Zusätzliche Multimedia-Unterstützung
sudo dnf install -y \
    gstreamer1-plugins-base \
    gstreamer1-plugins-good \
    gstreamer1-plugins-bad-free \
    gstreamer1-plugins-ugly-free
```

### Arch Linux

```bash
# System aktualisieren
sudo pacman -Syu

# Rust installieren (falls nicht vorhanden)
sudo pacman -S rustup
rustup default stable

# Build-Tools installieren
sudo pacman -S base-devel git pkg-config

# GTK3 und WebKit2GTK Bibliotheken installieren
sudo pacman -S \
    gtk3 \
    webkit2gtk-4.1 \
    libsoup3 \
    glib2 \
    cairo \
    gdk-pixbuf2 \
    pango \
    atk \
    libappindicator-gtk3 \
    librsvg

# Optional: Zusätzliche Multimedia-Unterstützung
sudo pacman -S \
    gstreamer \
    gst-plugins-base \
    gst-plugins-good \
    gst-plugins-bad \
    gst-plugins-ugly
```

### openSUSE

```bash
# System aktualisieren
sudo zypper refresh && sudo zypper update -y

# Rust installieren (falls nicht vorhanden)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build-Tools installieren
sudo zypper install -y -t pattern devel_basis
sudo zypper install -y git pkg-config

# GTK3 und WebKit2GTK Entwicklungsbibliotheken installieren
sudo zypper install -y \
    gtk3-devel \
    webkit2gtk4_1-devel \
    libsoup3-devel \
    glib2-devel \
    cairo-devel \
    gdk-pixbuf-devel \
    pango-devel \
    atk-devel \
    libappindicator3-devel \
    librsvg-devel
```

## Projekt klonen und bauen

```bash
# Repository klonen
git clone https://github.com/your-username/projekt-ora.git
cd projekt-ora

# Debug-Build erstellen
cargo build

# Release-Build erstellen (empfohlen für Produktion)
cargo build --release

# Tests ausführen
cargo test

# Anwendung starten (Debug)
cargo run

# Anwendung starten (Release)
./target/release/projekt-ora
```

## Fehlerbehebung

### Häufige Probleme

#### 1. GTK-Bibliotheken nicht gefunden
```bash
# Fehler: "Package gtk+-3.0 was not found"
# Lösung: GTK3-Entwicklungsbibliotheken installieren
sudo apt install libgtk-3-dev  # Ubuntu/Debian
sudo dnf install gtk3-devel    # Fedora
sudo pacman -S gtk3            # Arch
```

#### 2. WebKit2GTK nicht gefunden
```bash
# Fehler: "Package webkit2gtk-4.1 was not found"
# Lösung: WebKit2GTK 4.1-Entwicklungsbibliotheken installieren
sudo apt install libwebkit2gtk-4.1-dev  # Ubuntu/Debian
sudo dnf install webkit2gtk4.1-devel    # Fedora
sudo pacman -S webkit2gtk-4.1           # Arch
```

#### 3. JavaScriptCore GTK nicht gefunden
```bash
# Fehler: "Package javascriptcoregtk-4.1 was not found"
# Lösung: JavaScriptCore GTK 4.1-Entwicklungsbibliotheken installieren
sudo apt install libjavascriptcoregtk-4.1-dev  # Ubuntu/Debian
sudo dnf install webkit2gtk4.1-devel           # Fedora (enthalten)
sudo pacman -S webkit2gtk-4.1                  # Arch (enthalten)
```

#### 4. libsoup-3.0 nicht gefunden
```bash
# Fehler: "Package libsoup-3.0 was not found"
# Lösung: libsoup-3.0-Entwicklungsbibliotheken installieren
sudo apt install libsoup-3.0-dev  # Ubuntu/Debian
sudo dnf install libsoup3-devel   # Fedora
sudo pacman -S libsoup3           # Arch
```

#### 5. pkg-config nicht gefunden
```bash
# Fehler: "Could not find pkg-config"
# Lösung: pkg-config installieren
sudo apt install pkg-config    # Ubuntu/Debian
sudo dnf install pkg-config    # Fedora
sudo pacman -S pkg-config      # Arch
```

#### 6. Linker-Fehler
```bash
# Fehler: "cannot find -lgtk-3"
# Lösung: Umgebungsvariablen setzen
export PKG_CONFIG_PATH=/usr/lib/pkgconfig:/usr/lib/x86_64-linux-gnu/pkgconfig
export LD_LIBRARY_PATH=/usr/lib:/usr/lib/x86_64-linux-gnu
```

### Debugging-Tipps

```bash
# Verfügbare GTK-Versionen prüfen
pkg-config --modversion gtk+-3.0

# WebKit2GTK 4.1-Version prüfen
pkg-config --modversion webkit2gtk-4.1

# JavaScriptCore GTK 4.1-Version prüfen
pkg-config --modversion javascriptcoregtk-4.1

# libsoup-3.0-Version prüfen
pkg-config --modversion libsoup-3.0

# Alle verfügbaren pkg-config-Module anzeigen
pkg-config --list-all | grep -E "(gtk|webkit|soup)"

# Spezifische Bibliothek-Pfade anzeigen
pkg-config --cflags --libs webkit2gtk-4.1
pkg-config --cflags --libs javascriptcoregtk-4.1
pkg-config --cflags --libs libsoup-3.0

# Rust-Toolchain-Information
rustc --version
cargo --version

# Detaillierte Build-Ausgabe
RUST_LOG=debug cargo build --verbose

# Cargo-Dependencies prüfen
cargo tree
```

## Performance-Optimierung

### Release-Build-Optimierungen

Fügen Sie folgende Einstellungen in `Cargo.toml` hinzu:

```toml
[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

### Systemspezifische Optimierungen

```bash
# Für moderne CPUs optimieren
RUSTFLAGS="-C target-cpu=native" cargo build --release

# Für spezifische Architektur kompilieren
cargo build --release --target x86_64-unknown-linux-gnu
```

## Distribution und Packaging

### AppImage erstellen

```bash
# AppImage-Tools installieren
wget https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage
chmod +x appimagetool-x86_64.AppImage

# AppDir-Struktur erstellen
mkdir -p projekt-ora.AppDir/usr/bin
cp target/release/projekt-ora projekt-ora.AppDir/usr/bin/

# Desktop-Datei erstellen
cat > projekt-ora.AppDir/projekt-ora.desktop << EOF
[Desktop Entry]
Type=Application
Name=Projekt Ora
Exec=projekt-ora
Icon=projekt-ora
Categories=Network;WebBrowser;
EOF

# AppImage erstellen
./appimagetool-x86_64.AppImage projekt-ora.AppDir
```

### Flatpak-Package erstellen

```bash
# Flatpak-Builder installieren
sudo apt install flatpak-builder  # Ubuntu/Debian
sudo dnf install flatpak-builder  # Fedora

# Manifest erstellen (siehe docs/flatpak-manifest.json)
flatpak-builder build-dir org.projektora.ProjektOra.json
```

### Snap-Package erstellen

```bash
# Snapcraft installieren
sudo snap install snapcraft --classic

# snapcraft.yaml erstellen (siehe snap/snapcraft.yaml)
snapcraft
```

## Entwicklungsumgebung

### VS Code Setup

```bash
# Rust-Analyzer Extension installieren
code --install-extension rust-lang.rust-analyzer

# Zusätzliche nützliche Extensions
code --install-extension vadimcn.vscode-lldb
code --install-extension serayuzgur.crates
```

### Debugging mit GDB

```bash
# Debug-Symbole aktivieren
cargo build

# Mit GDB debuggen
gdb target/debug/projekt-ora
(gdb) run
(gdb) bt  # Backtrace bei Crash
```

## Continuous Integration

Das Projekt ist für automatische Linux-Builds in GitHub Actions konfiguriert. Siehe `.github/workflows/deploy.yml` für Details.

### Lokale CI-Simulation

```bash
# Act installieren (GitHub Actions lokal ausführen)
curl https://raw.githubusercontent.com/nektos/act/master/install.sh | sudo bash

# Workflow lokal ausführen
act -j build
```

## Weitere Ressourcen

- [GTK3 Dokumentation](https://docs.gtk.org/gtk3/)
- [WebKit2GTK Dokumentation](https://webkitgtk.org/reference/webkit2gtk/stable/)
- [Rust Cross-Compilation Guide](https://rust-lang.github.io/rustup/cross-compilation.html)
- [Linux Desktop Integration](https://specifications.freedesktop.org/)

## Support

Bei Problemen mit dem Linux-Build:

1. Überprüfen Sie die Systemvoraussetzungen
2. Stellen Sie sicher, dass alle Abhängigkeiten installiert sind
3. Prüfen Sie die Fehlermeldungen in der Debugging-Sektion
4. Erstellen Sie ein Issue im GitHub-Repository mit detaillierten Informationen 