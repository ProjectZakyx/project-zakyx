# 📚 Ora Browser API Dokumentation

## 🎯 Übersicht

Die Ora Browser API bietet umfassende Funktionalitäten für Tab-Management, Bookmarks, Plugin-System und mehr. Alle API-Calls sind asynchron und verwenden das Tauri Command-System.

## 📑 Tab Management

### `create_new_tab`
Erstellt einen neuen Tab.

```javascript
await invoke('create_new_tab', { 
  url: 'https://example.com' // Optional
});
```

**Parameter:**
- `url` (optional): URL für den neuen Tab

**Rückgabe:**
```typescript
interface Tab {
  id: number;
  title: string;
  url: string;
  is_active: boolean;
}
```

### `close_tab`
Schließt einen Tab.

```javascript
await invoke('close_tab', { 
  tab_id: '123' 
});
```

**Parameter:**
- `tab_id`: ID des zu schließenden Tabs

### `get_tabs`
Ruft alle Tabs ab.

```javascript
const tabs = await invoke('get_tabs');
```

**Rückgabe:** Array von `Tab` Objekten

### `update_tab_title`
Aktualisiert den Titel eines Tabs.

```javascript
await invoke('update_tab_title', {
  tab_id: '123',
  title: 'Neuer Titel'
});
```

## 🌐 Navigation

### `navigate_to`
Navigiert zu einer URL.

```javascript
await invoke('navigate_to', {
  tab_id: '123',
  url: 'https://example.com'
});
```

### `navigate_internally`
Interne Navigation mit WebView2.

```javascript
await invoke('navigate_internally', {
  tab_id: '123',
  url: 'https://example.com'
});
```

### `check_internal_navigation`
Prüft ob interne Navigation möglich ist.

```javascript
const canNavigate = await invoke('check_internal_navigation', {
  url: 'https://example.com'
});
```

## 🔖 Bookmark Management

### `add_bookmark`
Fügt ein Bookmark hinzu.

```javascript
await invoke('add_bookmark', {
  title: 'Beispiel Seite',
  url: 'https://example.com'
});
```

**Rückgabe:**
```typescript
interface Bookmark {
  id: string;
  title: string;
  url: string;
}
```

### `get_bookmarks`
Ruft alle Bookmarks ab.

```javascript
const bookmarks = await invoke('get_bookmarks');
```

### `remove_bookmark`
Entfernt ein Bookmark.

```javascript
await invoke('remove_bookmark', {
  bookmark_id: 'bookmark-123'
});
```

## ⚙️ Einstellungen

### `get_settings`
Ruft Browser-Einstellungen ab.

```javascript
const settings = await invoke('get_settings');
```

**Rückgabe:**
```typescript
interface BrowserSettings {
  homepage: string;
  search_engine: string;
  enable_javascript: boolean;
  enable_cookies: boolean;
}
```

### `update_settings`
Aktualisiert Browser-Einstellungen.

```javascript
await invoke('update_settings', {
  homepage: 'https://google.com',
  search_engine: 'https://google.com/search?q=',
  enable_javascript: true,
  enable_cookies: true
});
```

## 📚 Verlauf

### `get_history`
Ruft den Browser-Verlauf ab.

```javascript
const history = await invoke('get_history');
```

**Rückgabe:** Array von URL-Strings

## 🔌 Plugin Management

### `get_all_plugins`
Ruft alle verfügbaren Plugins ab.

```javascript
const plugins = await invoke('get_all_plugins');
```

**Rückgabe:**
```typescript
interface PluginInfo {
  id: string;
  manifest: PluginManifest;
  path: string;
  loaded: boolean;
  error?: string;
}

interface PluginManifest {
  name: string;
  version: string;
  description: string;
  author: string;
  main_script: string;
  permissions: string[];
  api_version: string;
  enabled: boolean;
}
```

### `get_loaded_plugins`
Ruft alle geladenen Plugins ab.

```javascript
const loadedPlugins = await invoke('get_loaded_plugins');
```

### `enable_plugin`
Aktiviert ein Plugin.

```javascript
await invoke('enable_plugin', {
  plugin_id: 'mein-plugin'
});
```

### `disable_plugin`
Deaktiviert ein Plugin.

```javascript
await invoke('disable_plugin', {
  plugin_id: 'mein-plugin'
});
```

### `load_plugin`
Lädt ein Plugin.

