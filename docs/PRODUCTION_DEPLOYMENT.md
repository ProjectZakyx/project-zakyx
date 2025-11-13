# 🚀 ZAKYX Browser - PRODUKTIONS-DEPLOYMENT GUIDE

## 📋 **PRODUKTIONSSTART - EINFACHE ANLEITUNG**

### **🎯 Schritt 1: Produktions-Start**

```bash
# Einfacher Start mit Batch-Datei
start_production.bat
```

**ODER manuell:**

```bash
# Release-Build erstellen (falls nicht vorhanden)
cargo build --release

# Produktions-Modus starten
target\release\zakyx-browser.exe
```

---

## 🔧 **PRODUKTIONS-KONFIGURATION**

### **📂 Konfigurationsdateien**

| Datei | Zweck | Speicherort |
|-------|-------|-------------|
| `config.toml` | Haupt-Konfiguration | `%APPDATA%\zakyx-browser\` |
| `zakyx-browser.log` | Log-Dateien | `%APPDATA%\zakyx-browser\logs\` |
| `bookmarks.json` | Gespeicherte Bookmarks | `%APPDATA%\zakyx-browser\` |

### **⚙️ Standard-Konfiguration kopieren**

```bash
# Kopiere Produktions-Konfiguration
copy production_config.toml "%APPDATA%\zakyx-browser\config.toml"
```

---

## 🌐 **NETZWERK-SETUP**

### **🔌 Port-Konfiguration**

| Service | Port | Zweck |
|---------|------|-------|
| Proxy-Server | 3030 | Haupt-Proxy für Web-Requests |
| Fallback-Proxy | 3031 | Backup falls 3030 belegt |
| Health-Check | 3030/health | Server-Status prüfen |

### **🛡️ Firewall-Einstellungen**

```bash
# Windows Firewall: Eingehende Verbindungen für Port 3030 erlauben
netsh advfirewall firewall add rule name="ZAKYX Browser Proxy" dir=in action=allow protocol=TCP localport=3030
```

---

## 📊 **MONITORING & ÜBERWACHUNG**

### **🔍 Health-Check**

```bash
# Server-Status prüfen
curl http://localhost:3030/health

# Erwartete Antwort:
# {"service":"ZAKYX Browser Proxy","status":"ok","timestamp":"..."}
```

### **📝 Log-Monitoring**

```bash
# Live-Logs verfolgen (PowerShell)
Get-Content "$env:APPDATA\zakyx-browser\logs\zakyx-browser.log" -Wait -Tail 10

