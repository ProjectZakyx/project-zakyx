# Projektüberblick (6–10 Bulletpoints)
- Rust 2021, Single-Crate Tauri v2 App ohne `src-tauri/` (Konfig in `tauri.conf.json` im Root).
- Frontend: Vanilla JS-Module mit eigenem Bundling (`frontend/build.js`) zu `dist/app.js`; kein Vite/Webpack/TS.
- Tauri v2 Plugins im `Cargo.toml` (shell/dialog/http); zur Laufzeit nur `shell` registriert.
- Browser-Domäne: Tabs, Navigation, Bookmarks, History, Settings; eigener Warp-Proxy mit Health-Check.
- IPC: Umfassende `#[tauri::command]`-API für Tabs/Navi/Bookmarks/Settings/Plugins.
- Events: Navigation- und Load-Events aus Rust an das WebView; keine Listener im Frontend gefunden.
- Persistenz: JSON-Dateien (`bookmarks.json`, `settings.json`, `history.json`) + TOML Config unter OS-Config-Pfad.
- Windows: WebView2-Integration (eigene WebView2-Module); Linux: WebKitGTK 4.1-Stack in CI/Script.
- CI/CD: Cross-Platform Build/Test/Artifacts via GitHub Actions; Bundling in Tauri deaktiviert.
- Sicherheit: CSP deaktiviert, Asset-Protocol weit offen; Shell-Plugin ohne Scope-Konfig ersichtlich.

# Dateibaum (gekürzt)
```text
.
├─ Cargo.toml
├─ Cargo.lock
├─ tauri.conf.json
├─ build.rs
├─ Makefile
├─ scripts/
│  └─ build-linux.sh
├─ frontend/
│  ├─ package.json
│  ├─ build.js
│  ├─ zakyxBrowser.js
│  ├─ modules/
│  │  ├─ core.js
│  │  ├─ navigation.js
│  │  ├─ tabManager.js
│  │  └─ bookmarkManager.js
│  └─ utils/
│     └─ utils.js
├─ src/
│  ├─ main.rs
│  ├─ lib.rs
│  ├─ config.rs
│  ├─ proxy_server.rs
│  ├─ tauri_commands/
│  │  ├─ navigation.rs
│  │  ├─ tab_management.rs
│  │  ├─ bookmark_management.rs
│  │  ├─ history.rs
│  │  ├─ settings.rs
│  │  └─ plugin_management.rs
│  ├─ browser/ ... (tabs, bookmarks, history, features)
│  ├─ ui/ ... (bookmarks toolbar, fullscreen, shortcuts)
│  ├─ proxy/ ... (core, strategies, processing, utils)
│  ├─ plugin/ ... (discovery, loader, management, types)
│  ├─ webview2/ ... (config, engine, env, perf)
│  ├─ platform/ ... (windows/macos/linux)
│  ├─ error/ ... & error_system/ ...
│  └─ weitere *.rs (state, logging, metrics ...)
├─ tests/
│  ├─ smoke_tests.rs
│  ├─ integration_tests.rs
│  └─ error_handling_tests.rs
├─ .github/workflows/
│  └─ deploy.yml
├─ resources/
│  └─ manifest.xml
├─ release/ ... (Artifacts)
└─ (diverse *.md im Root und docs/)
```

# Tech-Stack & Abhängigkeiten (Rust/Tauri/Frontend [+ Python/Java falls vorhanden])
- Rust
  - **Edition**: 2021; **Release-Profil**: `lto=true`, `opt-level="s"`, `panic="abort"`.
  - **Async/Net**: `tokio`, `reqwest` (rustls, http2), `warp` (HTTP-Proxy).
  - **Fehler/Logging**: `thiserror`, `anyhow`, `tracing`, `tracing-subscriber`, `tracing-appender`.
  - **Sonstiges**: `serde`/`serde_json`, `chrono`, `uuid`, `regex`, `rand`, `dirs`, `toml`, `windows`-crate.
  - **Features**: `default=["custom-protocol"]` → aktiviert `tauri/custom-protocol`.
- Tauri
  - **Version**: 2.0; `tauri-build = 2.0`.
  - **Plugins**: `tauri-plugin-shell`, `tauri-plugin-dialog`, `tauri-plugin-http` (registriert: nur `shell`).
  - **Konfig** (`tauri.conf.json`): `frontendDist: "./dist"`, Fenster `url: "index.html"`, `security.csp: null`, `assetProtocol.enable: true` mit `scope: ["**"]`, `bundle.active: false`.
