/**
 * OraBrowser Core Module
 * Grundlegende Browser-Funktionalität und Initialisierung
 */

class OraBrowserCore {
    constructor() {
        this.isLoading = false;
        this.currentUrl = '';
        this.settings = {
            homepage: 'https://www.google.com',
            searchEngine: 'https://www.google.com/search?q=',
            privacyMode: false,
            adBlocker: false,
            javascriptEnabled: true,
            cookiesEnabled: true
        };
        
        this.pluginManager = {
            loadedPlugins: 4,
            enabledPlugins: 3
        };
        
        console.log('🚀 OraBrowser Core initialized');
    }

    // 🔗 TAURI API PRÜFUNG
    checkTauriAPI() {
        console.log('🔧 === TAURI API DEBUG ===');
        console.log('🔧 window.__TAURI__:', window.__TAURI__);
        console.log('🔧 window.__TAURI_INTERNALS__:', window.__TAURI_INTERNALS__);
        console.log('🔧 window.__TAURI_METADATA__:', window.__TAURI_METADATA__);
        console.log('🔧 window.location.protocol:', window.location.protocol);
        
        if (window.__TAURI__) {
            console.log('🔧 __TAURI__ keys:', Object.keys(window.__TAURI__));
            if (window.__TAURI__.core) {
                console.log('🔧 __TAURI__.core keys:', Object.keys(window.__TAURI__.core));
            }
        }
        
        const hasAPI = !!(window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke);
        console.log('🔧 Tauri API available:', hasAPI);
        console.log('🔧 === END TAURI DEBUG ===');
        
        return hasAPI;
    }

    // URL Normalisierung
    normalizeUrl(url) {
        if (!url || typeof url !== 'string') {
            return 'about:blank';
        }
        
        url = url.trim();
        
        if (!url) {
            return 'about:blank';
        }
        
        if (url === 'about:blank' || url.startsWith('about:') || url.startsWith('data:')) {
            return url;
        }
        
        if (url.startsWith('http://') || url.startsWith('https://')) {
            return url;
        }
        
        if (url.startsWith('localhost') || url.match(/^\d+\.\d+\.\d+\.\d+/)) {
            return `http://${url}`;
        }
        
        if (url.includes('.') && !url.includes(' ')) {
            return `https://${url}`;
        }
        
        const settings = this.getSettings();
        const searchEngine = settings.searchEngine || 'https://www.google.com/search?q=';
        
        let searchUrl = searchEngine;
        if (!searchUrl.includes('?q=')) {
            searchUrl = searchUrl.endsWith('/') ? searchUrl + 'search?q=' : searchUrl + '/search?q=';
        }
        
        return `${searchUrl}${encodeURIComponent(url)}`;
    }

    // Domain aus URL extrahieren
    extractDomain(url) {
        try {
            const urlObj = new URL(url);
            return urlObj.hostname;
        } catch (error) {
            const match = url.match(/^(?:https?:\/\/)?(?:www\.)?([^\/]+)/);
            return match ? match[1] : url;
        }
    }

    // Settings Management
    getSettings() {
        try {
            const savedSettings = localStorage.getItem('ora-browser-settings');
            if (savedSettings) {
                return JSON.parse(savedSettings);
            }
        } catch (error) {
            console.error('❌ Error loading settings:', error);
        }
        
        return {
            homepage: 'https://www.google.com',
            searchEngine: 'https://www.google.com/search?q=',
            privacyMode: false,
            adBlocker: false,
            javascriptEnabled: true,
            cookiesEnabled: true
        };
    }

    saveSettings(settings) {
        try {
            localStorage.setItem('ora-browser-settings', JSON.stringify(settings));
            console.log('💾 Settings saved to localStorage');
            this.settings = { ...this.settings, ...settings };
            return true;
        } catch (error) {
            console.error('❌ Error saving settings:', error);
            this.updateStatus('❌ Fehler beim Speichern der Einstellungen');
            return false;
        }
    }

    // Status Updates
    updateStatus(message) {
        const statusText = document.getElementById('status-text');
        if (statusText) {
            statusText.textContent = message;
        }
        console.log(`📊 Status: ${message}`);
    }

    // Loading State Management
    setLoading(loading) {
        this.isLoading = loading;
        const loadingIndicator = document.getElementById('loading-indicator');
        
        if (loadingIndicator) {
            loadingIndicator.style.display = loading ? 'flex' : 'none';
        }
        
        const reloadBtn = document.getElementById('reload-btn');
        if (reloadBtn) {
            reloadBtn.disabled = loading;
            reloadBtn.style.opacity = loading ? '0.5' : '1';
        }
    }

    // Grundlegende Initialisierung
    initializeCore() {
        console.log('🔧 Initializing OraBrowser Core...');
        
        this.settings = this.getSettings();
        console.log('⚙️ Settings loaded:', this.settings);
        
        this.updateStatus('Bereit');
        
        console.log('🌐 Ora Browser v1.0.0 - Core Ready');
    }
}

// Global verfügbar machen
window.OraBrowserCore = OraBrowserCore;

export default OraBrowserCore; 