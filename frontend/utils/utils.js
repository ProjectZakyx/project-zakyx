/**
 * ZAKYXBrowser Utils
 * Helper-Funktionen und globale Utilities
 */

// Warte auf Tauri API
async function waitForTauri(maxWait = 5000) {
    const startTime = Date.now();
    
    while (Date.now() - startTime < maxWait) {
        if (window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke) {
            console.log('✅ Tauri API verfügbar');
            return true;
        }
        
        await new Promise(resolve => setTimeout(resolve, 100));
    }
    
    console.log('⚠️ Tauri API Timeout nach', maxWait, 'ms');
    return false;
}

// Debug-Tools
const DebugTools = {
    // Element-Debugging
    checkElements() {
        const elements = [
            'content-area',
            'webview-container',
            'content-container',
            'tabs-container',
            'bookmarks-container',
            'address-input',
            'bookmark-btn',
            'new-tab-btn'
        ];
        
        console.log('🔧 === ELEMENT CHECK ===');
        elements.forEach(id => {
            const element = document.getElementById(id);
            console.log(`🔧 ${id}:`, element ? 'EXISTS' : 'MISSING');
            if (element) {
                console.log(`   - Display: ${element.style.display || 'default'}`);
                console.log(`   - Visibility: ${element.style.visibility || 'default'}`);
                console.log(`   - Dimensions: ${element.offsetWidth}x${element.offsetHeight}`);
            }
        });
        console.log('🔧 === END ELEMENT CHECK ===');
    },
    
    // Performance-Monitoring
    startPerformanceMonitor() {
        if (performance.mark) {
            performance.mark('zakyx-browser-start');
        }
        
        setInterval(() => {
            const memInfo = performance.memory;
            if (memInfo) {
                console.log('📊 Memory:', {
                    used: Math.round(memInfo.usedJSHeapSize / 1024 / 1024) + 'MB',
                    total: Math.round(memInfo.totalJSHeapSize / 1024 / 1024) + 'MB',
                    limit: Math.round(memInfo.jsHeapSizeLimit / 1024 / 1024) + 'MB'
                });
            }
        }, 30000); // Alle 30 Sekunden
    },
    
    // Test-Content anzeigen
    showTestContent() {
        const testContent = `
<!DOCTYPE html>
<html>
        <head><title>ZAKYXBrowser Test</title></head>
<body style="font-family: Arial; padding: 20px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; min-height: 100vh;">
    <div style="text-align: center; padding: 40px;">
        <h1>🎉 ZAKYXBrowser Test erfolgreich!</h1>
        <p>Diese Seite zeigt, dass das modulare System funktioniert.</p>
        <div style="background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; margin: 20px 0;">
            <h3>✅ Module geladen:</h3>
            <ul style="text-align: left; display: inline-block;">
                <li>Core Module</li>
                <li>Navigation Manager</li>
                <li>Tab Manager</li>
                <li>Bookmark Manager</li>
                <li>Utils</li>
            </ul>
        </div>
        <p>Zeit: ${new Date().toLocaleString()}</p>
    </div>
</body>
</html>
        `;
        
        // Zeige Test-Content
        if (window.zakyxBrowser && window.zakyxBrowser.navigationManager) {
            window.zakyxBrowser.navigationManager.displayContent(testContent);
        } else {
            console.error('❌ ZAKYXBrowser instance not available');
        }
    }
};

// URL-Utilities
const UrlUtils = {
    // Prüfe ob URL gültig ist
    isValidUrl(string) {
        try {
            new URL(string);
            return true;
        } catch (_) {
            return false;
        }
    },
    
    // Extrahiere Domain
    extractDomain(url) {
        try {
            const urlObj = new URL(url);
            return urlObj.hostname.replace('www.', '');
        } catch (error) {
            const match = url.match(/^(?:https?:\/\/)?(?:www\.)?([^\/]+)/);
            return match ? match[1] : url;
        }
    },
    
    // Erstelle sichere URL
    makeSafeUrl(url) {
        if (!url) return 'about:blank';
        
        // Bereits sichere URLs
        if (url.startsWith('https://') || url.startsWith('http://') || url.startsWith('about:')) {
            return url;
        }
        
        // Domain ohne Protokoll
        if (url.includes('.') && !url.includes(' ')) {
            return `https://${url}`;
        }
        
        // Suchbegriff
        return `https://www.google.com/search?q=${encodeURIComponent(url)}`;
    }
};