```javascript
await invoke('load_plugin', {
  plugin_id: 'mein-plugin'
});
```

### `unload_plugin`
Entlädt ein Plugin.

```javascript
await invoke('unload_plugin', {
  plugin_id: 'mein-plugin'
});
```

## 🔧 Utility Functions

### `get_proxy_url`
Generiert eine Proxy-URL.

```javascript
const proxyUrl = await invoke('get_proxy_url', {
  url: 'https://example.com'
});
```

### `open_external_url`
Öffnet eine URL im Standard-Browser.

```javascript
await invoke('open_external_url', {
  url: 'https://example.com'
});
```

### `navigate_and_get_content`
Navigiert zu einer URL und ruft den Inhalt ab.

```javascript
const content = await invoke('navigate_and_get_content', {
  url: 'https://example.com'
});
```

## 🔍 WebView Configuration

### `get_webview_config`
Ruft WebView-Konfiguration für eine URL ab.

```javascript
const config = await invoke('get_webview_config', {
  url: 'https://example.com'
});
```

**Rückgabe:**
```typescript
interface WebViewConfig {
  enable_javascript: boolean;
  enable_cookies: boolean;
  enable_local_storage: boolean;
  user_agent: string;
  disable_web_security: boolean;
  allow_running_insecure_content: boolean;
}
```

## 🚨 Fehlerbehandlung

Alle API-Calls können Fehler zurückgeben. Verwende try-catch für Fehlerbehandlung:

```javascript
try {
  const tab = await invoke('create_new_tab', { 
    url: 'https://example.com' 
  });
  console.log('Tab erstellt:', tab);
} catch (error) {
  console.error('Fehler beim Erstellen des Tabs:', error);
}
```

## 📝 Events

Der Browser emittiert verschiedene Events:

### `webview_navigate`
Wird ausgelöst wenn Navigation startet.

```javascript
listen('webview_navigate', (event) => {
  console.log('Navigation zu:', event.payload);
});
```

### `webview_loaded`
Wird ausgelöst wenn Seite geladen ist.

```javascript
listen('webview_loaded', (event) => {
  console.log('Seite geladen:', event.payload);
});
```

### `internal_navigation_success`
Wird bei erfolgreicher interner Navigation ausgelöst.

```javascript
listen('internal_navigation_success', (event) => {
  console.log('Interne Navigation erfolgreich:', event.payload);
});
```

## 🔌 Plugin API

### Plugin-Struktur
```javascript
class MeinPlugin {
  constructor() {
    this.name = 'Mein Plugin';
    this.version = '1.0.0';
    this.permissions = ['network', 'storage'];
  }
  
  async initialize() {
    console.log('Plugin initialisiert');
  }
  
  async onNavigate(url) {
    console.log('Navigation zu:', url);
  }
  
  async onTabCreate(tab) {
    console.log('Tab erstellt:', tab);
  }
  
  async cleanup() {
    console.log('Plugin wird entladen');
  }
}

// Plugin registrieren
window.registerPlugin(new MeinPlugin());
```

### Plugin-Berechtigungen
- `network` - HTTP-Requests senden
- `storage` - Lokale Daten speichern
- `tabs` - Tab-Management
- `bookmarks` - Bookmark-Zugriff
- `history` - Verlaufs-Zugriff
- `settings` - Einstellungs-Zugriff

### Plugin-Manifest
```json
{
  "name": "Mein Plugin",
  "version": "1.0.0",
  "description": "Ein Beispiel-Plugin für Ora Browser",
  "author": "Dein Name",
  "main_script": "main.js",
  "permissions": ["network", "storage"],
  "api_version": "1.0",
  "enabled": true
}
```

## 📊 Performance-Tipps

1. **Batch-Operations**: Verwende mehrere API-Calls parallel
2. **Caching**: Cache häufig verwendete Daten
3. **Lazy Loading**: Lade Plugins nur bei Bedarf
4. **Event Debouncing**: Verhindere zu häufige API-Calls

## 🔒 Sicherheits-Hinweise

1. **Input-Validierung**: Validiere alle Eingaben
2. **Permission-Checks**: Prüfe Plugin-Berechtigungen
3. **URL-Sanitization**: Verwende sichere URLs
4. **Rate-Limiting**: Verhindere API-Spam

## 📚 Beispiele

