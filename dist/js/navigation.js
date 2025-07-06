/**
 * OraBrowser Navigation Manager
 * Verantwortlich für URL-Navigation, Proxy-Handling und Content-Display
 */

import { UrlUtils } from './utils.js';

export class NavigationManager {
    constructor(core) {
        this.core = core;
        this.history = [];
        this.historyIndex = -1;
        this.proxyUrl = 'http://localhost:3030/proxy';
        
        console.log('🧭 Navigation Manager initialized');
    }

    // Hauptnavigations-Methode
    async navigateToUrl(url, updateTab = true, addToHistoryFlag = true) {
        console.log('🌐 Navigating to:', url);
        
        if (!url || url.trim() === '') {
            console.warn('⚠️ Empty URL provided');
            return false;
        }

        // URL normalisieren
        const normalizedUrl = this.core.normalizeUrl(url);
        console.log('🔧 Normalized URL:', normalizedUrl);

        // Loading-Status setzen
        this.core.setLoading(true);
        this.core.updateStatus(`Lade ${normalizedUrl}...`);

        try {
            // Spezielle Behandlung für bestimmte URLs
            if (normalizedUrl === 'gui' || normalizedUrl.includes('gui')) {
                return await this.handleGUINavigation();
            }

            if (normalizedUrl.includes('google.com')) {
                return await this.handleGoogleNavigation(normalizedUrl);
            }

            // Standard Proxy-Navigation
            const success = await this.tryProxyNavigation(normalizedUrl);
            
            if (success) {
                this.core.currentUrl = normalizedUrl;
                
                if (addToHistoryFlag) {
                    this.addToHistory(normalizedUrl);
                }
                
                this.core.updateStatus(`Geladen: ${this.core.extractDomain(normalizedUrl)}`);
                console.log('✅ Navigation successful');
                return true;
            } else {
                throw new Error('Proxy navigation failed');
            }

        } catch (error) {
            console.error('❌ Navigation error:', error);
            this.showErrorPage(normalizedUrl, error);
            return false;
        } finally {
            this.core.setLoading(false);
        }
    }

    // Navigation zurücksetzen
    resetNavigationState() {
        this.core.setLoading(false);
        this.core.updateStatus('Bereit');
        
        const contentContainer = document.getElementById('content-container');
        if (contentContainer) {
            contentContainer.innerHTML = '';
        }
        
        console.log('🔄 Navigation state reset');
    }

    // Proxy-Navigation versuchen
    async tryProxyNavigation(url) {
        try {
            const proxyFullUrl = `${this.proxyUrl}?url=${encodeURIComponent(url)}`;
            console.log('🔗 Proxy request:', proxyFullUrl);

            const response = await fetch(proxyFullUrl, {
                method: 'GET',
                headers: {
                    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
                }
            });

            if (!response.ok) {
                throw new Error(`HTTP ${response.status}: ${response.statusText}`);
            }

            const content = await response.text();
            console.log('📦 Content received:', content.length, 'bytes');

            if (content && content.length > 0) {
                const optimizedContent = this.optimizeContent(content, url);
                this.displayContent(optimizedContent);
                return true;
            } else {
                throw new Error('Empty content received');
            }

        } catch (error) {
            console.error('❌ Proxy navigation error:', error);
            return false;
        }
    }

    // Google-Navigation speziell behandeln
    async handleGoogleNavigation(url) {
        console.log('🔍 Handling Google navigation:', url);
        
        try {
            const success = await this.tryProxyNavigation(url);
            
            if (success) {
                // Zusätzliche Google-Optimierungen
                setTimeout(() => {
                    this.optimizeGoogleContent();
                }, 1000);
                
                return true;
            } else {
                // Fallback auf iframe
                return this.createOptimizedGoogleIframe(url);
            }
            
        } catch (error) {
            console.error('❌ Google navigation error:', error);
            return this.createOptimizedGoogleIframe(url);
        }
    }

