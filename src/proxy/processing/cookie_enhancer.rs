// 🍪 Cookie Enhancement Module
// Injiziert Cookie-Skripte für bessere Kompatibilität

pub struct CookieEnhancer;

impl CookieEnhancer {
    pub fn new() -> Self {
        Self
    }

    /// Injiziere Cookie-Akzeptanz-Skript in HTML
    pub fn inject_cookie_script(&self, html: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let cookie_script = r#"
<script type="text/javascript">
// 🍪 COOKIE-BANNER-KOMPATIBILITÄT & POPUP-KONTROLLE
// Erweiterte Kompatibilität für moderne Websites

(function() {
    'use strict';
    
    console.log('🍪 Cookie-Banner-Kompatibilität wird aktiviert...');
    
    // 🚫 POPUP-BLOCKIERUNG (Zuerst ausführen!)
    function blockPopupsAndRedirects() {
        try {
            // Überschreibe window.open für Popup-Blockierung
            const originalOpen = window.open;
            window.open = function(url, name, features) {
                console.log('🚫 Popup blockiert:', url);
                return null;
            };
            
            // Überschreibe document.write für Redirect-Blockierung
            const originalWrite = document.write;
            document.write = function(content) {
                if (content.includes('location.href') || content.includes('window.open')) {
                    console.log('🚫 Redirect-Skript blockiert');
                    return;
                }
                return originalWrite.call(this, content);
            };
            
            // Event-Listener für unerwünschte Redirects
            window.addEventListener('beforeunload', function(event) {
                event.preventDefault();
                event.returnValue = '';
                console.log('🚫 Redirect blockiert');
            });
            
            console.log('✅ Popup-Blockierung aktiviert');
        } catch (e) {
            console.log('❌ Popup-Blockierung fehlgeschlagen:', e);
        }
    }
    
    // 🌐 DOMAIN-SPOOFING FÜR COOKIE-BANNER
    function setupDomainSpoofing() {
        try {
            // Extrahiere echte URL aus Proxy-URL
            const urlParams = new URLSearchParams(window.location.search);
            const realUrl = urlParams.get('url');
            
            if (realUrl) {
                const realUrlObj = new URL(realUrl);
                
                // Überschreibe window.location für Cookie-Banner
                const fakeLocation = {
                    href: realUrl,
                    hostname: realUrlObj.hostname,
                    host: realUrlObj.host,
                    origin: realUrlObj.origin,
                    protocol: realUrlObj.protocol,
                    pathname: realUrlObj.pathname,
                    search: realUrlObj.search,
                    hash: realUrlObj.hash,
                    port: realUrlObj.port,
                    toString: () => realUrl,
                    assign: (url) => {
                        console.log('🔄 Location.assign abgefangen:', url);
                        const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                        window.location.assign(proxyUrl);
                    },
                    replace: (url) => {
                        console.log('🔄 Location.replace abgefangen:', url);
                        const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
                        window.location.replace(proxyUrl);
                    },
                    reload: () => window.location.reload()
                };
                
                // Überschreibe document.domain
                try {
                    Object.defineProperty(document, 'domain', {
                        value: realUrlObj.hostname,
                        writable: false,
                        configurable: false
                    });
                } catch (e) {
                    console.log('⚠️ Could not override document.domain:', e);
                }
                
                console.log('✅ Domain spoofing setup:', realUrlObj.hostname);
            }
        } catch (e) {
            console.log('❌ Domain spoofing failed:', e);
        }
    }
    
    // 🔄 AJAX-REQUEST-PROXYING FÜR COOKIE-BANNER
    function setupAjaxProxying() {
        try {
            // Überschreibe XMLHttpRequest
            const originalXHR = window.XMLHttpRequest;
            window.XMLHttpRequest = function() {
                const xhr = new originalXHR();
                const originalOpen = xhr.open;
                
                xhr.open = function(method, url, async, user, password) {
                    // Wenn es ein relativer oder same-origin Request ist, proxye ihn
                    if (url.startsWith('/') || url.startsWith('./') || !url.includes('://')) {
                        const urlParams = new URLSearchParams(window.location.search);
                        const realUrl = urlParams.get('url');
                        if (realUrl) {
                            const realUrlObj = new URL(realUrl);
                            const fullUrl = new URL(url, realUrlObj.origin).href;
                            url = `http://localhost:3030/proxy?url=${encodeURIComponent(fullUrl)}`;
                        }
                    }
                    return originalOpen.call(this, method, url, async, user, password);
                };
                
                return xhr;
            };
            
            // Überschreibe fetch
            const originalFetch = window.fetch;
            window.fetch = function(input, init) {
                let url = typeof input === 'string' ? input : input.url;
                
                // Proxye relative URLs
                if (url.startsWith('/') || url.startsWith('./') || !url.includes('://')) {
                    const urlParams = new URLSearchParams(window.location.search);
                    const realUrl = urlParams.get('url');
                    if (realUrl) {
                        const realUrlObj = new URL(realUrl);
                        const fullUrl = new URL(url, realUrlObj.origin).href;
                        url = `http://localhost:3030/proxy?url=${encodeURIComponent(fullUrl)}`;
                    }
                }
                
                return originalFetch.call(this, url, init);
            };
            
            console.log('✅ AJAX proxying setup complete');
        } catch (e) {
            console.log('❌ AJAX proxying failed:', e);
        }
    }
    
    // 🍪 COOKIE-HANDLING-VERBESSERUNG
    function enhanceCookieHandling() {
        try {
            // Überschreibe document.cookie für bessere Kompatibilität
            const originalCookieDescriptor = Object.getOwnPropertyDescriptor(Document.prototype, 'cookie') || 
                                           Object.getOwnPropertyDescriptor(HTMLDocument.prototype, 'cookie');
            
            if (originalCookieDescriptor) {
                Object.defineProperty(document, 'cookie', {
                    get: function() {
                        return originalCookieDescriptor.get.call(this);
                    },
                    set: function(value) {
                        // Modifiziere Cookie-Domain für bessere Kompatibilität
                        const urlParams = new URLSearchParams(window.location.search);
                        const realUrl = urlParams.get('url');
                        if (realUrl) {
                            const realUrlObj = new URL(realUrl);
                            // Füge Domain hinzu wenn nicht vorhanden
                            if (!value.includes('domain=')) {
                                value += `; domain=${realUrlObj.hostname}`;
                            }
                        }
                        return originalCookieDescriptor.set.call(this, value);
                    },
                    configurable: true
                });
            }
            
            console.log('✅ Cookie handling enhanced');
        } catch (e) {
            console.log('❌ Cookie handling enhancement failed:', e);
        }
    }
    
    // 🚀 INITIALISIERUNG (Popup-Blockierung zuerst!)
    blockPopupsAndRedirects();
    setupDomainSpoofing();
    setupAjaxProxying();
    enhanceCookieHandling();
    
    // Warte auf DOM-Ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', () => {
            setTimeout(() => {
                blockPopupsAndRedirects();
                setupDomainSpoofing();
                setupAjaxProxying();
            }, 100);
        });
    }
    
    console.log('✅ Cookie-Banner-Kompatibilität + Popup-Kontrolle vollständig aktiviert!');
    
})();
</script>
"#;
        
        // Füge Script am Anfang des Head hinzu (sehr früh)
        let result = if let Some(head_start) = html.find("<head>") {
            let mut result = html.to_string();
            result.insert_str(head_start + 6, cookie_script);
            result
        } else if let Some(html_start) = html.find("<html>") {
            let mut result = html.to_string();
            result.insert_str(html_start + 6, &format!("<head>{}</head>", cookie_script));
            result
        } else {
            format!("{}{}", cookie_script, html)
        };
        
        Ok(result)
    }
}

impl Default for CookieEnhancer {
    fn default() -> Self {
        Self::new()
    }
} 