### Vollständiges Tab-Management
```javascript
// Neuen Tab erstellen
const tab = await invoke('create_new_tab', { 
  url: 'https://github.com' 
});

// Zu URL navigieren
await invoke('navigate_to', {
  tab_id: tab.id.toString(),
  url: 'https://github.com/trending'
});

// Tab-Titel aktualisieren
await invoke('update_tab_title', {
  tab_id: tab.id.toString(),
  title: 'GitHub Trending'
});

// Bookmark hinzufügen
await invoke('add_bookmark', {
  title: 'GitHub Trending',
  url: 'https://github.com/trending'
});
```

### Plugin-Entwicklung
```javascript
// plugin.json
{
  "name": "URL Logger",
  "version": "1.0.0",
  "description": "Loggt alle besuchten URLs",
  "author": "Entwickler",
  "main_script": "logger.js",
  "permissions": ["storage", "tabs"],
  "api_version": "1.0",
  "enabled": true
}

// logger.js
class URLLogger {
  constructor() {
    this.name = 'URL Logger';
    this.urls = [];
  }
  
  async initialize() {
    console.log('URL Logger gestartet');
    this.urls = JSON.parse(localStorage.getItem('logged_urls') || '[]');
  }
  
  async onNavigate(url) {
    this.urls.push({
      url: url,
      timestamp: new Date().toISOString()
    });
    localStorage.setItem('logged_urls', JSON.stringify(this.urls));
    console.log('URL geloggt:', url);
  }
  
  getLoggedUrls() {
    return this.urls;
  }
}

window.registerPlugin(new URLLogger());
```

## 🔌 Plugin API

### Plugin-Verwaltung
```rust
// Holt alle verfügbaren Plugins
get_all_plugins() -> Result<Vec<PluginInfo>, String>

// Aktiviert ein Plugin
enable_plugin(plugin_id: String) -> Result<(), String>

// Deaktiviert ein Plugin
disable_plugin(plugin_id: String) -> Result<(), String>

// Lädt ein Plugin
load_plugin(plugin_path: String) -> Result<(), String>

// Entlädt ein Plugin
unload_plugin(plugin_id: String) -> Result<(), String>

// Holt Plugin-Informationen
get_plugin_info(plugin_id: String) -> Result<PluginInfo, String>
```

### Plugin-Konfiguration
```rust
// Holt Plugin-Konfiguration
get_plugin_config(plugin_id: String) -> Result<serde_json::Value, String>

// Setzt Plugin-Konfiguration
set_plugin_config(plugin_id: String, config: serde_json::Value) -> Result<(), String>

// Validiert Plugin-Berechtigungen
validate_plugin_permissions(plugin_id: String, permissions: Vec<String>) -> Result<bool, String>
```

## Datenstrukturen

### BrowserState
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrowserState {
    pub current_url: String,
    pub can_go_back: bool,
    pub can_go_forward: bool,
    pub is_loading: bool,
    pub page_title: String,
    pub history: Vec<String>,
    pub bookmarks: Vec<Bookmark>,
    pub settings: BrowserSettings,
}
```

### PluginInfo
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub enabled: bool,
    pub permissions: Vec<String>,
    pub config: serde_json::Value,
}
```

### PluginManifest
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginManifest {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub author: String,
    pub main: String,
    pub permissions: Vec<String>,
    pub config_schema: Option<serde_json::Value>,
}
```

## Plugin-Entwicklung

### Plugin-Struktur
```
extensions/mein-plugin/
├── plugin.json          # Manifest-Datei
├── main.js             # Haupt-Plugin-Code
├── ui/                 # UI-Dateien (optional)
│   ├── popup.html
│   ├── popup.css
│   └── popup.js
└── assets/             # Ressourcen (optional)
    └── icon.png
```

### Manifest-Datei (plugin.json)
```json
{
  "id": "mein-plugin",
  "name": "Mein Plugin",
  "version": "1.0.0",
  "description": "Beschreibung meines Plugins",
  "author": "Mein Name",
  "main": "main.js",
  "permissions": [
    "tabs",
    "storage",
    "notifications"
  ],
  "config_schema": {
    "type": "object",
    "properties": {
      "enabled": {
        "type": "boolean",
        "default": true
      },
      "api_key": {
        "type": "string",
        "description": "API-Schlüssel für externe Dienste"
      }
    }
  }
}
```

### Plugin-Hauptdatei (main.js)
```javascript
class MeinPlugin {
    constructor() {
        this.name = 'Mein Plugin';
        this.version = '1.0.0';
        this.config = {};
    }