    // Content optimieren
    optimizeContent(content, url) {
        console.log('⚡ Optimizing content for:', url);
        
        // Security Script einfügen
        const securityScript = this.createSecurityScript();
        
        // Base-Tag für relative URLs
        const baseTag = `<base href="${new URL(url).origin}/">`;
        
        // Content-Optimierungen
        let optimizedContent = content
            // Meta-Tags hinzufügen
            .replace('<head>', `<head>\n${baseTag}\n${securityScript}`)
            // Externe Links in neuem Tab öffnen
            .replace(/<a\s+([^>]*?)href=["']([^"']*?)["']([^>]*?)>/gi, 
                (match, before, href, after) => {
                    if (href.startsWith('http') && !href.includes(new URL(url).hostname)) {
                        return `<a ${before}href="${href}"${after} target="_blank" rel="noopener noreferrer">`;
                    }
                    return match;
                })
            // Formulare für Proxy umleiten
            .replace(/<form\s+([^>]*?)action=["']([^"']*?)["']([^>]*?)>/gi,
                (match, before, action, after) => {
                    if (action && !action.startsWith('javascript:')) {
                        const fullAction = new URL(action, url).href;
                        return `<form ${before}action="${this.proxyUrl}?url=${encodeURIComponent(fullAction)}"${after}>`;
                    }
                    return match;
                });

        console.log('✨ Content optimization complete');
        return optimizedContent;
    }

    // Security Script erstellen
    createSecurityScript() {
        return `
<script>
(function() {
    'use strict';
    
    // Sichere Referenz auf OraBrowser
    function getOraBrowser() {
        // Versuche verschiedene Wege zum OraBrowser-Objekt
        if (window.parent && window.parent.oraBrowser) {
            return window.parent.oraBrowser;
        }
        if (window.top && window.top.oraBrowser) {
            return window.top.oraBrowser;
        }
        if (window.parent && window.parent.parent && window.parent.parent.oraBrowser) {
            return window.parent.parent.oraBrowser;
        }
        
        // Fallback: Versuche über postMessage
        return null;
    }
    
    // Link-Interception für interne Navigation
    document.addEventListener('click', function(e) {
        const link = e.target.closest('a');
        if (link && link.href) {
            const url = link.href;
            
            // Versuche OraBrowser-Navigation
            const oraBrowser = getOraBrowser();
            if (oraBrowser && oraBrowser.navigationManager) {
                e.preventDefault();
                console.log('🔗 Intercepted link:', url);
                oraBrowser.navigationManager.navigateToUrl(url);
                return false;
            }
            
            // Fallback: PostMessage an parent
            if (window.parent !== window) {
                e.preventDefault();
                console.log('🔗 Intercepted link (postMessage):', url);
                window.parent.postMessage({
                    type: 'oraBrowser_navigate',
                    url: url
                }, '*');
                return false;
            }
        }
    });
    
    // Form-Interception
    document.addEventListener('submit', function(e) {
        const form = e.target;
        if (form && form.action) {
            console.log('📝 Form submitted to:', form.action);
            
            // Versuche Form-Navigation über OraBrowser
            const oraBrowser = getOraBrowser();
            if (oraBrowser && oraBrowser.navigationManager) {
                e.preventDefault();
                const formData = new FormData(form);
                const searchParams = new URLSearchParams(formData);
                const fullUrl = form.action + '?' + searchParams.toString();
                console.log('📝 Form navigation:', fullUrl);
                oraBrowser.navigationManager.navigateToUrl(fullUrl);
                return false;
            }
        }
    });
    
    // Console-Logging reduzieren
    if (window.console) {
        ['log', 'info', 'warn'].forEach(method => {
            const original = console[method];
            console[method] = function(...args) {
                if (args[0] && typeof args[0] === 'string' && 
                    !args[0].includes('OraBrowser')) {
                    return;
                }
                original.apply(console, args);
            };
        });
    }
    
    console.log('🔒 OraBrowser security script loaded');
})();
</script>`;
    }

    // Content anzeigen
    displayContent(htmlContent) {
        console.log('🖼️ Displaying content...');
        
        // Welcome-Screen verstecken
        this.hideWelcomeScreen();
        
        // WebView-Container anzeigen
        this.showWebView();
        
        // Content in iframe setzen
        const webviewFrame = document.getElementById('webview-frame');
        if (webviewFrame) {
            // Sichere Content-Insertion
            webviewFrame.srcdoc = htmlContent;
            console.log('✅ Content loaded in iframe');
        } else {
            console.error('❌ Webview frame not found');
            
            // Fallback: Content-Container verwenden
            const contentContainer = document.getElementById('content-container');
            if (contentContainer) {
                contentContainer.innerHTML = htmlContent;
                console.log('✅ Content loaded in container (fallback)');
            }
        }
        
        // Link-Interception setup
        setTimeout(() => {
            this.setupLinkInterception();
        }, 500);
    }

