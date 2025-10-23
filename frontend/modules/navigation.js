/**
 * ZAKYXBrowser Navigation Manager
 * Verantwortlich für URL-Navigation, Proxy-Handling und Content-Display
 */

class NavigationManager {
    constructor(core) {
        this.core = core;
        this.history = [];
        this.historyIndex = -1;
        this.linkInterceptionSetup = false;
        this.linkObserver = null;
        this.navigationHistory = [];
        
        console.log('🌐 Navigation Manager initialized');
    }

    // Hauptnavigationsfunktion
    async navigateToUrl(url, updateTab = true, addToHistoryFlag = true) {
        if (!url || url.trim() === '') {
            console.log('❌ Empty URL provided');
            return false;
        }
        
        this.resetNavigationState();
        
        const normalizedUrl = this.core.normalizeUrl(url.trim());
        console.log('🌐 Navigating to:', normalizedUrl);
        
        this.core.setLoading(true);
        this.core.updateStatus('Loading...');
        
        if (addToHistoryFlag) {
            this.addToHistory(normalizedUrl);
        }
        
        try {
            // Google-spezielle Behandlung
            if (normalizedUrl.includes('google.com') || normalizedUrl.includes('google.de')) {
                console.log('🔍 Google URL detected, using optimized navigation');
                return await this.handleGoogleNavigation(normalizedUrl);
            }
            
            // Proxy-Navigation versuchen
            const success = await this.tryProxyNavigation(normalizedUrl);
            if (success) {
                this.core.setLoading(false);
                return true;
            }
            
            // Fallback: Direkte iframe-Einbettung
            console.log('🔄 Using direct iframe navigation');
            this.createIframeFallback(normalizedUrl);
            this.core.setLoading(false);
            return true;
            
        } catch (error) {
            console.error('❌ Navigation failed:', error);
            this.showErrorPage(normalizedUrl, error);
            this.core.setLoading(false);
            return false;
        }
    }

    // Reset Navigation State
    resetNavigationState() {
        this.linkInterceptionSetup = false;
        if (this.linkObserver) {
            this.linkObserver.disconnect();
            this.linkObserver = null;
        }
    }

    // Proxy-Navigation versuchen
    async tryProxyNavigation(url) {
        try {
            console.log('🔍 Testing proxy server availability...');
            const proxyTest = await fetch('http://localhost:3030/health', { 
                method: 'GET',
                signal: AbortSignal.timeout(3000)
            });
            
            if (!proxyTest.ok) {
                console.log('🔍 Proxy server not available');
                return false;
            }
            
            const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
            console.log('🔄 Trying proxy navigation:', proxyUrl);
            
            const response = await fetch(proxyUrl, {
                method: 'GET',
                headers: {
                    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                    'Accept-Language': 'de-DE,de;q=0.9,en;q=0.8',
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
                }
            });
            
            if (response.ok) {
                const content = await response.text();
                console.log(`✅ Content loaded via proxy: ${content.length} bytes`);
                
                const optimizedContent = this.optimizeContent(content, url);
                this.displayContent(optimizedContent);
                return true;
            }
            
        } catch (error) {
            console.log('🔄 Proxy failed:', error.message);
        }
        
        return false;
    }

    // Google-Navigation speziell behandeln
    async handleGoogleNavigation(url) {
        console.log('🔍 Handling Google navigation for:', url);
        
        try {
            const success = await this.tryProxyNavigation(url);
            if (success) {
                return true;
            }
            
            // Fallback: Optimierter Google-Iframe
            console.log('🔄 Using optimized Google iframe');
            this.createOptimizedGoogleIframe(url);
            return true;
            
        } catch (error) {
            console.log('❌ Google navigation failed:', error);
            this.showErrorPage(url, error);
            return false;
        }
    }

