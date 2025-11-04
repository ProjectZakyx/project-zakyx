# 🌐 ZAKYX Browser

**A modern, security-oriented web browser with advanced features and cross-platform support.**

[🇩🇪 Deutsche Version](README_DE.md) | [🇬🇧 English Version](README.md)

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![Tauri](https://img.shields.io/badge/tauri-%2324C8DB.svg?style=for-the-badge&logo=tauri&logoColor=%23FFFFFF)
![Windows](https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=windows&logoColor=white)
![macOS](https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=macos&logoColor=F0F0F0)
![Linux](https://img.shields.io/badge/Linux-FCC624?style=for-the-badge&logo=linux&logoColor=black)

## ✨ Features

### 🚀 **Core Features**
- **Modern Browser Engine** with WebView2 Integration
- **Tab Management** with dynamic tab creation and management
- **Bookmark System** with persistent storage
- **History Management** for visited pages
- **Settings Management** with customizable configurations

### 🔌 **Plugin System**
- **Dynamic Plugin Loading** at runtime
- **Permission System** for secure plugin execution
- **Plugin Manager** with Enable/Disable functionality
- **API Versioning** for plugin compatibility
- **Manifest-based Configuration**

### 🛡️ **Security Features**
- **Smart Proxy System** for problematic websites
- **URL Normalization** and validation
- **Ethical Safeguards** with rate limiting
- **Header Stripping** for iframe compatibility
- **Domain-specific Fallbacks**

### 🏗️ **Architecture**
- **Modular Structure** with clear separation of concerns
- **Async/Await** for performant I/O operations
- **Thread-safe State Management** with RwLock
- **Comprehensive Test Coverage** (Unit + Integration Tests)
- **Plugin API** for extensibility

## 🚀 Installation

### Prerequisites
- **Rust** (1.70+)
- **Node.js** (16+)
- **WebView2** (Windows)
- **Tauri CLI**

```bash
# Install Tauri CLI
cargo install tauri-cli

# Clone repository
git clone https://github.com/ProjectZakyx/project-zakyx.git
cd project-zakyx

# Install dependencies
cargo build --release

# Build frontend
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

## 📖 Usage

### Start Browser
```bash
# Development
cargo run

# Production
./target/release/zakyx-browser

# With PowerShell Launcher (Windows)
.\launch_zakyx_browser_with_gui.ps1
```

### Plugin Development
```json
{
  "name": "My Plugin",
  "version": "1.0.0",
  "description": "An example plugin",
  "author": "Your Name",
  "main_script": "main.js",
  "permissions": ["network", "storage"],
  "api_version": "1.0",
  "enabled": true
}
```

### API Usage
```javascript
// Create tab
await invoke('create_new_tab', { url: 'https://example.com' });

// Add bookmark
await invoke('add_bookmark', { 
  title: 'Example', 
  url: 'https://example.com' 
});

// Load plugin
await invoke('load_plugin', { plugin_id: 'my-plugin' });
```

## 🏗️ Architecture

### Module Structure
```
src/
├── main.rs                    # Main application (87 lines)
├── browser_state.rs           # Browser State & Data Structures
├── tauri_commands.rs          # All Tauri Commands
├── plugin_manager.rs          # Plugin System
├── url_utils.rs              # URL Helper Functions
├── proxy_server.rs           # Smart Proxy System
├── smart_proxy.rs            # Advanced Proxy Logic
├── browser_features.rs       # Browser Features
├── ethical_safeguards.rs     # Security Features
└── internal_webview2_navigation.rs  # WebView2 Integration
```

### Plugin System
```
extensions/
├── antibot-plugin/           # Anti-Bot Plugin (Example)
│   ├── plugin.json          # Plugin Manifest
│   ├── antibot-plugin.js    # Main Script
│   └── ui/                  # UI Components
└── your-plugin/             # Your Plugin
    ├── plugin.json
    └── main.js
```

## 🧪 Tests

### Test Coverage
- **17 Unit Tests** (100% Pass Rate)
- **4 Integration Tests** (100% Pass Rate)
- **Comprehensive Module Tests**

```bash
# Run all tests
cargo test

# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test integration_tests
```

## 🔌 Plugin Development

### Create Plugin
1. **Create directory**: `extensions/my-plugin/`
2. **Create manifest**: `plugin.json`
3. **Develop script**: `main.js`
4. **Install plugin**: Via Plugin Manager

### Permissions
- `network` - Network access
- `storage` - Local storage
- `tabs` - Tab management
- `bookmarks` - Bookmark access
- `history` - History access
- `settings` - Settings access

## 🛠️ Development

### Contributing
1. **Fork** the repository
2. **Create** feature branch
3. **Add** tests
4. **Create** pull request

## 📊 Performance

### Metrics
- **Startup Time**: < 2 seconds
- **Memory Usage**: < 100MB (Base)
- **Plugin Loading**: < 500ms
- **Tab Creation**: < 100ms

## 🔒 Security

### Security Features
- **Plugin Sandboxing** with permission system
- **URL Validation** and normalization
- **Rate Limiting** for network requests
- **Header Stripping** for secure iframe embedding
- **Ethical Safeguards** for responsible browsing

## 🤝 Community

### Support
- **GitHub Issues** for bug reports
- **Discussions** for feature requests
- **Wiki** for documentation

### License
MIT License - see [LICENSE](LICENSE) for details.

---

**Developed with ❤️ and Rust**

*ZAKYX Browser - Redefining browsing* 