    // Welcome-Screen verstecken
    hideWelcomeScreen() {
        const welcomeScreen = document.getElementById('welcome-screen');
        const webviewContainer = document.getElementById('webview-container');
        
        if (welcomeScreen) {
            welcomeScreen.style.display = 'none';
        }
        
        if (webviewContainer) {
            webviewContainer.style.display = 'block';
        }
        
        console.log('🏠 Welcome screen hidden');
    }

    // WebView anzeigen
    showWebView() {
        const webviewContainer = document.getElementById('webview-container');
        if (webviewContainer) {
            webviewContainer.style.display = 'block';
        }
    }

    // Iframe-Fallback erstellen
    createIframeFallback(url) {
        console.log('🖼️ Creating iframe fallback for:', url);
        
        const safeUrl = UrlUtils.makeSafeUrl(url);
        
        const iframeHtml = `
<!DOCTYPE html>
<html>
<head>
    <title>OraBrowser - ${safeUrl}</title>
    <style>
        body { margin: 0; padding: 0; font-family: Arial, sans-serif; }
        .error-container { 
            padding: 40px; 
            text-align: center; 
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
        }
        .error-title { font-size: 2rem; margin-bottom: 1rem; }
        .error-message { font-size: 1.2rem; margin-bottom: 2rem; }
        .retry-btn { 
            padding: 12px 24px; 
            background: rgba(255,255,255,0.2); 
            border: none; 
            border-radius: 8px; 
            color: white; 
            cursor: pointer;
            font-size: 1rem;
        }
        .retry-btn:hover { background: rgba(255,255,255,0.3); }
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-title">🌐 Iframe-Modus</div>
        <div class="error-message">
            Die Seite wird in einem sicheren Iframe geladen:<br>
            <strong>${safeUrl}</strong>
        </div>
        <button class="retry-btn" onclick="window.parent.location.reload()">
            🔄 Seite neu laden
        </button>
    </div>
</body>
</html>`;
        
        this.displayContent(iframeHtml);
        return true;
    }

    // Google-Iframe erstellen
    createOptimizedGoogleIframe(url) {
        console.log('🔍 Creating Google iframe for:', url);
        
        const googleHtml = `
<!DOCTYPE html>
<html>
<head>
    <title>Google Search - OraBrowser</title>
    <style>
        body { margin: 0; padding: 0; }
        .google-container { 
            width: 100%; 
            height: 100vh; 
            border: none;
            display: flex;
            justify-content: center;
            align-items: center;
            background: #f8f9fa;
        }
        .search-frame {
            width: 100%;
            height: 100%;
            border: none;
            background: white;
        }
    </style>
</head>
<body>
    <div class="google-container">
        <iframe src="${url}" class="search-frame" sandbox="allow-scripts allow-same-origin allow-forms allow-top-navigation allow-popups allow-downloads"></iframe>
    </div>
</body>
</html>`;
        
        this.displayContent(googleHtml);
        return true;
    }

    // Error-Page anzeigen
    showErrorPage(url, error) {
        console.log('❌ Showing error page for:', url, error);
        
        const errorHtml = `
<!DOCTYPE html>
<html>
<head>
    <title>Fehler - OraBrowser</title>
    <style>
        body { 
            margin: 0; 
            padding: 40px; 
            font-family: Arial, sans-serif; 
            background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
            color: white;
            min-height: 100vh;
            display: flex;
            flex-direction: column;
            justify-content: center;
            text-align: center;
        }
        .error-title { font-size: 3rem; margin-bottom: 1rem; }
        .error-subtitle { font-size: 1.5rem; margin-bottom: 2rem; opacity: 0.9; }
        .error-details { 
            background: rgba(255,255,255,0.1); 
            padding: 20px; 
            border-radius: 10px; 
            margin: 20px 0;
            text-align: left;
        }
        .retry-btn { 
            padding: 15px 30px; 
            background: rgba(255,255,255,0.2); 
            border: none; 
            border-radius: 8px; 
            color: white; 
            cursor: pointer;
            font-size: 1.1rem;
            margin: 10px;
        }
        .retry-btn:hover { background: rgba(255,255,255,0.3); }
    </style>
</head>
<body>
    <div class="error-title">🚨 Navigationsfehler</div>
    <div class="error-subtitle">Die Seite konnte nicht geladen werden</div>
    
    <div class="error-details">
        <strong>URL:</strong> ${url}<br>
        <strong>Fehler:</strong> ${error.message || error}<br>
        <strong>Zeit:</strong> ${new Date().toLocaleString()}
    </div>
    
    <div>
        <button class="retry-btn" onclick="window.parent.oraBrowser?.navigateToUrl('${url}')">
            🔄 Erneut versuchen
        </button>
        <button class="retry-btn" onclick="window.parent.oraBrowser?.goHome()">
            🏠 Zur Startseite
        </button>
    </div>
</body>
</html>`;

        this.displayContent(errorHtml);
        this.core.updateStatus(`❌ Fehler beim Laden von ${this.core.extractDomain(url)}`);
    }