- Frontend
  - **Stack**: Node >= 14, reines JS; eigener Bundler `frontend/build.js` → `dist/app.js`.
  - **Nicht gefunden**: Vite/Webpack/TS/ESLint/Prettier-Konfigs.
- Windows
  - WebView2 (wry); Registry-Check im CI; Windows-APIs via `windows`-crate-Features.
- Linux
  - CI/Script: `gtk3`, `webkit2gtk-4.1`, `javascriptcoregtk-4.1`, `libsoup-3.0`, `libappindicator`, `librsvg` etc.
- Python/Java
  - Keine Submodule/Manifeste gefunden.

# Was–Wo–Wie (Kernlogik & Datenfluss)
- **Was**: Desktop-Browser mit Tabs, Navigation, Bookmarks, History, Settings, Plugin-Framework, optional Proxy-Routing/Content-Injection.
- **Wo**
  - Tauri-Entry: `src/main.rs`.
  - Commands: `src/tauri_commands/*` (Navigation/Tab/Bookmarks/Settings/History/Plugins).
  - Proxy: `src/proxy_server.rs` + `src/proxy/*`.
  - WebView2/Plattform: `src/webview2/*`, `src/platform/*`.
  - UI/State: `src/ui/*`, `src/browser_state.rs`.
  - Frontend: `frontend/modules/*.js`, Bundle: `dist/app.js`.
- **Wie**
  - Frontend → Rust: `window.__TAURI__.core.invoke("<command>")`.
  - Rust → Frontend: `window.emit("<event>", payload)`.
  - Proxy-Server (Warp) in separatem Thread; Health-Check `/health`, universelle Proxy-Route.
  - Persistenz über JSON-Dateien im Arbeitsverzeichnis; zentrale TOML-Config im OS-Config-Verzeichnis.

# Architektur & Module (mit Pfaden)
- **Frontend**: UI/State in `frontend/modules/*`, Build nach `dist/app.js`.
- **Tauri**: App-Setup/Window/IPC in `src/main.rs`, Commands in `src/tauri_commands/*`, Plugins-Init (`shell`).
- **Rust-Core**
  - Domain: `src/browser/*`, `src/ui/*`, `src/plugin/*`, `src/proxy/*`.
  - Ports/Adapter: `src/webview2/*`, `src/platform/*`, `src/proxy_server.rs`.
  - Fehler: `src/error/*`, `src/error_system/*`, `ZAKYXBrowserError`.
- **Concurrency**: `tokio`-Async; Proxy in eigenem Thread mit Multi-Thread-Runtime.

# Tauri-Bridge (Commands, Events, Plugins)
- **Commands**
  - Navigation (`src/tauri_commands/navigation.rs`): `navigate_to`, `internal_webview_navigate`, `navigate_internally`, `navigate_and_get_content -> String`, `check_internal_navigation -> bool`, `get_webview_config -> WebViewConfig`, `get_proxy_url -> String`, `open_external_url`.
  - Tabs (`src/tauri_commands/tab_management.rs`): `create_new_tab -> Tab`, `close_tab`, `get_tabs -> Vec<Tab>`, `update_tab_title`, `activate_tab`, `duplicate_tab -> Tab`, `update_tab_url`, `get_active_tab -> Option<Tab>`, `get_tab_count -> usize`.
  - Bookmarks (`src/tauri_commands/bookmark_management.rs`): `add_bookmark -> Bookmark`, `get_bookmarks -> Vec<Bookmark>`, `remove_bookmark`, `sync_bookmarks -> Vec<Bookmark>`.
  - Settings (`src/tauri_commands/settings.rs`): `get_settings -> BrowserSettings`, `update_settings`.
  - History (`src/tauri_commands/history.rs`): `get_history -> Vec<String>`, `clear_history`, `add_to_history`.
  - Plugins (`src/tauri_commands/plugin_management.rs`): `get_all_plugins` (TODO), `get_loaded_plugins` (TODO), `enable_plugin`, `disable_plugin`, `load_plugin`, `unload_plugin`.
- **Events** (emit in `navigation.rs`)
  - `"webview_navigate"`, `"webview_loaded"`, `"internal_webview_navigate"`, `"internal_webview_loaded"`, `"navigation_success"`.
  - Hinweis: Frontend-Listener nicht gefunden.
- **Plugins**
  - Registriert: `tauri_plugin_shell::init()` in `src/main.rs`.
  - Dependencies vorhanden: shell/dialog/http (dialog/http aktuell nicht registriert).