    // Content optimieren
    optimizeContent(content, url) {
        console.log('🔧 Optimizing content for better compatibility');
        
        let optimized = content;
        
        // CSP-Beschränkungen entfernen
        optimized = optimized.replace(/<meta[^>]*http-equiv=["']Content-Security-Policy["'][^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*http-equiv=["']X-Frame-Options["'][^>]*>/gi, '');
        
        // Meta-Tags hinzufügen
        const meta = `
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-Frame-Options" content="ALLOWALL">
            <base href="${url}">
        `;
        
        if (optimized.includes('<head>')) {
            optimized = optimized.replace('<head>', '<head>' + meta);
        }
        
        // Security-Script einfügen
        const securityScript = this.createSecurityScript();
        if (optimized.includes('</body>')) {
            optimized = optimized.replace('</body>', securityScript + '</body>');
        } else {
            optimized += securityScript;
        }
        
        return optimized;
    }

    // Security-Script erstellen
    createSecurityScript() {
        return `
<script>
        // ZAKYXBrowser Security Enhancement
(function() {
    'use strict';
    
            console.log('🛡️ ZAKYXBrowser Security Script loaded');
    
    // Aktiviere alle Eingabefelder
    function enableInteractions() {
        const style = document.createElement('style');
        style.textContent = \`
            *, *::before, *::after { 
                pointer-events: auto !important; 
                cursor: auto !important;
            }
            button, input, select, textarea, a { 
                pointer-events: auto !important; 
                cursor: pointer !important; 
                opacity: 1 !important;
            }
        \`;
        document.head.appendChild(style);
        
        // Aktiviere alle Elemente
        document.querySelectorAll('*').forEach(element => {
            element.removeAttribute('disabled');
            element.removeAttribute('readonly');
            if (element.style) {
                element.style.pointerEvents = 'auto';
                element.style.cursor = 'auto';
            }
        });
    }
    
    // Link-Interception
    function setupLinkInterception() {
        document.addEventListener('click', function(e) {
            const link = e.target.closest('a[href]');
            if (!link) return;
            
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !href.startsWith('javascript:')) {
                e.preventDefault();
                
                let fullUrl = href;
                if (href.startsWith('/')) {
                    fullUrl = window.location.origin + href;
                } else if (!href.startsWith('http')) {
                    fullUrl = new URL(href, window.location.href).href;
                }
                
                // Sende an Parent Browser
                        if (window.parent && window.parent.zakyxBrowser) {
            window.parent.zakyxBrowser.navigateToUrl(fullUrl);
                } else {
                    window.parent.postMessage({
                        type: 'navigate',
                        url: fullUrl
                    }, '*');
                }
            }
        }, true);
    }
    
    // Initialisierung
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', function() {
            enableInteractions();
            setupLinkInterception();
        });
    } else {
        enableInteractions();
        setupLinkInterception();
    }
    
    // Wiederhole für dynamische Inhalte
    setInterval(enableInteractions, 1000);
    
            console.log('✅ ZAKYXBrowser Security Script initialized');
})();
</script>
        `;
    }

    // Content anzeigen
    displayContent(htmlContent) {
        console.log('📄 Displaying content...');
        
        if (!htmlContent || !htmlContent.trim()) {
            console.error('❌ HTML content is empty!');
            return false;
        }
        
        // Container finden
        let targetContainer = document.getElementById('webview-container') ||
                            document.getElementById('content-area') ||
                            document.getElementById('content-container');
        
        if (!targetContainer) {
            console.error('❌ No suitable container found!');
            return false;
        }
        
        // Welcome Screen entfernen
        this.hideWelcomeScreen();
        
        // Container vorbereiten
        targetContainer.innerHTML = '';
        targetContainer.style.cssText = `
            position: absolute !important;
            top: 0 !important;
            left: 0 !important;
            width: 100% !important;
            height: 100% !important;
            overflow: auto !important;
            background: white !important;
            z-index: 1000 !important;
            display: block !important;
            visibility: visible !important;
        `;
        
        // Content einfügen
        targetContainer.innerHTML = htmlContent;
        
        // Link-Interception einrichten
        setTimeout(() => {
            this.setupLinkInterception(targetContainer);
        }, 100);
        
        console.log('✅ Content displayed successfully');
        return true;
    }

    // Welcome Screen verstecken
    hideWelcomeScreen() {
        const welcomeElements = [
            'welcome-screen',
            'welcome-container',
            'start-screen'
        ];
        
        welcomeElements.forEach(id => {
            const element = document.getElementById(id);
            if (element) {
                element.remove();
            }
        });
    }

    // Iframe-Fallback erstellen
    createIframeFallback(url) {
        const fallbackContent = `
<!DOCTYPE html>
<html>
<head>
            <title>ZAKYXBrowser - ${this.core.extractDomain(url)}</title>
    <style>
        body { margin: 0; padding: 0; overflow: hidden; }
        iframe { width: 100%; height: 100vh; border: none; }
        .header { background: #667eea; color: white; padding: 8px; font-size: 12px; }
    </style>
</head>
<body>
            <div class="header">🌐 ${url} via ZAKYXBrowser</div>
    <iframe src="${url}" sandbox="allow-same-origin allow-scripts allow-forms allow-popups"></iframe>
</body>
</html>
        `;
        
        this.displayContent(fallbackContent);
        this.addToHistory(url);
        this.core.updateStatus(`Iframe: ${url}`);
    }

    // Google-Iframe optimiert
    createOptimizedGoogleIframe(url) {
        const googleContent = `
<!DOCTYPE html>
<html>
<head>
            <title>Google - ZAKYXBrowser</title>
    <style>
        body { margin: 0; padding: 0; overflow: hidden; }
        iframe { width: 100%; height: 100vh; border: none; }
        .google-header { background: #4285f4; color: white; padding: 8px; font-size: 12px; }
    </style>
</head>
<body>
            <div class="google-header">🔍 Google Search - ZAKYXBrowser</div>
    <iframe src="${url}" sandbox="allow-same-origin allow-scripts allow-forms allow-popups"></iframe>
</body>
</html>
        `;
        
        this.displayContent(googleContent);
    }

    // Error Page anzeigen
    showErrorPage(url, error) {
        const errorContent = `
<!DOCTYPE html>
<html>
<head>
            <title>ZAKYXBrowser - Fehler</title>
    <style>
        body { font-family: Arial; padding: 40px; background: #f5f5f5; }
        .error-container { background: white; padding: 30px; border-radius: 10px; text-align: center; }
        .error-icon { font-size: 48px; margin-bottom: 20px; }
        .retry-btn { background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 6px; cursor: pointer; margin: 10px; }
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-icon">🌐❌</div>
        <h2>Verbindungsfehler</h2>
        <p>Die Website konnte nicht geladen werden:</p>
        <div style="background: #f0f0f0; padding: 10px; margin: 20px 0; word-break: break-all;">${url}</div>
        <p>Fehler: ${error.message || error}</p>
        <button class="retry-btn" onclick="parent.zakyxBrowser.navigateToUrl('${url}')">🔄 Erneut versuchen</button>
        <button class="retry-btn" onclick="window.open('${url}', '_blank')">🌐 Extern öffnen</button>
    </div>
</body>
</html>
        `;
        
        this.displayContent(errorContent);
    }

    // History Management
    addToHistory(url) {
        this.history = this.history.slice(0, this.historyIndex + 1);
        
        if (this.history[this.history.length - 1] !== url) {
            this.history.push(url);
            this.historyIndex = this.history.length - 1;
        }
        
        if (this.history.length > 100) {
            this.history = this.history.slice(-100);
            this.historyIndex = this.history.length - 1;
        }
        
        console.log(`📚 Added to history: ${url}`);
    }

    // Navigation Controls
    goBack() {
        if (this.history.length > 1 && this.historyIndex > 0) {
            this.historyIndex--;
            const url = this.history[this.historyIndex];
            this.navigateToUrl(url, true, false);
        }
    }

    goForward() {
        if (this.historyIndex < this.history.length - 1) {
            this.historyIndex++;
            const url = this.history[this.historyIndex];
            this.navigateToUrl(url, true, false);
        }
    }

    reload() {
        if (this.core.currentUrl && this.core.currentUrl !== 'about:blank') {
            this.navigateToUrl(this.core.currentUrl);
        }
    }

    // Link-Interception einrichten
    setupLinkInterception(container) {
        if (this.linkInterceptionSetup) return;
        
        const handleLinkClick = (e) => {
            const link = e.target.closest('a[href]');
            if (!link) return;
            
            const href = link.getAttribute('href');
            if (href && !href.startsWith('#') && !href.startsWith('javascript:')) {
                e.preventDefault();
                e.stopPropagation();
                
                let fullUrl = href;
                if (href.startsWith('/')) {
                    fullUrl = window.location.origin + href;
                } else if (!href.startsWith('http')) {
                    fullUrl = new URL(href, window.location.href).href;
                }
                
                this.navigateToUrl(fullUrl);
            }
        };
        
        container.addEventListener('click', handleLinkClick, true);
        this.linkInterceptionSetup = true;
        
        console.log('✅ Link interception setup complete');
    }
}

// Global verfügbar machen
window.NavigationManager = NavigationManager;

export default NavigationManager; 