    // History-Management
    addToHistory(url) {
        // Entferne zukünftige Einträge wenn wir in der Mitte der History sind
        if (this.historyIndex < this.history.length - 1) {
            this.history = this.history.slice(0, this.historyIndex + 1);
        }
        
        this.history.push(url);
        this.historyIndex = this.history.length - 1;
        
        console.log('📚 Added to history:', url, `(${this.history.length} entries)`);
    }

    // Navigation zurück
    goBack() {
        if (this.historyIndex > 0) {
            this.historyIndex--;
            const url = this.history[this.historyIndex];
            console.log('⬅️ Going back to:', url);
            this.navigateToUrl(url, true, false);
        }
    }

    // Navigation vorwärts
    goForward() {
        if (this.historyIndex < this.history.length - 1) {
            this.historyIndex++;
            const url = this.history[this.historyIndex];
            console.log('➡️ Going forward to:', url);
            this.navigateToUrl(url, true, false);
        }
    }

    // Seite neu laden
    reload() {
        const currentUrl = this.core.currentUrl;
        if (currentUrl && currentUrl !== 'about:blank') {
            console.log('🔄 Reloading:', currentUrl);
            this.navigateToUrl(currentUrl, true, false);
        }
    }

    // Link-Interception setup
    setupLinkInterception(container) {
        const targetContainer = container || document;
        
        const handleLinkClick = (e) => {
            const link = e.target.closest('a');
            if (link && link.href && !link.target) {
                const url = link.href;
                
                // Interne Navigation
                if (!url.startsWith('javascript:') && !url.startsWith('mailto:')) {
                    e.preventDefault();
                    console.log('🔗 Intercepted navigation:', url);
                    this.navigateToUrl(url);
                    return false;
                }
            }
        };

        targetContainer.addEventListener('click', handleLinkClick);
        console.log('🔗 Link interception setup complete');
    }

    // Google-Content optimieren
    optimizeGoogleContent() {
        const webviewFrame = document.getElementById('webview-frame');
        if (webviewFrame && webviewFrame.contentDocument) {
            try {
                const frameDoc = webviewFrame.contentDocument;
                
                // Google-spezifische Optimierungen
                const searchBox = frameDoc.querySelector('input[name="q"]');
                if (searchBox) {
                    searchBox.addEventListener('keydown', (e) => {
                        if (e.key === 'Enter') {
                            console.log('🔍 Google search:', searchBox.value);
                        }
                    });
                }
                
                console.log('🔍 Google content optimized');
            } catch (error) {
                console.log('⚠️ Could not optimize Google content (CORS)');
            }
        }
    }

    // GUI-Navigation behandeln
    async handleGUINavigation() {
        console.log('🖥️ Handling GUI navigation');
        
        const guiHtml = `
<!DOCTYPE html>
<html>
<head>
    <title>OraBrowser - GUI Interface</title>
    <style>
        body { 
            margin: 0; 
            padding: 40px; 
            font-family: Arial, sans-serif; 
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
        }
        .gui-title { font-size: 2.5rem; text-align: center; margin-bottom: 2rem; }
        .features { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .feature { 
            background: rgba(255,255,255,0.1); 
            padding: 20px; 
            border-radius: 10px; 
            text-align: center;
        }
        .feature h3 { margin-top: 0; }
    </style>
</head>
<body>
    <div class="gui-title">🌐 OraBrowser GUI</div>
    <div class="features">
        <div class="feature">
            <h3>📊 Navigation</h3>
            <p>Erweiterte URL-Navigation mit Proxy-Support</p>
        </div>
        <div class="feature">
            <h3>📚 Bookmarks</h3>
            <p>Lesezeichen-Verwaltung mit Backend-Synchronisation</p>
        </div>
        <div class="feature">
            <h3>🔧 Tabs</h3>
            <p>Multi-Tab-Browsing mit Drag & Drop</p>
        </div>
        <div class="feature">
            <h3>🔌 Plugins</h3>
            <p>Anti-Bot und weitere Plugin-Features</p>
        </div>
    </div>
</body>
</html>`;

        this.displayContent(guiHtml);
        return true;
    }
}

console.log('📦 Navigation module loaded'); 