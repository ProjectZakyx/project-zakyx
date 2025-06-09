// 🌐 ORA BROWSER - TAURI FRONTEND APPLICATION
// Modern cross-platform browser frontend with Tauri integration

// 🚨 TAURI VERFÜGBARKEITSPRÜFUNG
let invoke, listen, appWindow;

if (typeof window.__TAURI__ !== 'undefined') {
    console.log('✅ Tauri API available');
    console.log('🔍 Tauri structure:', window.__TAURI__);
    
    // Check different possible API structures
    if (window.__TAURI__.tauri && window.__TAURI__.tauri.invoke) {
        console.log('🔧 Using __TAURI__.tauri.invoke');
        invoke = window.__TAURI__.tauri.invoke;
    } else if (window.__TAURI__.invoke) {
        console.log('🔧 Using __TAURI__.invoke');
        invoke = window.__TAURI__.invoke;
    } else {
        console.log('❌ invoke not found in Tauri API');
        invoke = null;
    }
    
    if (window.__TAURI__.event && window.__TAURI__.event.listen) {
        listen = window.__TAURI__.event.listen;
    } else if (window.__TAURI__.listen) {
        listen = window.__TAURI__.listen;
    } else {
        console.log('❌ listen not found in Tauri API');
        listen = null;
    }
    
    if (window.__TAURI__.window && window.__TAURI__.window.appWindow) {
        appWindow = window.__TAURI__.window.appWindow;
    } else {
        appWindow = null;
    }
    
    // Fallback wenn Tauri-APIs nicht verfügbar sind
    if (!invoke) {
        console.log('⚠️ Using fallback invoke function');
        invoke = async (command, args) => {
            console.log(`🔄 FALLBACK invoke: ${command}`, args);
            
            if (command === 'open_external_url') {
                console.log(`🚀 ÖFFNE BROWSER: ${args.url}`);
                try {
                    window.open(args.url, '_blank');
                    return Promise.resolve({});
                } catch (e) {
                    console.error('❌ Fallback window.open auch fehlgeschlagen:', e);
                    return Promise.reject('Fallback failed');
                }
            } else if (command === 'create_new_tab') {
                console.log(`🚀 ERSTELLE TAB: ${args ? args.url : 'Neuer Tab'}`);
                return Promise.resolve({
                    id: 'tab-' + Date.now(),
                    title: 'New Tab',
                    url: args ? args.url : 'about:blank',
                    favicon: null,
                    is_active: true,
                    is_loading: false
                });
            } else if (command === 'get_bookmarks') {
                return Promise.resolve([]);
            } else if (command === 'get_settings') {
                return Promise.resolve({
                    homepage: 'https://www.google.com',
                    search_engine: 'https://www.google.com/search?q=',
                    privacy_mode: false,
                    ad_blocker: true,
                    javascript_enabled: true,
                    cookies_enabled: true
                });
            }
            return Promise.resolve({});
        };
    }
    
    if (!listen) {
        listen = (event, callback) => {
            console.log(`🔄 FALLBACK listen: ${event}`);
            return Promise.resolve();
        };
    }
} else {
    console.error('❌ Tauri API not available - using fallback functions');
    
    // Fallback functions when __TAURI__ is completely missing
    invoke = async (command, args) => {
        console.log(`🔄 FALLBACK invoke: ${command}`, args);
        
        if (command === 'open_external_url') {
            try {
                window.open(args.url, '_blank');
                return Promise.resolve({});
            } catch (e) {
                console.error('❌ Fallback window.open auch fehlgeschlagen:', e);
                return Promise.reject('Fallback failed');
            }
        } else if (command === 'get_bookmarks') {
            return Promise.resolve([]);
        } else if (command === 'get_settings') {
            return Promise.resolve({
                homepage: 'https://www.google.com',
                search_engine: 'https://www.google.com/search?q=',
                privacy_mode: false,
                ad_blocker: true,
                javascript_enabled: true,
                cookies_enabled: true
            });
        } else if (command === 'create_new_tab') {
            return Promise.resolve({
                id: 'tab-' + Date.now(),
                title: 'New Tab',
                url: args ? args.url : 'about:blank',
                favicon: null,
                is_active: true,
                is_loading: false
            });
        } else if (command === 'add_bookmark') {
            return Promise.resolve({
                id: 'bookmark-' + Date.now(),
                title: args.title,
                url: args.url
            });
        } else if (command === 'update_settings') {
            return Promise.resolve({});
        }
        return Promise.resolve({});
    };
    
    listen = (event, callback) => {
        console.log(`🔄 FALLBACK listen: ${event}`);
        return Promise.resolve();
    };
    
    appWindow = {
        emit: (event, data) => {
            console.log(`🔄 FALLBACK emit: ${event}`, data);
            return Promise.resolve();
        }
    };
}

