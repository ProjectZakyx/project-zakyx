# 🔍 DEBUG DOCUMENTATION - ZAKYX Browser

## 📝 Structured Logging System

### **Logging-Ebenen**

| Ebene | Verwendung | Beispiel |
|-------|------------|----------|
| `error` | Kritische Fehler, die zum Absturz führen | `error!("❌ Critical: Proxy server failed to start")` |
| `warn` | Warnungen, die Aufmerksamkeit erfordern | `warn!("⚠️ Invalid port, using default")` |
| `info` | Wichtige Informationen über den Systemzustand | `info!("✅ ZAKYX Browser window created successfully")` |
| `debug` | Detaillierte Debug-Informationen | `debug!("🔧 Setting up browser state...")` |
| `trace` | Sehr detaillierte Trace-Informationen | `trace!("📊 Metric recorded: {} = {}")` |

### **Log-Ausgabe-Orte**

1. **Console (stdout)**: Farbige, kompakte Ausgabe für Entwicklung
2. **Datei**: JSON-formatierte Logs in `%CONFIG%/zakyx-browser/logs/`
3. **Rotation**: Täglich neue Log-Dateien

### **Konfiguration**

```rust
// Umgebungsvariable für Log-Level
RUST_LOG=zakyx_browser=debug,info

// Oder in config.toml
[logging]
log_level = "info"
enable_file_logging = true
enable_json_logging = true
```

---

## 🎯 **DEBUG-SYMBOLE UND BEDEUTUNG**

### **Status-Symbole**
- `🚀` - Systemstart/Initialisierung
- `✅` - Erfolgreiche Operation
- `❌` - Fehler/Fehlschlag
- `⚠️` - Warnung
- `🔧` - Konfiguration/Setup
- `🌐` - Netzwerk/Web-Operationen
- `📊` - Metriken/Performance
- `🔄` - Retry/Wiederholung
- `🧹` - Cleanup/Bereinigung

### **Komponenten-Symbole**
- `📑` - Tab-Management
- `🔖` - Bookmark-Verwaltung
- `⚙️` - Einstellungen
- `🔌` - Plugin-System
- `🛡️` - Sicherheit
- `📝` - Logging
- `📁` - Dateisystem

---

## 📊 **METRICS SYSTEM**

### **Verfügbare Metriken**

| Metrik | Typ | Beschreibung |
|--------|-----|--------------|
| `proxy_request_duration` | Timer | Zeit für Proxy-Requests |
| `page_load_time` | Timer | Seitenladezeit |
| `memory_usage` | Gauge | Speicherverbrauch |
| `active_tabs` | Counter | Anzahl aktiver Tabs |
| `bookmark_operations` | Counter | Bookmark-Operationen |
| `plugin_events` | Counter | Plugin-Events |

### **Metrics-Verwendung**

```rust
// Timer starten
let _timer = metrics.start_timer("operation_name");

// Wert aufzeichnen
metrics.record_value("memory_usage", 1024.0, None);

// Event zählen
metrics.count("page_loaded", Some(tags));

// Statistiken abrufen
let stats = metrics.get_stats("proxy_request_duration");
```

### **Metrics-Export**

```bash
# JSON-Export der Metriken
curl http://localhost:3030/metrics

# Zusammenfassung in Logs
metrics.log_summary();
```

---

## 🔧 **KONFIGURATIONSSYSTEM**

### **Konfigurationsdatei**: `%CONFIG%/zakyx-browser/config.toml`

```toml
version = "1.0.0"

[proxy]
primary_port = 3030
fallback_port = 3031
request_timeout_secs = 30
connect_timeout_secs = 10
max_redirects = 5
max_retries = 3
stack_size_mb = 2

[logging]
log_level = "info"
enable_file_logging = true
enable_json_logging = true
max_log_files = 7
log_rotation_days = 1

[security]
enable_cors_bypass = true
enable_csp_bypass = true
enable_cookie_handling = true
enable_https_upgrade = true
max_content_size_mb = 50
```

### **Automatische Validierung**

```rust
let mut config = ZAKYXConfig::load();
config.validate_and_fix(); // Korrigiert ungültige Werte
config.save(); // Speichert korrigierte Konfiguration
```

---

## 🧪 **TESTING & DEBUGGING**

### **Unit Tests**
```bash
cargo test --lib                    # Alle Unit Tests
cargo test browser_state            # Spezifische Tests
cargo test --release               # Release-Tests
```

### **Integration Tests**
```bash
cargo test --test integration_tests # End-to-End Tests
cargo test test_proxy_server_health # Spezifischer Test
```