# API & Schnittstellen (öffentlich)
- **Rust**: Öffentliche Module via `src/lib.rs` (u. a. `BrowserState`, `ProxyServer`, `ZAKYXConfig`, WebView2/Platform-APIs).
- **IPC**: Frontend-Aufrufe via `window.__TAURI__.core.invoke("<command>")`; Events via `window.emit`.

# Datenmodelle & Persistenz
- **Modelle**: `Tab`, `Bookmark`, `BrowserSettings`, `WebViewConfig`, diverse Plugin-/Proxy-/Error-Typen.
- **Persistenz**
  - JSON-Dateien: `bookmarks.json`, `settings.json`, `history.json` im Projektverzeichnis.
  - App-Config (TOML): `dirs::config_dir()/zakyx-browser/config.toml` (`src/config.rs`).
- **DB/ORM**: nicht vorhanden.

# Build/Run/Test/Deploy (Befehle)
- **Frontend (Dev/Build)**: `cd frontend && npm install && npm run dev` bzw. `npm run build` → erzeugt `../dist/app.js`.
- **App (Dev)**: `cargo run` (oder `cargo tauri dev`).
- **Build**: `cargo build --release` (Tauri-Bundle deaktiviert).
- **Tests/Lint/Format**: `cargo test`, `cargo clippy --all-targets --all-features -D warnings`, `cargo fmt --all`.
- **CI/CD**: `.github/workflows/deploy.yml` baut/testet Linux/Windows/macOS; veröffentlicht Artifacts und Release mit Checksums.

# Qualität & Sicherheit (Kurzcheck)
- **CSP**: `null` + `dangerousDisableAssetCspModification: true` → zu permissiv.
- **Asset-Protocol**: `enable: true`, `scope: ["**"]` → weit offen; Scope reduzieren.
- **Shell-Plugin**: keine Scope-Restriktionen ersichtlich → einschränken.
- **IPC**: Events ohne Frontend-Listener → fehlende Rückkanalvalidierung.
- **Errors**: `Result<..., ZAKYXBrowserError>` breit genutzt; Error-System vorhanden.

# TODOs, Risiken, Unknowns
- Fehlendes `dist/index.html` bei `tauri.conf.json`-Fenster `url: "index.html"` → Startfehler möglich.
- `uuid` → `parse::<u32>().unwrap()` für Tab-IDs kann paniken (UUID ≠ `u32`).
- Plugin-Listing (`get_all_plugins`/`get_loaded_plugins`) nicht implementiert.
- Events werden emittiert, aber Frontend-Listener fehlen.
- Shell-Plugin-Scopes nicht eingeschränkt; CSP/Asset-Protocol sehr permissiv.
- Linux Deps: Makefile (4.0) vs CI (4.1) → Versionsdrift.

# Onboarding-Plan (Tag 1 / Woche 1)
- **Tag 1**
  - Install: Rustup (stable), Node 18 LTS (>=14), optional `cargo install tauri-cli`.
  - Windows: WebView2 Runtime prüfen (Win11 meist vorinstalliert).
  - Linux: `gtk3`, `webkit2gtk-4.1`, `javascriptcoregtk-4.1`, `libsoup-3.0`, `libappindicator`, `librsvg` installieren.
  - Frontend bauen: `cd frontend && npm install && npm run build`.
  - `dist/index.html` anlegen (Container mit `<script src="app.js">`).
  - App starten: `cargo run` (oder `cargo tauri dev`); Proxy-Health `http://localhost:3030/health` prüfen.
- **Woche 1**
  - Fix Tab-ID-Generierung (keine `unwrap()`-Panics; UUID als String verwenden).
  - `dist/index.html` + Tauri-Event-Listener im Frontend verdrahten.
  - Plugin-Listing implementieren.
  - Shell-Scope einschränken; CSP/Asset-Protocol härten.
  - Makefile auf WebKitGTK 4.1 angleichen; einfache Frontend-Smoketests.

# Quick Wins (konkret)
- **Security**: CSP setzen; `dangerousDisableAssetCspModification` entfernen.
- **Assets**: Asset-Protocol-Scope reduzieren (keine `"**"`).
- **Shell**: erlaubte Kommandos/Argumente whitelisten.
- **Stabilität**: Tab-ID-Erzeugung korrigieren (keine `parse::<u32>()`).
- **Frontend**: `dist/index.html` hinzufügen; Listener für `"webview_*"` Events.
- **Konsistenz**: WebKitGTK-Versionen vereinheitlichen (4.1).
- **CI**: `cargo audit` in Workflow integrieren.