    // Wird beim Laden des Plugins aufgerufen
    async initialize(config) {
        this.config = config;
        console.log(`${this.name} initialisiert`);
        
        // Event-Listener registrieren
        this.registerEventListeners();
    }

    // Wird beim Entladen des Plugins aufgerufen
    async cleanup() {
        console.log(`${this.name} wird entladen`);
        // Aufräumarbeiten hier
    }

    // Event-Listener registrieren
    registerEventListeners() {
        // Navigation-Events
        window.addEventListener('ora-navigation', (event) => {
            this.onNavigation(event.detail);
        });

        // Page-Load-Events
        window.addEventListener('ora-page-loaded', (event) => {
            this.onPageLoaded(event.detail);
        });
    }

    // Navigation-Handler
    onNavigation(data) {
        console.log('Navigation zu:', data.url);
        // Plugin-spezifische Logik hier
    }

    // Page-Load-Handler
    onPageLoaded(data) {
        console.log('Seite geladen:', data.url);
        // Plugin-spezifische Logik hier
    }

    // Konfiguration aktualisieren
    updateConfig(newConfig) {
        this.config = { ...this.config, ...newConfig };
    }

    // Plugin-Status abrufen
    getStatus() {
        return {
            name: this.name,
            version: this.version,
            active: true,
            config: this.config
        };
    }
}

// Plugin-Instanz exportieren
window.MeinPlugin = new MeinPlugin();
```

### UI-Integration (popup.html)
```html
<!DOCTYPE html>
<html>
<head>
    <meta charset="UTF-8">
    <title>Mein Plugin</title>
    <link rel="stylesheet" href="popup.css">
</head>
<body>
    <div class="plugin-popup">
        <h2>Mein Plugin</h2>
        <div class="controls">
            <label>
                <input type="checkbox" id="enabled"> Aktiviert
            </label>
            <input type="text" id="api-key" placeholder="API-Schlüssel">
            <button id="save-config">Speichern</button>
        </div>
        <div class="status" id="status">
            Status: Bereit
        </div>
    </div>
    <script src="popup.js"></script>
</body>
</html>
```

## Berechtigungen

### Verfügbare Berechtigungen
- `tabs`: Zugriff auf Tab-Verwaltung
- `storage`: Zugriff auf lokalen Speicher
- `notifications`: Benachrichtigungen anzeigen
- `network`: Netzwerk-Anfragen senden
- `dom`: DOM-Manipulation
- `cookies`: Cookie-Zugriff
- `history`: Browser-Verlauf
- `bookmarks`: Lesezeichen-Verwaltung

### Berechtigungs-Validierung
```rust
// Beispiel: Überprüfung ob Plugin Berechtigung hat
if plugin_manager.has_permission(&plugin_id, "network") {
    // Netzwerk-Operation erlauben
} else {
    // Zugriff verweigern
}
```

## Events

### Browser-Events
```javascript
// Navigation-Event
window.dispatchEvent(new CustomEvent('ora-navigation', {
    detail: { url: 'https://example.com', timestamp: Date.now() }
}));

// Page-Load-Event
window.dispatchEvent(new CustomEvent('ora-page-loaded', {
    detail: { url: 'https://example.com', title: 'Beispiel', timestamp: Date.now() }
}));

// Plugin-Event
window.dispatchEvent(new CustomEvent('ora-plugin-message', {
    detail: { plugin_id: 'mein-plugin', message: 'Hallo Welt!' }
}));
```

### Event-Listener
```javascript
// In Plugin-Code
window.addEventListener('ora-navigation', (event) => {
    console.log('Navigation:', event.detail);
});

window.addEventListener('ora-page-loaded', (event) => {
    console.log('Seite geladen:', event.detail);
});
```

## Fehlerbehandlung

### Rust-Seite
```rust
// Fehler-Typen
#[derive(Debug, thiserror::Error)]
pub enum PluginError {
    #[error("Plugin nicht gefunden: {0}")]
    NotFound(String),
    
    #[error("Plugin bereits geladen: {0}")]
    AlreadyLoaded(String),
    
    #[error("Ungültige Berechtigung: {0}")]
    InvalidPermission(String),
    
    #[error("Konfigurationsfehler: {0}")]
    ConfigError(String),
}