### **Performance Tests**
```bash
cargo test test_proxy_performance   # Performance-Benchmarks
cargo test test_memory_usage        # Memory-Leak Tests
```

### **Debug-Build vs Release-Build**

| Aspekt | Debug | Release |
|--------|-------|---------|
| Optimierung | Keine | Vollständig |
| Debug-Symbole | Ja | Nein |
| Kompilierzeit | Schnell | Langsam |
| Laufzeit | Langsam | Schnell |
| Log-Level | `debug` | `info` |

---

## 🔍 **DEBUGGING-STRATEGIEN**

### **1. Proxy-Server Debug**

```bash
# Health Check
curl http://localhost:3030/health

# Universal Route Test
curl "http://localhost:3030/universal?url=https://example.com"

# Proxy Route Test  
curl "http://localhost:3030/proxy?url=https://example.com"
```

### **2. Log-Analyse**

```bash
# Live-Logs verfolgen
tail -f "%CONFIG%/zakyx-browser/logs/zakyx-browser.log"

# Nach Fehlern suchen
grep "ERROR\|❌" "%CONFIG%/zakyx-browser/logs/zakyx-browser.log"

# Performance-Metriken
grep "📊" "%CONFIG%/zakyx-browser/logs/zakyx-browser.log"
```

### **3. Memory-Profiling**

```rust
// Memory-Tracking aktivieren
RUST_LOG=zakyx_browser=debug cargo run

// Memory-Stats in Logs
grep "memory\|Memory" logs/zakyx-browser.log
```

### **4. Network-Debugging**

```bash
# Proxy-Verkehr überwachen
netstat -an | findstr :3030

# DNS-Auflösung testen
nslookup example.com

# Verbindungstest
telnet localhost 3030
```

---

## 🚨 **HÄUFIGE DEBUG-SZENARIEN**

### **Problem**: Proxy-Server startet nicht
```
❌ Critical: Proxy server failed to start: Address already in use
```
**Lösung**: 
1. Port 3030 prüfen: `netstat -an | findstr :3030`
2. Prozess beenden oder alternativen Port verwenden
3. Firewall-Einstellungen prüfen

### **Problem**: Webseiten laden nicht
```
⚠️ Proxy server not reachable: Connection refused
```
**Lösung**:
1. Health-Check: `curl http://localhost:3030/health`
2. Proxy-Status in Logs prüfen
3. Browser-Cache leeren

### **Problem**: JavaScript-Injection funktioniert nicht
```
🌐 Detected HTML content, injecting security fixes
```
**Lösung**:
1. Content-Type prüfen: Sollte `text/html` sein
2. HTML-Struktur validieren
3. CSP-Header überprüfen

### **Problem**: Performance-Issues
```
📊 proxy_request_duration: avg=5000ms (zu langsam)
```
**Lösung**:
1. Timeout-Werte anpassen
2. Retry-Strategien optimieren
3. Cache-Mechanismen implementieren

---

## 📈 **PERFORMANCE-MONITORING**

### **Key Performance Indicators (KPIs)**

| Metrik | Zielwert | Kritisch |
|--------|----------|----------|
| Proxy Response Time | < 2s | > 10s |
| Memory Usage | < 100MB | > 500MB |
| CPU Usage | < 20% | > 80% |
| Active Connections | < 50 | > 200 |

### **Performance-Dashboard**

```rust
// Metriken-Zusammenfassung alle 60 Sekunden
tokio::spawn(async {
    loop {
        tokio::time::sleep(Duration::from_secs(60)).await;
        if let Some(metrics) = get_metrics() {
            metrics.log_summary();
        }
    }
});
```

---

## 🔐 **SECURITY DEBUGGING**

### **CORS-Debug**
```javascript
// Browser-Console: CORS-Status prüfen
console.log('CORS enabled:', window.fetch !== originalFetch);
```

### **CSP-Debug**
```bash
# CSP-Header prüfen
curl -I "http://localhost:3030/universal?url=https://example.com"
```

### **Cookie-Debug**
```javascript
// Cookie-Handling testen
document.cookie = "test=value";
console.log(document.cookie);
```

---

## 📚 **WEITERFÜHRENDE RESSOURCEN**

- [Tracing Documentation](https://docs.rs/tracing/)
- [Tokio Debugging](https://tokio.rs/tokio/topics/debugging)
- [Rust Performance Book](https://nnethercote.github.io/perf-book/)
- [WebView2 Debugging](https://docs.microsoft.com/en-us/microsoft-edge/webview2/how-to/debug)

---

**Letzte Aktualisierung**: 2025-06-21  
**Version**: 1.0.0 