# Nach Fehlern suchen
Select-String "ERROR|❌" "$env:APPDATA\zakyx-browser\logs\zakyx-browser.log"
```

### **📈 Performance-Metriken**

```bash
# Proxy-Performance testen
Measure-Command { curl http://localhost:3030/health }

# Memory-Usage prüfen (Task Manager)
tasklist | findstr zakyx-browser
```

---

## 🚨 **TROUBLESHOOTING**

### **Problem: Port 3030 bereits belegt**

```bash
# Prüfe welcher Prozess Port 3030 verwendet
netstat -ano | findstr :3030

# Prozess beenden (PID ersetzen)
taskkill /PID <PID> /F

# Oder alternativen Port in config.toml setzen
primary_port = 3031
```

### **Problem: Proxy-Server startet nicht**

```bash
# Debug-Modus aktivieren
set RUST_LOG=zakyx_browser=debug
target\release\zakyx-browser.exe

# Logs prüfen
type "%APPDATA%\zakyx-browser\logs\zakyx-browser.log"
```

### **Problem: Webseiten laden nicht**

```bash
# Proxy-Verbindung testen
curl http://localhost:3030/universal?url=https://google.com

# Browser-Cache leeren
# DNS-Cache leeren: ipconfig /flushdns
```

---

## 🔐 **SICHERHEIT**

### **🛡️ Produktions-Sicherheit**

- ✅ **CORS-Bypass**: Aktiviert für Web-Kompatibilität
- ✅ **CSP-Bypass**: Aktiviert für Sicherheits-Fixes
- ✅ **HTTPS-Upgrade**: Automatisch HTTP→HTTPS
- ✅ **Cookie-Handling**: Sicher implementiert
- ✅ **Content-Size-Limit**: 50MB Maximum

### **🔒 Empfohlene Sicherheitsmaßnahmen**

1. **Nur lokale Verbindungen**: Proxy nur auf `localhost` binden
2. **Firewall**: Externe Zugriffe auf Port 3030 blockieren
3. **Updates**: Regelmäßige Updates des Browsers
4. **Monitoring**: Log-Dateien auf verdächtige Aktivitäten prüfen

---

## 📦 **DEPLOYMENT-OPTIONEN**

### **🏠 Lokale Installation**

```bash
# 1. Repository klonen
git clone <repository-url>
cd project-zakyx-main

# 2. Release-Build erstellen
cargo build --release

# 3. Produktions-Start
start_production.bat
```

### **📋 Portable Installation**

```bash
# 1. Nur benötigte Dateien kopieren
mkdir zakyx-browser-portable
copy target\release\zakyx-browser.exe zakyx-browser-portable\
copy production_config.toml zakyx-browser-portable\config.toml
copy start_production.bat zakyx-browser-portable\

# 2. Portable starten
cd zakyx-browser-portable
start_production.bat
```

### **🏢 Enterprise-Deployment**

```bash
# 1. MSI-Installer erstellen (Windows)
cargo install cargo-wix
cargo wix --nocapture

# 2. Silent Installation
msiexec /i zakyx-browser.msi /quiet

# 3. Zentrale Konfiguration
copy \\server\config\zakyx-browser-config.toml "%APPDATA%\zakyx-browser\config.toml"
```

---

## 📊 **PERFORMANCE-OPTIMIERUNG**

### **⚡ Produktions-Optimierungen**

| Einstellung | Wert | Zweck |
|-------------|------|-------|
| `stack_size_mb` | 2 | Reduzierter Memory-Footprint |
| `request_timeout_secs` | 30 | Balance zwischen Speed/Reliability |
| `max_redirects` | 5 | Verhindert Redirect-Loops |
| `log_level` | "info" | Optimale Performance |

### **🔧 Tuning-Parameter**

```toml
# Für langsame Verbindungen
request_timeout_secs = 60
connect_timeout_secs = 20

# Für schnelle Verbindungen
request_timeout_secs = 15
connect_timeout_secs = 5

# Für hohe Last
stack_size_mb = 4
max_retries = 1
```

---

## 📞 **SUPPORT & WARTUNG**

### **🔄 Updates**

```bash
# Code aktualisieren
git pull origin main
cargo build --release

# Konfiguration sichern
copy "%APPDATA%\zakyx-browser\config.toml" config-backup.toml

# Neuen Browser starten
start_production.bat
```

### **🧹 Wartung**

```bash
# Log-Dateien bereinigen (älter als 7 Tage)
forfiles /p "%APPDATA%\zakyx-browser\logs" /s /m *.log /d -7 /c "cmd /c del @path"

# Konfiguration validieren
# (Automatisch beim Start)
```

### **📊 Monitoring-Dashboard**

```bash
# Einfaches Status-Dashboard
echo "=== ZAKYX Browser STATUS ==="
curl -s http://localhost:3030/health | jq .
echo "Memory: " & tasklist | findstr zakyx-browser
echo "Uptime: " & systeminfo | findstr "Systemstartzeit"
```

---

## 🎯 **ERFOLGSKRITERIEN**

### ✅ **Produktions-Checkliste**

- [ ] Release-Build erfolgreich erstellt
- [ ] Proxy-Server startet auf Port 3030
- [ ] Health-Check antwortet mit "ok"
- [ ] Webseiten laden korrekt
- [ ] Logs werden erstellt
- [ ] Konfiguration ist angepasst
- [ ] Performance ist akzeptabel (< 2s Response Time)
- [ ] Keine kritischen Fehler in Logs

### 📈 **KPIs für Produktions-Betrieb**

| Metrik | Zielwert | Kritisch |
|--------|----------|----------|
| Uptime | > 99% | < 95% |
| Response Time | < 2s | > 10s |
| Memory Usage | < 100MB | > 500MB |
| Error Rate | < 1% | > 5% |

---

**🎉 Herzlichen Glückwunsch! Ihr ZAKYX Browser ist jetzt produktionsbereit!**

**Support**: Siehe `docs/DEBUG_DOCUMENTATION.md` für detaillierte Debugging-Informationen.  
**Version**: 1.0.0  
**Datum**: 2025-06-21 