// DOM-Utilities
const DomUtils = {
    // Element sicher finden
    safeGetElement(id) {
        const element = document.getElementById(id);
        if (!element) {
            console.warn(`⚠️ Element not found: ${id}`);
        }
        return element;
    },
    
    // Element mit Retry finden
    async waitForElement(id, maxWait = 5000) {
        const startTime = Date.now();
        
        while (Date.now() - startTime < maxWait) {
            const element = document.getElementById(id);
            if (element) {
                return element;
            }
            await new Promise(resolve => setTimeout(resolve, 100));
        }
        
        console.warn(`⚠️ Element timeout: ${id}`);
        return null;
    },
    
    // Alle Event-Listener entfernen
    removeAllEventListeners(element) {
        const newElement = element.cloneNode(true);
        element.parentNode.replaceChild(newElement, element);
        return newElement;
    }
};

// Storage-Utilities
const StorageUtils = {
    // Sichere localStorage-Operationen
    get(key, defaultValue = null) {
        try {
            const value = localStorage.getItem(key);
            return value ? JSON.parse(value) : defaultValue;
        } catch (error) {
            console.error(`Storage get error for ${key}:`, error);
            return defaultValue;
        }
    },
    
    set(key, value) {
        try {
            localStorage.setItem(key, JSON.stringify(value));
            return true;
        } catch (error) {
            console.error(`Storage set error for ${key}:`, error);
            return false;
        }
    },
    
    remove(key) {
        try {
            localStorage.removeItem(key);
            return true;
        } catch (error) {
            console.error(`Storage remove error for ${key}:`, error);
            return false;
        }
    }
};

// Event-Utilities
const EventUtils = {
    // Sichere Event-Listener
    addListener(element, event, handler, options = {}) {
        if (!element) {
            console.warn('⚠️ Cannot add listener to null element');
            return false;
        }
        
        try {
            element.addEventListener(event, handler, options);
            return true;
        } catch (error) {
            console.error('Event listener error:', error);
            return false;
        }
    },
    
    // Event mit Timeout
    addTimeoutListener(element, event, handler, timeout = 5000) {
        let timeoutId;
        
        const wrappedHandler = (e) => {
            clearTimeout(timeoutId);
            handler(e);
        };
        
        this.addListener(element, event, wrappedHandler);
        
        timeoutId = setTimeout(() => {
            console.warn(`⚠️ Event timeout for ${event}`);
        }, timeout);
        
        return timeoutId;
    }
};

// Error-Handling
const ErrorHandler = {
    // Globaler Error-Handler
    setupGlobalErrorHandling() {
        window.addEventListener('error', (e) => {
            console.error('🚨 Global error:', e.error);
            console.error('🚨 Message:', e.message);
            console.error('🚨 Filename:', e.filename);
            console.error('🚨 Line:', e.lineno);
        });
        
        window.addEventListener('unhandledrejection', (e) => {
            console.error('🚨 Unhandled promise rejection:', e.reason);
        });
        
        console.log('✅ Global error handling setup complete');
    },
    
    // Safe function execution
    safeExecute(fn, fallback = null) {
        try {
            return fn();
        } catch (error) {
            console.error('Safe execute error:', error);
            return fallback;
        }
    },
    
    // Async safe execution
    async safeExecuteAsync(fn, fallback = null) {
        try {
            return await fn();
        } catch (error) {
            console.error('Async safe execute error:', error);
            return fallback;
        }
    }
};

// Global verfügbar machen
window.waitForTauri = waitForTauri;
window.DebugTools = DebugTools;
window.UrlUtils = UrlUtils;
window.DomUtils = DomUtils;
window.StorageUtils = StorageUtils;
window.EventUtils = EventUtils;
window.ErrorHandler = ErrorHandler;

// Auto-Setup
document.addEventListener('DOMContentLoaded', () => {
    ErrorHandler.setupGlobalErrorHandling();
    DebugTools.startPerformanceMonitor();
            console.log('✅ ZAKYXBrowser Utils loaded');
});

export { 
    waitForTauri, 
    DebugTools, 
    UrlUtils, 
    DomUtils, 
    StorageUtils, 
    EventUtils, 
    ErrorHandler 
}; 