/**
 * OraBrowser Utils Module
 * Helper-Funktionen und globale Utilities
 */

// Warte auf Tauri API
export async function waitForTauri(maxWait = 5000) {
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

// URL-Utilities
export const UrlUtils = {
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
        
        if (url.startsWith('https://') || url.startsWith('http://') || url.startsWith('about:')) {
            return url;
        }
        
        if (url.includes('.') && !url.includes(' ')) {
            return `https://${url}`;
        }
        
        return `https://www.google.com/search?q=${encodeURIComponent(url)}`;
    }
};

// DOM-Utilities
export const DomUtils = {
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
export const StorageUtils = {
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
export const EventUtils = {
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
export const ErrorHandler = {
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

// Debug-Tools
export const DebugTools = {
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
        }, 30000);
    }
};

console.log('📦 Utils module loaded'); 