// 🚨 ULTIMATE DEBUG LOGGING
function debugLog(message, data = null) {
    console.log(`🐛 DEBUG: ${message}`, data || '');
    
    // Show debug info in status bar
    const statusText = document.getElementById('status-text');
    if (statusText) {
        statusText.textContent = `DEBUG: ${message}`;
        statusText.style.color = '#ff6b6b';
    }
}

// 🚨 ALERT DEBUGGING (shows what's happening step by step)
function debugAlert(message) {
    console.log(`🚨 ALERT DEBUG: ${message}`);
    // alert(`🐛 DEBUG SCHRITT: ${message}`); // Deactivated - working fine now!
}

// 🗂️ APPLICATION STATE
class BrowserApp {
    constructor() {
        this.tabs = [];
        this.activeTabId = null;
        this.bookmarks = [];
        this.settings = null;
        this.history = [];
        
        this.init();
    }
    
    async init() {
        console.log('🚀 Initializing Ora Browser...');
        
        // Load initial data
        await this.loadBookmarks();
        await this.loadSettings();
        
        // Setup event listeners
        this.setupEventListeners();
        
        // Create first tab
        await this.createNewTab('https://www.google.com');
        
        // Update UI
        this.renderTabs();
        this.renderBookmarks();
        
        console.log('✅ Ora Browser initialized successfully!');
    }
    
    // 📑 TAB MANAGEMENT
    async createNewTab(url = null) {
        try {
            const tab = await invoke('create_new_tab', { url });
            this.tabs.push(tab);
            this.activeTabId = tab.id;
            this.renderTabs();
            this.updateAddressBar(tab.url);
            this.hideWelcomeScreen();
            return tab;
        } catch (error) {
            console.error('Failed to create tab:', error);
        }
    }
    
    async closeTab(tabId) {
        try {
            await invoke('close_tab', { tabId });
            this.tabs = this.tabs.filter(tab => tab.id !== tabId);
            
            if (this.tabs.length === 0) {
                this.showWelcomeScreen();
                this.activeTabId = null;
            } else if (this.activeTabId === tabId) {
                this.activeTabId = this.tabs[0].id;
                this.switchToTab(this.activeTabId);
            }
            
            this.renderTabs();
        } catch (error) {
            console.error('Failed to close tab:', error);
        }
    }
    