# Erweiterungspunkte (mit Pfaden)
- **Neue Commands**: `src/tauri_commands/*.rs` (Modul ergänzen; `generate_handler!` in `src/main.rs` erweitern).
- **Plugins/Services**: `src/plugin/*` (Discovery/Loader/Registry erweitern).
- **Proxy-Strategien**: `src/proxy/strategies/*`; Content-Processing: `src/proxy/processing/*`.
- **Frontend-Module**: `frontend/modules/*` + `build.js`-Modulliste.
- **Domain-Module**: `src/browser/*`, `src/ui/*`.
- **WebView/Platform**: `src/webview2/*`, `src/platform/*`.


# 10) Plattform-Packaging & Signatur
- **Windows (MSIX/EXE)**
  - Aktuell: Bundling in `tauri.conf.json` deaktiviert (`"bundle.active": false`).
  - Pfad: `tauri.conf.json` (Root). Aktivieren für Packaging: `"bundle.active": true` und `bundle.windows`-Optionen ergänzen.
  - Build-Befehle:
    - `cargo tauri build --target x86_64-pc-windows-msvc`
    - Signatur (Beispiel): `signtool sign /fd SHA256 /tr http://timestamp.digicert.com /td SHA256 /a "path\\to\\zakyx-browser.exe"`
  - WebView2: Evergreen (empfohlen). Fixed-Version optional via Tauri-Config (nicht gesetzt).
- **Linux (AppImage/DEB/RPM/Flatpak)**
  - Aktuell keine Paket-Manifeste im Repo; CI erzeugt nur Binärartefakte (`.github/workflows/deploy.yml`).
  - Systempakete zur Laufzeit: `libgtk-3-0`, `libwebkit2gtk-4.1-0`, `libjavascriptcoregtk-4.1-0`, `libsoup-3.0-0` (siehe CI).
  - Packaging-Optionen (Vorschlag): AppImage/DEB/RPM via Tauri-Bundler nach Aktivierung → `cargo tauri build`.
- **macOS** (nur bei Bedarf)
  - Codesign/Notarization: nicht konfiguriert. Tauri-Bundle aktivieren und `APPLE_ID`/`APP_SPECIFIC_PASSWORD` in CI setzen.

# 11) Update-Mechanismus & Rollback
- Status: Kein Tauri-Updater konfiguriert (`tauri.conf.json` ohne `updater`, kein `tauri-plugin-updater`).
- Empfehlung:
  - Rust: `tauri-plugin-updater` hinzufügen; Frontend: ggf. `@tauri-apps/plugin-updater`.
  - Konfiguration: `tauri.conf.json` → `updater` Quelle (Release-Feed), Signatur/Kanal (stable/beta).
  - Rollback: Version-Pinning + Beibehaltung letzter Version im User-Datenpfad.
- Update-Fluss: Start → Feed prüfen → signiertes Paket laden → Validierung → Anwenden → Neustart → bei Fehler Rollback.

# 12) Crash-/Fehler-Handling & Observability
- Logs: `dirs::config_dir()/zakyx-browser/logs/zakyx-browser.log` (daily rolling) – `src/main.rs::init_logging()`.
- Log-Level: via `RUST_LOG` (EnvFilter). JSON-Logfile aktiv.
- Runbook – Debug-Logs:
  - Windows: `$env:RUST_LOG="debug"; cargo run`
  - Linux/macOS: `RUST_LOG=debug cargo run`
- Telemetrie: keine externe Telemetrie im Code gefunden.

# 13) Datenschutz & DSGVO
- Speicherung: `bookmarks.json`, `history.json`, `settings.json` (Projektwurzel); `config.toml` im OS-Config-Verzeichnis.
- Minimierung/Dauer: nicht geregelt; History max 1000 Einträge.
- Export/Löschen: Commands `clear_history`, `remove_bookmark`, `sync_bookmarks`.
- Offene Fragen (DPIA): Rechtsgrundlage, Speicherfristen, Export-Schnittstelle, Consent/Opt-outs.

# 14) Internationalisierung (i18n) & Barrierefreiheit (a11y)
- i18n: nicht vorhanden; Strings in `frontend/modules/*` zentralisieren, Loader in `frontend/modules/core.js`.
- a11y: keine speziellen Hooks; Fokus/Shortcuts in `src/ui/*` ausbaubar.