// Fehler-Handler
fn handle_plugin_error(error: PluginError) -> String {
    match error {
        PluginError::NotFound(id) => format!("Plugin '{}' wurde nicht gefunden", id),
        PluginError::AlreadyLoaded(id) => format!("Plugin '{}' ist bereits geladen", id),
        PluginError::InvalidPermission(perm) => format!("Ungültige Berechtigung: '{}'", perm),
        PluginError::ConfigError(msg) => format!("Konfigurationsfehler: {}", msg),
    }
}
```

### JavaScript-Seite
```javascript
// Fehler-Handler in Plugin
class PluginErrorHandler {
    static handle(error, context = '') {
        console.error(`Plugin-Fehler ${context}:`, error);
        
        // Benutzer benachrichtigen
        window.dispatchEvent(new CustomEvent('ora-plugin-error', {
            detail: {
                error: error.message,
                context: context,
                timestamp: Date.now()
            }
        }));
    }
}

// Verwendung
try {
    await this.performOperation();
} catch (error) {
    PluginErrorHandler.handle(error, 'beim Ausführen der Operation');
}
```

## Beispiele

### Einfaches Plugin
```javascript
// Einfaches "Hallo Welt" Plugin
class HelloWorldPlugin {
    async initialize(config) {
        console.log('Hallo Welt Plugin gestartet!');
        
        // Button zur Seite hinzufügen
        const button = document.createElement('button');
        button.textContent = 'Hallo sagen';
        button.onclick = () => alert('Hallo von meinem Plugin!');
        document.body.appendChild(button);
    }

    async cleanup() {
        // Button entfernen
        const buttons = document.querySelectorAll('button');
        buttons.forEach(btn => {
            if (btn.textContent === 'Hallo sagen') {
                btn.remove();
            }
        });
    }
}

window.HelloWorldPlugin = new HelloWorldPlugin();
```

### Erweiterte Plugin-Funktionen
```javascript
// Plugin mit Speicher und Netzwerk-Funktionen
class AdvancedPlugin {
    constructor() {
        this.storage = new Map();
        this.apiEndpoint = 'https://api.example.com';
    }

    async initialize(config) {
        this.config = config;
        await this.loadStoredData();
        this.setupUI();
    }

    async loadStoredData() {
        // Daten aus lokalem Speicher laden
        const stored = localStorage.getItem('advanced-plugin-data');
        if (stored) {
            this.storage = new Map(JSON.parse(stored));
        }
    }

    async saveData() {
        // Daten in lokalem Speicher speichern
        const data = Array.from(this.storage.entries());
        localStorage.setItem('advanced-plugin-data', JSON.stringify(data));
    }

    async fetchExternalData(url) {
        try {
            const response = await fetch(`${this.apiEndpoint}/data?url=${encodeURIComponent(url)}`);
            return await response.json();
        } catch (error) {
            console.error('Fehler beim Abrufen externer Daten:', error);
            return null;
        }
    }

    setupUI() {
        // UI-Elemente erstellen und Event-Listener hinzufügen
        const panel = document.createElement('div');
        panel.className = 'advanced-plugin-panel';
        panel.innerHTML = `
            <h3>Erweiterte Plugin-Funktionen</h3>
            <button id="fetch-data">Daten abrufen</button>
            <button id="save-data">Daten speichern</button>
            <div id="data-display"></div>
        `;
        
        document.body.appendChild(panel);
        
        // Event-Listener
        document.getElementById('fetch-data').onclick = () => this.handleFetchData();
        document.getElementById('save-data').onclick = () => this.handleSaveData();
    }

    async handleFetchData() {
        const currentUrl = window.location.href;
        const data = await this.fetchExternalData(currentUrl);
        
        if (data) {
            this.storage.set(currentUrl, data);
            this.displayData(data);
        }
    }

    async handleSaveData() {
        await this.saveData();
        alert('Daten gespeichert!');
    }

    displayData(data) {
        const display = document.getElementById('data-display');
        display.innerHTML = `<pre>${JSON.stringify(data, null, 2)}</pre>`;
    }
}

window.AdvancedPlugin = new AdvancedPlugin();
```

## Debugging

### Plugin-Debugging
```javascript
// Debug-Modus aktivieren
const DEBUG = true;

function debugLog(message, data = null) {
    if (DEBUG) {
        console.log(`[Plugin Debug] ${message}`, data);
    }
}

// Verwendung
debugLog('Plugin initialisiert', this.config);
debugLog('Event empfangen', event.detail);
```

### Rust-Debugging
```rust
// Debug-Makros verwenden
use log::{debug, info, warn, error};

