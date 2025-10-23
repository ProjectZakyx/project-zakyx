
## Häufige Probleme und Lösungen

### WebView2-Fehler
- Problem: "WebView2Loader.dll nicht gefunden"
- Lösung: WebView2 Runtime neu installieren

### Linker-Fehler
- Problem: "cannot find -ladvapi32" oder "cannot find -lwevtapi"
- Lösung: Windows SDK neu installieren oder Pfad in der Systemumgebung überprüfen

### Build-Fehler mit Windows-Features
- Problem: Fehlende Windows-Features
- Lösung: Visual Studio Installer öffnen und erforderliche Windows SDK Komponenten nachinstallieren

## Optimierung

### Release-Build-Optimierungen
Fügen Sie folgende Einstellungen in `Cargo.toml` hinzu für bessere Performance:
