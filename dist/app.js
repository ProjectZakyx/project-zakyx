// 🌐 ORA BROWSER - TAURI FRONTEND APPLICATION
// Modern cross-platform browser frontend with Tauri integration

const { invoke } = window.__TAURI__.tauri;
const { listen } = window.__TAURI__.event;
const { appWindow } = window.__TAURI__.window;

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
        if (!url.startsWith('http')) {
            if (url.includes('.')) {
                url = 'https://' + url;
            } else {
                url = `https://www.google.com/search?q=${encodeURIComponent(url)}`;
            }
        }
        
        if (!this.activeTabId) {
            await this.createNewTab(url);
            return;
        }
        
        try {
            await invoke('navigate_to', { tabId: this.activeTabId, url });
            const tab = this.tabs.find(t => t.id === this.activeTabId);
            if (tab) {
                tab.url = url;
                tab.title = 'Loading...';
            }
            this.renderTabs();
            this.updateAddressBar(url);
            this.hideWelcomeScreen();
        } catch (error) {
            console.error('Navigation failed:', error);
        }
    }
    
    getTitleFromUrl(url) {
        try {
            const domain = new URL(url).hostname;
            return domain.replace('www.', '').split('.')[0];
        } catch {
            return 'New Tab';
        }
    }
    
    // 📚 BOOKMARK MANAGEMENT
    async loadBookmarks() {
        try {
            this.bookmarks = await invoke('get_bookmarks');
        } catch (error) {
            console.error('Failed to load bookmarks:', error);
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
            
            tabEl.addEventListener('click', () => {
                this.activeTabId = tab.id;
                this.renderTabs();
                this.updateAddressBar(tab.url);
            });
            
            container.appendChild(tabEl);
        });
    }
    
    renderBookmarks() {
        const container = document.getElementById('bookmarks-container');
        container.innerHTML = '';
        
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
    }
    
    hideWelcomeScreen() {
        document.getElementById('welcome-screen').style.display = 'none';
    }
    
    // 🎯 EVENT LISTENERS
    setupEventListeners() {
        // Address bar
        const addressInput = document.getElementById('address-input');
        const goBtn = document.getElementById('go-btn');
        
        const navigate = () => {
            const url = addressInput.value.trim();
            if (url) this.navigateToUrl(url);
        };
        
        goBtn.addEventListener('click', navigate);
        addressInput.addEventListener('keypress', (e) => {
            if (e.key === 'Enter') navigate();
        });
        
        // New tab button
        document.getElementById('new-tab-btn').addEventListener('click', () => {
            this.createNewTab();
        });
        
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
    
    // Initialize the browser application
    window.browserApp = new BrowserApp();
    
    // Update status bar
    document.getElementById('status-text').textContent = 'Ora Browser Ready';
    
    console.log('✅ Ora Browser Frontend started successfully!');
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