// In Plugin-Manager
debug!("Plugin wird geladen: {}", plugin_id);
info!("Plugin erfolgreich aktiviert: {}", plugin_id);
warn!("Plugin-Berechtigung fehlt: {}", permission);
error!("Plugin-Fehler: {}", error);
```

## Performance-Optimierung

### Plugin-Performance
```javascript
// Lazy Loading für schwere Operationen
class OptimizedPlugin {
    constructor() {
        this.heavyModule = null;
    }

    async loadHeavyModule() {
        if (!this.heavyModule) {
            // Modul nur bei Bedarf laden
            this.heavyModule = await import('./heavy-module.js');
        }
        return this.heavyModule;
    }

    // Debouncing für häufige Events
    debounce(func, wait) {
        let timeout;
        return function executedFunction(...args) {
            const later = () => {
                clearTimeout(timeout);
                func(...args);
            };
            clearTimeout(timeout);
            timeout = setTimeout(later, wait);
        };
    }
}
```

### Speicher-Management
```rust
// Rust-seitige Optimierungen
impl PluginManager {
    // Plugin-Cache mit LRU-Eviction
    fn cleanup_unused_plugins(&mut self) {
        let now = std::time::Instant::now();
        self.plugins.retain(|_, plugin| {
            now.duration_since(plugin.last_used) < std::time::Duration::from_secs(3600)
        });
    }

    // Ressourcen-Limits
    fn check_resource_limits(&self, plugin_id: &str) -> Result<(), PluginError> {
        if self.get_plugin_memory_usage(plugin_id) > MAX_PLUGIN_MEMORY {
            return Err(PluginError::ResourceLimit("Speicher-Limit überschritten".to_string()));
        }
        Ok(())
    }
}
```

## Sicherheit

### Plugin-Sandboxing
```rust
// Sicherheits-Validierung
impl PluginManager {
    fn validate_plugin_security(&self, manifest: &PluginManifest) -> Result<(), PluginError> {
        // Gefährliche Berechtigungen prüfen
        let dangerous_permissions = ["system", "file_system", "network_admin"];
        for perm in &manifest.permissions {
            if dangerous_permissions.contains(&perm.as_str()) {
                return Err(PluginError::SecurityViolation(format!(
                    "Gefährliche Berechtigung: {}", perm
                )));
            }
        }
        
        // Code-Signatur prüfen (falls implementiert)
        self.verify_plugin_signature(manifest)?;
        
        Ok(())
    }
}
```

### Content Security Policy
```javascript
// CSP für Plugin-UI
const cspMeta = document.createElement('meta');
cspMeta.httpEquiv = 'Content-Security-Policy';
cspMeta.content = "default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline';";
document.head.appendChild(cspMeta);
```

## Migration und Updates

### Plugin-Updates
```rust
// Update-System
impl PluginManager {
    async fn update_plugin(&mut self, plugin_id: &str, new_version: &str) -> Result<(), PluginError> {
        // Altes Plugin sichern
        self.backup_plugin(plugin_id)?;
        
        // Neues Plugin laden
        let new_plugin = self.load_plugin_version(plugin_id, new_version)?;
        
        // Migration durchführen
        self.migrate_plugin_data(plugin_id, &new_plugin)?;
        
        // Plugin ersetzen
        self.replace_plugin(plugin_id, new_plugin)?;
        
        Ok(())
    }
}
```

### Daten-Migration
```javascript
// Plugin-Daten-Migration
class PluginMigration {
    static async migrateFrom(oldVersion, newVersion, data) {
        const migrations = {
            '1.0.0': (data) => {
                // Migration von 1.0.0 zu 1.1.0
                return { ...data, newField: 'default' };
            },
            '1.1.0': (data) => {
                // Migration von 1.1.0 zu 1.2.0
                return { ...data, anotherField: [] };
            }
        };

        let currentData = data;
        const versions = Object.keys(migrations).sort();
        
        for (const version of versions) {
            if (this.isVersionGreater(version, oldVersion) && 
                this.isVersionLessOrEqual(version, newVersion)) {
                currentData = migrations[version](currentData);
            }
        }
        
        return currentData;
    }
}
```

Diese API-Dokumentation bietet eine vollständige Referenz für die Entwicklung von Plugins für den Ora Browser. Weitere Beispiele und Tutorials finden Sie in den entsprechenden Verzeichnissen des Projekts. 