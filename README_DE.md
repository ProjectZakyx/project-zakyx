# 🌐 ZAKYX Browser

**Ein moderner, sicherheitsorientierter Web-Browser mit erweiterten Features und plattformübergreifender Unterstützung.**

[🇬🇧 English Version](README.md) | [🇩🇪 Deutsche Version](README_DE.md)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

## ✨ Features

### 🚀 **Kern-Features**
- **Moderne Browser-Engine** mit WebView2 Integration
- **Tab-Management** mit dynamischer Tab-Erstellung und -Verwaltung
- **Bookmark-System** mit persistenter Speicherung
- **Verlaufs-Management** für besuchte Seiten
- **Einstellungs-Management** mit anpassbaren Konfigurationen

### 🔌 **Plugin-System**
- **Dynamisches Plugin-Loading** zur Laufzeit
- **Berechtigungssystem** für sichere Plugin-Ausführung
- **Plugin-Manager** mit Enable/Disable-Funktionalität
- **API-Versionierung** für Plugin-Kompatibilität
- **Manifest-basierte Konfiguration**

### 🛡️ **Sicherheits-Features**
- **Smart Proxy System** für problematische Websites
- **URL-Normalisierung** und -Validierung
- **Ethische Safeguards** mit Rate-Limiting
- **Header-Stripping** für iframe-Kompatibilität
- **Domain-spezifische Fallbacks**

### 🏗️ **Architektur**
- **Modulare Struktur** mit klarer Trennung der Verantwortlichkeiten
- **Async/Await** für performante I/O-Operationen
- **Thread-sichere State-Verwaltung** mit RwLock
- **Umfassende Test-Abdeckung** (Unit + Integration Tests)
- **Plugin-API** für Erweiterbarkeit

## 🚀 Installation

### Voraussetzungen
- **Rust** (1.70+)
- **Node.js** (16+)
- **WebView2** (Windows)
- **Tauri CLI**

```bash
# Tauri CLI installieren
cargo install tauri-cli

# Repository klonen
git clone https://github.com/ProjectZakyx/project-zakyx.git
cd project-zakyx-main

# Dependencies installieren
cargo build --release

# Frontend bauen
cd frontend
npm install
node build.js
cd ..
```

### Build
```bash
# Development Build
cargo build

# Production Build
cargo build --release

# Frontend Build
cd frontend && node build.js

# Tests ausführen
cargo test
npm test
```

## 📖 Verwendung

### Browser starten
```bash
# Development
cargo run

# Production
./target/release/zakyx-browser

# Mit PowerShell-Launcher (Windows)
.\launch_zakyx_browser_with_gui.ps1
```

### Plugin entwickeln
```json
{
  "name": "Mein Plugin",
  "version": "1.0.0",
  "description": "Ein Beispiel-Plugin",
  "author": "Dein Name",
  "main_script": "main.js",
  "permissions": ["network", "storage"],
  "api_version": "1.0",
  "enabled": true
}
```

### API verwenden
```javascript
// Tab erstellen
await invoke('create_new_tab', { url: 'https://example.com' });

// Bookmark hinzufügen
await invoke('add_bookmark', { 
  title: 'Beispiel', 
  url: 'https://example.com' 
});

// Plugin laden
await invoke('load_plugin', { plugin_id: 'mein-plugin' });
```

## 🏗️ Architektur

### Module-Struktur
```
src/
├── main.rs                    # Haupt-Anwendung (87 Zeilen)
├── browser_state.rs           # Browser State & Data Structures
├── tauri_commands.rs          # Alle Tauri Commands
├── plugin_manager.rs          # Plugin-System
├── url_utils.rs              # URL-Hilfsfunktionen
├── proxy_server.rs           # Smart Proxy System
├── smart_proxy.rs            # Erweiterte Proxy-Logik
├── browser_features.rs       # Browser-Features
├── ethical_safeguards.rs     # Sicherheits-Features
└── internal_webview2_navigation.rs  # WebView2 Integration
```

### Plugin-System
```
extensions/
├── antibot-plugin/           # Anti-Bot Plugin (Beispiel)
│   ├── plugin.json          # Plugin Manifest
│   ├── antibot-plugin.js    # Haupt-Script
│   └── ui/                  # UI-Komponenten
└── your-plugin/             # Dein Plugin
    ├── plugin.json
    └── main.js
```

## 🧪 Tests

### Test-Abdeckung
- **17 Unit Tests** (100% Pass-Rate)
- **4 Integration Tests** (100% Pass-Rate)
- **Umfassende Modul-Tests**

```bash
# Alle Tests ausführen
cargo test

# Nur Unit Tests
cargo test --lib

# Nur Integration Tests
cargo test --test integration_tests
```

## 🔌 Plugin-Entwicklung

### Plugin erstellen
1. **Verzeichnis erstellen**: `extensions/mein-plugin/`
2. **Manifest erstellen**: `plugin.json`
3. **Script entwickeln**: `main.js`
4. **Plugin installieren**: Über Plugin-Manager

### Berechtigungen
- `network` - Netzwerk-Zugriff
- `storage` - Lokale Speicherung
- `tabs` - Tab-Management
- `bookmarks` - Bookmark-Zugriff
- `history` - Verlaufs-Zugriff
- `settings` - Einstellungs-Zugriff

## 🛠️ Entwicklung

### Beitragen
1. **Fork** das Repository
2. **Feature Branch** erstellen
3. **Tests** hinzufügen
4. **Pull Request** erstellen

## 📊 Performance

### Metriken
- **Startup Zeit**: < 2 Sekunden
- **Memory Usage**: < 100MB (Base)
- **Plugin Loading**: < 500ms
- **Tab Creation**: < 100ms

## 🔒 Sicherheit

### Sicherheits-Features
- **Plugin-Sandboxing** mit Berechtigungssystem
- **URL-Validierung** und -Normalisierung
- **Rate-Limiting** für Netzwerk-Requests
- **Header-Stripping** für sichere iframe-Einbettung
- **Ethische Safeguards** für verantwortliches Browsing

## 🤝 Community

### Support
- **GitHub Issues** für Bug Reports
- **Discussions** für Feature Requests
- **Wiki** für Dokumentation

### Lizenz
MIT License - siehe [LICENSE](LICENSE) für Details.

---

**Entwickelt mit ❤️ und Rust**

*ZAKYX Browser - Browsing neu definiert*