# 15) Performance & Footprint
- Rust: aktiv `lto=true`, `opt-level="s"`, `panic="abort"`; erwäge PGO.
- Frontend: `npm run build:min` für Minify; Ziel: `dist/app.min.js` ≤ 300 KB, Cold Start < 2.0s.
- Messen: `hyperfine 'target/release/zakyx-browser'` (Startzeit); Artefaktgrößen via CI-Logs/`ls -lh`.

# 16) Reproduzierbarkeit & Supply Chain
- Lockfiles: `Cargo.lock` vorhanden; kein npm-Lockfile.
- Prüfen: `cargo audit`, optional `cargo deny`, SBOM via `cyclonedx-cargo` oder `cargo auditable`.

# 17) Release-Management & Versionierung
- Tags `v*` triggern CI-Release.
- Kurz-Checkliste:
  - [ ] Version erhöhen (`Cargo.toml`)
  - [ ] `cargo fmt && cargo clippy && cargo test`
  - [ ] Frontend `npm run build`
  - [ ] Bundling-Einstellung prüfen (`tauri.conf.json`)
  - [ ] Git-Tag `vX.Y.Z` pushen

# 18) Branching/Workflow & Code-Standards
- CI-Jobs: Build/Test/Artifacts; Lint/Format via Makefile-Targets vorhanden.
- Empfehlung: Conventional Commits + Pre-commit Hooks (fmt/clippy/test).

# 19) Persistenz-/Schema-Migrationen
- Datei-Schema ohne Versionsfeld. Migrationen beim Laden in `src/config.rs::validate_and_fix()` anlehnen.
- Empfehlung: JSON-Versionierung + Migrationspfad beim Lesen.

# 20) Cross-Plattform Testmatrix
- OS: Windows x64, Linux x64, macOS (optional)
- Modi: Debug/Release, Proxy an/aus
- Befehle:
  - `cargo test`
  - `cargo build --release --target x86_64-pc-windows-msvc`
  - `cargo build --release --target x86_64-unknown-linux-gnu`
  - `cd frontend && npm test`

# 21) Sicherheits-Hardening (vertieft)
- `tauri.conf.json`: `security.csp` setzen; `dangerousDisableAssetCspModification` entfernen.
- `assetProtocol.scope`: von `"**"` auf benötigte Pfade reduzieren.
- Shell-Scope: nur erlaubte Kommandos/Argumente zulassen.
- Optional Sandbox: Flatpak/AppArmor (Linux).

# 22) Entwickler-Umgebung & DX
- Toolchains: Rust stable, Node ≥ 14 (empf. 18), Tauri-CLI optional.
- 5 Schritte – From Zero to Dev:
  1. `rustup default stable`
  2. `cd frontend && npm install && npm run build`
  3. `cargo fmt && cargo clippy`
  4. `cargo run` (oder `cargo tauri dev`)
  5. Smoke-Test: Proxy-Health, Tab, Navigation, Bookmarks

# 23) Abhängigkeits-/Upgrade-Plan
- Kern: `tauri 2`, `tokio 1.42`, `reqwest 0.12`, `warp 0.3`, `windows 0.61`, `tracing 0.1`.
- Roadmap:
  1. `cargo update` + `cargo audit` fixen
  2. Allowlist/Plugins härten
  3. Frontend-Minify/Bundle-Budget
  4. Optional: PGO aktivieren

# 24) Abschließende Checkliste (Output)
- [ ] App startet mit `dist/index.html` und rendert UI
- [ ] `cargo test` grün in allen CI-Targets
- [ ] Frontend `npm run build` erzeugt `dist/app.js` (≤ 300 KB min)
- [ ] `bundle.active: true` wenn Installer benötigt
- [ ] Windows-Build signiert (SignTool) und SmartScreen-OK
- [ ] Linux-Runtime-Deps dokumentiert (GTK/WebKitGTK/libsoup)
- [ ] CSP gesetzt, `dangerousDisableAssetCspModification` entfernt
- [ ] Asset-Protocol-Scope eingeschränkt
- [ ] Shell-Plugin-Scope eingeschränkt
- [ ] Logs rotieren; Debug-Logs per `RUST_LOG` aktivierbar
- [ ] Updater konfiguriert/signiert ODER bewusst deaktiviert
- [ ] Datenschutzfragen (DPIA) geklärt
- [ ] Performance-Budgets eingehalten (Startzeit/Binary/Bundle)
- [ ] SBOM/`cargo audit` läuft in CI
