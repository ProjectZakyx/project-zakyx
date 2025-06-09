# 🌐 Ora Browser - Full Feature Browser

Ein moderner, selbstständiger Webbrowser entwickelt in **Rust** mit **WebView2-Integration**, **Tab-Management**, **Bookmark-System** und **History-Tracking**.

![Ora Browser](https://img.shields.io/badge/Version-1.0.0-blue)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![Windows](https://img.shields.io/badge/Platform-Windows-lightgrey)
![License](https://img.shields.io/badge/License-MIT-green)

## ✨ Features

### 🌐 **WebView2 Integration**
- **Echte Webseiten-Darstellung** mit Chromium Engine
- **JavaScript & CSS Support** vollständig
- **DOM Manipulation** und moderne Web-APIs
- **Hardware-beschleunigte Rendering**

### 📂 **Tab Management System**
- **Multiple Tabs** mit dynamischer Erstellung
- **Tab-Navigation** (Zurück/Vorwärts)
- **Smart Tab-Titel** basierend auf URL
- **Tab-Schließen** mit Sicherheitscheck

### ⭐ **Bookmark System**
- **Default Bookmarks** (Google, GitHub, Wikipedia, GUI)
- **Bookmark hinzufügen/entfernen**
- **Persistente Speicherung**
- **Bookmark-Übersicht** mit Details

### 📚 **History Tracking**
- **Automatisches Tracking** aller besuchten Seiten
- **Chronologische Sortierung**
- **Besuchszähler** pro URL
- **History-Suche** und Anzeige

### 🎨 **HTML GUI**
- **Tailwind CSS** Responsive Design
- **CSS Animations** und Hover-Effekte
- **Smart Navigation Bar** mit URL-Suggestions
- **Real-time Status Bar** mit Counters

### 🚀 **Browser Commands**
- `gui` → HTML GUI laden
- `newtab` → Neuen Tab erstellen
- `bookmarks` → Bookmark-Übersicht
- `history` → History anzeigen
- URLs → Direkte Navigation

## 🛠️ Installation

### Voraussetzungen
- **Windows 10/11** (64-bit)
- **Microsoft Edge WebView2 Runtime** [(Download)](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
- **Rust 1.70+** für Kompilierung [(Download)](https://rustup.rs/)

### 📥 Binary Download
1. **Download** der neuesten Version von [Releases](../../releases)
2. **Entpacken** von `ora-browser-v1.0.0.zip`
3. **Ausführen** von `projekt-ora.exe`

### 🔧 Aus Quellcode kompilieren
```bash
# Repository klonen
git clone https://github.com/user/ora-browser
cd ora-browser

# Abhängigkeiten installieren
cargo build --release

# Browser starten
cargo run --release
```

## 🚀 Verwendung

### **Grundlegende Navigation**
1. **Browser starten** durch Doppelklick auf `projekt-ora.exe`
2. **URL eingeben** in die Adressleiste (z.B. `google.de`)
3. **Enter drücken** zum Navigieren

### **Tab Management**
- `newtab` eingeben → Erstellt neuen Tab
- URL in Tab eingeben → Navigiert im aktiven Tab
- Tabs werden automatisch verwaltet

### **Bookmarks verwenden**
- `bookmarks` eingeben → Zeigt alle Bookmarks an
- Navigation zu URL → Automatisches Hinzufügen möglich
- Default Bookmarks sind vorinstalliert

### **History durchsuchen**
- `history` eingeben → Zeigt Verlauf an
- Automatisches Tracking aller Besuche
- Chronologische Sortierung

### **GUI Features**
- `gui` eingeben → Lädt HTML GUI Interface
- Responsive Design für alle Bildschirmgrößen
- CSS Animations und moderne UI

## 📁 Projektstruktur

```
ora-browser/
├── src/
│   ├── main.rs                    # Hauptanwendung & Message Loop
│   ├── webview_integration.rs     # WebView2 Engine Integration
│   ├── browser_features_simple.rs # Tab/Bookmark/History Manager
│   └── lib.rs                     # Bibliothek (falls benötigt)
├── gui.html                       # Responsive HTML GUI
├── Cargo.toml                     # Rust Dependencies
├── Cargo.lock                     # Dependency Lock File
├── README.md                      # Diese Dokumentation
├── build.rs                       # Build Script (Optional)
└── resources/                     # Icons & Assets
    ├── icon.ico                   # Browser Icon
    └── manifest.xml               # Windows Manifest
```

## 🔧 Technische Details

### **Architektur**
- **Rust Backend** mit Windows APIs
- **WebView2** für Web-Rendering
- **Event-driven** Message Loop
- **Asynchrone Navigation** mit Tokio
- **Memory-safe** Rust Implementation

### **Dependencies**
```toml
[dependencies]
anyhow = "1.0.98"           # Error Handling
tokio = "1.45.1"            # Async Runtime
windows = "0.52"            # Windows APIs
serde_json = "1.0"          # JSON Serialization
webview2-com = "0.19"       # WebView2 Integration
chrono = "0.4"              # Time Handling
```

### **Systemanforderungen**
- **RAM**: Minimum 512 MB, empfohlen 1 GB+
- **Speicher**: 50 MB für Installation
- **CPU**: x64 Prozessor
- **WebView2**: Microsoft Edge WebView2 Runtime

## 🐛 Troubleshooting

### **Browser startet nicht**
1. **WebView2 Runtime** installiert? → [Download Link](https://developer.microsoft.com/en-us/microsoft-edge/webview2/)
2. **Windows Defender** blockiert? → Ausnahme hinzufügen
3. **Antivirus Software** → Ordner zur Whitelist hinzufügen

### **Webseiten laden nicht**
1. **Internetverbindung** prüfen
2. **Firewall-Einstellungen** überprüfen
3. **WebView2 Updates** installieren

### **GUI zeigt nicht an**
1. `gui.html` Datei vorhanden?
2. **Fallback-Modus** automatisch aktiv
3. **Neustart** des Browsers versuchen

### **Performance-Probleme**
1. **Tabs schließen** (weniger als 10)
2. **History leeren** falls sehr groß
3. **Browser neustarten**

## 🎯 Geplante Features (Roadmap)

### **Version 1.1.0**
- [ ] **Keyboard Shortcuts** (Strg+T, Strg+W, etc.)
- [ ] **Download Manager**
- [ ] **Settings Panel**
- [ ] **Theme Support** (Dark/Light Mode)

### **Version 1.2.0**
- [ ] **Extensions Support**
- [ ] **Password Manager**
- [ ] **Private Browsing Mode**
- [ ] **Multi-Window Support**

### **Version 2.0.0**
- [ ] **Linux Support**
- [ ] **Mobile Version**
- [ ] **Synchronization**
- [ ] **Advanced Security Features**

## 🤝 Contributing

Beiträge sind willkommen! Bitte:

1. **Fork** das Repository
2. **Feature Branch** erstellen (`git checkout -b feature/amazing-feature`)
3. **Changes committen** (`git commit -m 'Add amazing feature'`)
4. **Branch pushen** (`git push origin feature/amazing-feature`)
5. **Pull Request** erstellen

### **Development Setup**
```bash
# Repository klonen
git clone https://github.com/user/ora-browser
cd ora-browser

# Dependencies installieren
cargo build

# Tests ausführen
cargo test

# Development Build
cargo run
```

## 📄 License

Dieses Projekt ist unter der **MIT License** lizenziert - siehe [LICENSE](LICENSE) Datei für Details.

## 🙏 Acknowledgments

- **Microsoft** für WebView2 Runtime
- **Rust Community** für excellente Crates
- **Tailwind CSS** für das UI Framework
- **Tokio** für Async Runtime

## 📞 Support

- **Issues**: [GitHub Issues](../../issues)
- **Discussions**: [GitHub Discussions](../../discussions)
- **Email**: support@ora-browser.com (geplant)

---

**Entwickelt mit ❤️ in Rust** 

**© 2024 Ora Browser Project. Alle Rechte vorbehalten.**