    switchToTab(tabId) {
        this.activeTabId = tabId;
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab) {
            this.updateAddressBar(tab.url);
            this.renderTabs();
        }
    }
    
    async navigateToUrl(url) {
        console.log('🌐 Navigating to:', url);
        
        // NORMALIZE URL FIRST (before any URL parsing)
        if (!url.startsWith('http')) {
            if (url.includes('.')) {
                url = 'https://' + url;
            } else {
                url = `https://www.google.com/search?q=${encodeURIComponent(url)}`;
            }
        }
        
        console.log('🔧 Normalized URL:', url);
        
        // Create new tab if none exists
        if (!this.activeTabId) {
            await this.createNewTab(url);
            return;
        }
        
        try {
            // CLEAN UP: Hide any previous content first
            this.hideWelcomeScreen();
            document.getElementById('webview-container').style.display = 'none';
            
            // Update tab state immediately
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                tab.url = url;
                tab.title = 'Loading...';
                tab.is_loading = true;
            }
            
            // Update UI immediately to prevent duplicates
            this.renderTabs();
            this.updateAddressBar(url);
            
            // Update status
            document.getElementById('status-text').textContent = `Opening ${url}...`;
            
            // Smart navigation: check iframe compatibility with detailed debugging
            const iframeFriendly = this.isIframeFriendly(url);
            console.log(`🔍 DEBUGGING URL: ${url}`);
            console.log(`🔍 DOMAIN: ${new URL(url).hostname.toLowerCase()}`);
            console.log(`🔍 IFRAME-FRIENDLY: ${iframeFriendly}`);
            
            if (iframeFriendly) {
                console.log('✅ DECISION: Loading internally in iframe');
                this.showWebViewContent(url);
                if (tab) {
                    tab.title = this.getTitleFromUrl(url);
                    tab.is_loading = false;
                    this.renderTabs();
                }
                document.getElementById('status-text').textContent = `Loaded ${url} internally`;
            } else {
                console.log('🌐 DECISION: Opening externally in system browser');
                await this.fallbackToExternalBrowser(url);
                // Reset tab state for external navigation
                if (tab) {
                    tab.title = `External: ${this.getTitleFromUrl(url)}`;
                    tab.is_loading = false;
                    this.renderTabs();
                }
            }
            
        } catch (error) {
            console.error('Navigation failed:', error);
            document.getElementById('status-text').textContent = `Failed to load ${url}`;
            
            // Reset tab state on error
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                tab.title = 'Error';
                tab.is_loading = false;
                this.renderTabs();
            }
        }
    }
    
    async openInNewWindow(url) {
        // DEPRECATED: Don't create new Tauri windows, use system browser instead
        console.log('🚫 Redirecting new window request to system browser:', url);
        await this.fallbackToExternalBrowser(url);
    }
    
    async fallbackToExternalBrowser(url) {
        try {
            await invoke('open_external_url', { url });
            document.getElementById('status-text').textContent = 'Opened in system browser';
            
            // Update tab title
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                tab.title = `External: ${this.getTitleFromUrl(url)}`;
                this.renderTabs();
            }
        } catch (error) {
            console.error('❌ Failed to open external URL:', error);
            document.getElementById('status-text').textContent = 'Navigation failed';
        }
    }
    
    // WebView container nicht mehr benötigt - verwenden echte Tauri Windows
    
    getTitleFromUrl(url) {
        try {
            const domain = new URL(url).hostname;
            return domain.replace('www.', '').split('.')[0];
        } catch {
            return 'New Tab';
        }
    }
    
    // 🔍 SMART NAVIGATION - Check if URL should be loaded internally or externally
    isIframeFriendly(url) {
        // 🚫 KNOWN IFRAME-BLOCKING DOMAINS (X-Frame-Options: DENY/SAMEORIGIN)
        const blockedDomains = [
            'google.com', 'www.google.com', 'google.de', 'www.google.de',
            'facebook.com', 'www.facebook.com',
            'youtube.com', 'www.youtube.com',
            'github.com', 'www.github.com',
            'twitter.com', 'www.twitter.com', 'x.com', 'www.x.com',
            'instagram.com', 'www.instagram.com',
            'linkedin.com', 'www.linkedin.com',
            'amazon.com', 'www.amazon.com', 'amazon.de', 'www.amazon.de',
            'ebay.com', 'www.ebay.com', 'ebay.de', 'www.ebay.de',
            'paypal.com', 'www.paypal.com',
            'microsoft.com', 'www.microsoft.com',
            'apple.com', 'www.apple.com',
            'wikipedia.org', 'www.wikipedia.org', 'de.wikipedia.org',
            'yahoo.com', 'www.yahoo.com', 'yahoo.de', 'www.yahoo.de',
            'bing.com', 'www.bing.com',
            'reddit.com', 'www.reddit.com',
            'stackoverflow.com', 'www.stackoverflow.com',
            'netflix.com', 'www.netflix.com',
            'twitch.tv', 'www.twitch.tv'
        ];
        
        // ✅ IFRAME-FRIENDLY DOMAINS (usually allow embedding)
        const friendlyDomains = [
            'jsonplaceholder.typicode.com',
            'httpbin.org',
            'example.com', 'www.example.com',
            'codepen.io',
            'jsfiddle.net',
            'codesandbox.io'
        ];
        
        try {
            const domain = new URL(url).hostname.toLowerCase();
            
            // Check if explicitly friendly
            if (friendlyDomains.some(friendly => domain === friendly || domain.endsWith('.' + friendly))) {
                return true;
            }
            
            // Check if explicitly blocked
            if (blockedDomains.some(blocked => domain === blocked || domain.endsWith('.' + blocked))) {
                return false;
            }
            
            // Default to false for unknown domains (safer approach)
            return false;
        } catch (e) {
            console.warn('🚨 Invalid URL for iframe check:', url);
            return false;
        }
    }
    
    // 📚 BOOKMARK MANAGEMENT
    async loadBookmarks() {
        try {
            this.bookmarks = await invoke('get_bookmarks');
            // Ensure bookmarks is always an array
            if (!Array.isArray(this.bookmarks)) {
                this.bookmarks = [];
            }
        } catch (error) {
            console.error('Failed to load bookmarks:', error);
            this.bookmarks = []; // Fallback to empty array
        }
    }
    
    async addBookmark(title, url) {
        try {
            const bookmark = await invoke('add_bookmark', { title, url });
            this.bookmarks.push(bookmark);
            this.renderBookmarks();
            return bookmark;
        } catch (error) {
            console.error('Failed to add bookmark:', error);
        }
    }
    
    async removeBookmark(bookmarkId) {
        try {
            await invoke('remove_bookmark', { bookmarkId });
            this.bookmarks = this.bookmarks.filter(b => b.id !== bookmarkId);
            this.renderBookmarks();
        } catch (error) {
            console.error('Failed to remove bookmark:', error);
        }
    }
    
    // ⚙️ SETTINGS MANAGEMENT
    async loadSettings() {
        try {
            this.settings = await invoke('get_settings');
        } catch (error) {
            console.error('Failed to load settings:', error);
            this.settings = {
                homepage: 'https://www.google.com',
                search_engine: 'https://www.google.com/search?q=',
                privacy_mode: false,
                ad_blocker: true,
                javascript_enabled: true,
                cookies_enabled: true
            };
        }
    }
    
    async updateSettings(newSettings) {
        try {
            await invoke('update_settings', { newSettings });
            this.settings = newSettings;
        } catch (error) {
            console.error('Failed to update settings:', error);
        }
    }
    
    // 🎨 UI RENDERING
    renderTabs() {
        const container = document.getElementById('tabs-container');
        container.innerHTML = '';
        
        this.tabs.forEach(tab => {
            const tabEl = document.createElement('div');
            tabEl.className = `tab ${tab.id === this.activeTabId ? 'active' : ''}`;
            tabEl.innerHTML = `
                <div class="tab-favicon"></div>
                <span class="tab-title">${tab.title}</span>
                <button class="tab-close">×</button>
            `;
            
            // Tab click to switch
            tabEl.addEventListener('click', (e) => {
                if (e.target.classList.contains('tab-close')) {
                    return; // Don't switch tab when closing
                }
                this.activeTabId = tab.id;
                this.renderTabs();
                this.updateAddressBar(tab.url);
            });
            
            // Tab close button
            const closeBtn = tabEl.querySelector('.tab-close');
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.closeTab(tab.id);
            });
            
            container.appendChild(tabEl);
        });
    }
    
    renderBookmarks() {
        const container = document.getElementById('bookmarks-container');
        container.innerHTML = '';
        
        // Safety check - ensure bookmarks is an array
        if (!Array.isArray(this.bookmarks)) {
            this.bookmarks = [];
        }
        
        this.bookmarks.forEach(bookmark => {
            const bookmarkEl = document.createElement('a');
            bookmarkEl.className = 'bookmark';
            bookmarkEl.href = '#';
            bookmarkEl.textContent = bookmark.title;
            
            bookmarkEl.addEventListener('click', (e) => {
                e.preventDefault();
                this.navigateToUrl(bookmark.url);
            });
            
            container.appendChild(bookmarkEl);
        });
    }
    
    updateAddressBar(url) {
        document.getElementById('address-input').value = url;
    }
    
    showWelcomeScreen() {
        document.getElementById('welcome-screen').style.display = 'flex';
        document.getElementById('webview-container').style.display = 'none';
    }
    
    hideWelcomeScreen() {
        document.getElementById('welcome-screen').style.display = 'none';
    }
    
    showWebViewContent(url) {
        console.log(`🎯 showWebViewContent called with: ${url}`);
        
        // Hide welcome screen
        this.hideWelcomeScreen();
        console.log(`🎯 Welcome screen hidden`);
        
        // Show webview container
        const webviewContainer = document.getElementById('webview-container');
        const webviewFrame = document.getElementById('webview-frame');
        
        console.log(`🎯 Elements found: container=${!!webviewContainer}, frame=${!!webviewFrame}`);
        
        if (webviewContainer && webviewFrame) {
            webviewContainer.style.display = 'block';
            webviewFrame.src = url;
            
            console.log(`✅ IFRAME SHOULD LOAD: ${url} in internal iframe`);
            console.log(`✅ Container display: ${webviewContainer.style.display}`);
            console.log(`✅ Frame src: ${webviewFrame.src}`);
            
            // Update current tab
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                tab.url = url;
                tab.title = this.getTitleFromUrl(url);
                this.renderTabs();
                console.log(`✅ Tab updated: ${tab.title}`);
            }
        } else {
            console.error(`❌ Missing elements: container=${!!webviewContainer}, frame=${!!webviewFrame}`);
        }
    }
    
    // 🎯 EVENT LISTENERS
    setupEventListeners() {
        console.log('🔧 Setting up event listeners...');
        
        // Address bar
        const addressInput = document.getElementById('address-input');
        const goBtn = document.getElementById('go-btn');
        
        console.log('🔍 Found elements:', {
            addressInput: !!addressInput,
            goBtn: !!goBtn,
            menuBtn: !!document.getElementById('menu-btn'),
            bookmarkBtn: !!document.getElementById('bookmark-btn')
        });
        
        const navigate = () => {
            const url = addressInput.value.trim();
            if (url) {
                this.navigateToUrl(url);
            }
        };
        
        goBtn.addEventListener('click', navigate);
        addressInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') navigate();
        });
        
        // New tab button
        document.getElementById('new-tab-btn').addEventListener('click', () => {
            this.createNewTab();
        });
        
        // Bookmark button
        document.getElementById('bookmark-btn').addEventListener('click', () => {
            this.showBookmarkModal();
        });
        
        // Menu button  
        const menuBtn = document.getElementById('menu-btn');
        if (menuBtn) {
            menuBtn.addEventListener('click', () => {
                console.log('🔧 Menu button clicked!');
                this.showSettingsModal();
            });
        } else {
            console.error('❌ Menu button not found!');
        }
        
        // Quick links
        document.querySelectorAll('.quick-link').forEach(link => {
            link.addEventListener('click', (e) => {
                e.preventDefault();
                this.navigateToUrl(link.dataset.url);
            });
        });
        
        // Welcome search
        const welcomeSearch = document.getElementById('welcome-search');
        const welcomeBtn = document.getElementById('welcome-search-btn');
        
        const welcomeNavigate = () => {
            const query = welcomeSearch.value.trim();
            if (query) this.navigateToUrl(query);
        };
        
        welcomeBtn.addEventListener('click', welcomeNavigate);
        welcomeSearch.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') welcomeNavigate();
        });
        
        // Modal events
        this.setupModalEvents();
        
        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey || e.metaKey) {
                switch (e.key) {
                    case 't':
                        e.preventDefault();
                        this.createNewTab();
                        break;
                    case 'w':
                        e.preventDefault();
                        if (this.activeTabId) {
                            this.closeTab(this.activeTabId);
                        }
                        break;
                    case 'r':
                        e.preventDefault();
                        if (this.activeTabId) {
                            const tab = this.tabs.find(t => t.id === this.activeTabId);
                            if (tab) {
                                this.navigateToUrl(tab.url);
                            }
                        }
                        break;
                    case 'd':
                        e.preventDefault();
                        this.showBookmarkModal();
                        break;
                    case 'l':
                        e.preventDefault();
                        addressInput.focus();
                        addressInput.select();
                        break;
                }
            }
        });
        
        // Tauri menu events
        listen('menu:new_tab', () => {
            this.createNewTab();
        });
    }
    
    // 🎨 MODAL MANAGEMENT
    setupModalEvents() {
        // Bookmark modal
        const bookmarkModal = document.getElementById('bookmark-modal');
        const bookmarkClose = document.getElementById('bookmark-modal-close');
        const bookmarkCancel = document.getElementById('bookmark-cancel');
        const bookmarkSave = document.getElementById('bookmark-save');
        
        const closeBookmarkModal = () => {
            bookmarkModal.classList.remove('active');
        };
        
        bookmarkClose.addEventListener('click', closeBookmarkModal);
        bookmarkCancel.addEventListener('click', closeBookmarkModal);
        bookmarkModal.addEventListener('click', (e) => {
            if (e.target === bookmarkModal) {
                closeBookmarkModal();
            }
        });
        
        bookmarkSave.addEventListener('click', async () => {
            const title = document.getElementById('bookmark-title').value.trim();
            const url = document.getElementById('bookmark-url').value.trim();
            
            if (title && url) {
                await this.addBookmark(title, url);
                closeBookmarkModal();
                
                // Clear form
                document.getElementById('bookmark-title').value = '';
                document.getElementById('bookmark-url').value = '';
            }
        });
        
        // Settings modal
        const settingsModal = document.getElementById('settings-modal');
        const settingsClose = document.getElementById('settings-modal-close');
        const settingsCancel = document.getElementById('settings-cancel');
        const settingsSave = document.getElementById('settings-save');
        
        const closeSettingsModal = () => {
            settingsModal.classList.remove('active');
        };
        
        settingsClose.addEventListener('click', closeSettingsModal);
        settingsCancel.addEventListener('click', closeSettingsModal);
        settingsModal.addEventListener('click', (e) => {
            if (e.target === settingsModal) {
                closeSettingsModal();
            }
        });
        
        settingsSave.addEventListener('click', async () => {
            const newSettings = {
                homepage: document.getElementById('homepage-input').value,
                search_engine: document.getElementById('search-engine-input').value,
                privacy_mode: document.getElementById('privacy-mode').checked,
                ad_blocker: document.getElementById('ad-blocker').checked,
                javascript_enabled: document.getElementById('javascript-enabled').checked,
                cookies_enabled: document.getElementById('cookies-enabled').checked
            };
            
            await this.updateSettings(newSettings);
            closeSettingsModal();
        });
    }
    
    showBookmarkModal() {
        const modal = document.getElementById('bookmark-modal');
        modal.classList.add('active');
        
        // Pre-fill current page info if available
        if (this.activeTabId) {
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                document.getElementById('bookmark-title').value = tab.title;
                document.getElementById('bookmark-url').value = tab.url;
            }
        }
        
        document.getElementById('bookmark-title').focus();
    }
    
    showSettingsModal() {
        const modal = document.getElementById('settings-modal');
        modal.classList.add('active');
        
        // Pre-fill current settings
        if (this.settings) {
            document.getElementById('homepage-input').value = this.settings.homepage;
            document.getElementById('search-engine-input').value = this.settings.search_engine;
            document.getElementById('privacy-mode').checked = this.settings.privacy_mode;
            document.getElementById('ad-blocker').checked = this.settings.ad_blocker;
            document.getElementById('javascript-enabled').checked = this.settings.javascript_enabled;
            document.getElementById('cookies-enabled').checked = this.settings.cookies_enabled;
        }
    }
}

// 🚀 APPLICATION STARTUP
document.addEventListener('DOMContentLoaded', () => {
    console.log('🌐 Starting Ora Browser Frontend...');
    
    try {
        // Initialize the browser application
        window.browserApp = new BrowserApp();
        
        // Update status bar
        const statusElement = document.getElementById('status-text');
        if (statusElement) {
            statusElement.textContent = 'Ora Browser Ready';
        }
        
        console.log('✅ Ora Browser Frontend started successfully!');
        
    } catch (error) {
        console.error('Startup error:', error);
    }
});

// 🔧 UTILITY FUNCTIONS
function updateStatusText(text) {
    document.getElementById('status-text').textContent = text;
}

function showNotification(message, type = 'info') {
    // Simple notification system
    console.log(`[${type.toUpperCase()}] ${message}`);
    updateStatusText(message);
    
    // Clear after 3 seconds
    setTimeout(() => {
        updateStatusText('Ready');
    }, 3000);
}

// 🎯 EXPORT FOR DEBUGGING
window.showNotification = showNotification;
window.updateStatusText = updateStatusText;

// ✅ JAVASCRIPT SUCCESSFULLY LOADED
console.log('🚨 JavaScript file fully loaded and executed'); 