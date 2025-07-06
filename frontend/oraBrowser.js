/**
 * OraBrowser - Modulare Browser-Hauptklasse
 * Vereint alle Module zu einem funktionsfähigen Browser
 */

import OraBrowserCore from './modules/core.js';
import NavigationManager from './modules/navigation.js';
import TabManager from './modules/tabManager.js';
import BookmarkManager from './modules/bookmarkManager.js';
import { waitForTauri, DebugTools, ErrorHandler } from './utils/utils.js';

class OraBrowser {
    constructor() {
        // Initialisiere Core zuerst
        this.core = new OraBrowserCore();
        
        // Initialisiere Manager mit Core-Referenz
        this.navigationManager = new NavigationManager(this.core);
        this.tabManager = new TabManager(this.core, this.navigationManager);
        this.bookmarkManager = new BookmarkManager(this.core, this.navigationManager);
        
        // Referenzen für Kompatibilität
        this.tabs = this.tabManager.tabs;
        this.bookmarks = this.bookmarkManager.bookmarks;
        this.currentUrl = this.core.currentUrl;
        this.isLoading = this.core.isLoading;
        
        console.log('🚀 OraBrowser (modular) initialized');
    }

    // Hauptinitialisierung
    async init() {
        console.log('🔧 Initializing modular OraBrowser...');
        
        try {
            // Warte auf Tauri API
            const tauriAvailable = await waitForTauri();
            console.log('🔧 Tauri API Status:', tauriAvailable);
            
            // Initialisiere Core
            this.core.initializeCore();
            
            // Lade Bookmarks
            console.log('📚 Loading bookmarks...');
            await this.bookmarkManager.loadBookmarksFromBackend();
            
            // Setup UI-Komponenten
            this.setupUIComponents();
            
            // Erstelle ersten Tab
            this.tabManager.createInitialTab();
            
            // Rendere UI
            this.renderInitialUI();
            
            // Setup Event-Listener
            this.setupEventListeners();
            
            console.log('✅ OraBrowser modular initialization complete');
            
        } catch (error) {
            console.error('❌ Initialization error:', error);
            this.core.updateStatus('❌ Initialisierungsfehler');
        }
    }

    // UI-Komponenten einrichten
    setupUIComponents() {
        console.log('🎨 Setting up UI components...');
        
        // Tab-Management einrichten
        this.tabManager.setupTabManagement();
        
        // Bookmark Event-Listener
        this.bookmarkManager.setupEventListeners();
        
        // Navigation-Buttons
        this.setupNavigationButtons();
        
        console.log('✅ UI components setup complete');
    }

    // Navigation-Buttons einrichten
    setupNavigationButtons() {
        // Back Button
        const backBtn = document.getElementById('back-btn');
        if (backBtn) {
            backBtn.addEventListener('click', () => {
                console.log('⬅️ Back button clicked');
                this.navigationManager.goBack();
            });
        }

        // Forward Button
        const forwardBtn = document.getElementById('forward-btn');
        if (forwardBtn) {
            forwardBtn.addEventListener('click', () => {
                console.log('➡️ Forward button clicked');
                this.navigationManager.goForward();
            });
        }

        // Reload Button
        const reloadBtn = document.getElementById('reload-btn');
        if (reloadBtn) {
            reloadBtn.addEventListener('click', () => {
                console.log('🔄 Reload button clicked');
                this.navigationManager.reload();
            });
        }

        // Home Button
        const homeBtn = document.getElementById('home-btn');
        if (homeBtn) {
            homeBtn.addEventListener('click', () => {
                console.log('🏠 Home button clicked');
                this.goHome();
            });
        }

        console.log('✅ Navigation buttons configured');
    }

    // Event-Listener einrichten
    setupEventListeners() {
        console.log('🔧 Setting up main event listeners...');
        
        // URL-Eingabe
        const addressInput = document.getElementById('address-input');
        if (addressInput) {
            addressInput.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    const url = addressInput.value.trim();
                    if (url) {
                        if (e.ctrlKey) {
                            // Ctrl+Enter: Neuer Tab
                            this.tabManager.createNewTabWithUrl(url);
                        } else {
                            // Normal: Aktueller Tab
                            this.navigateToUrl(url);
                        }
                    }
                }
            });
        }

        // Go-Button
        const goBtn = document.getElementById('go-btn');
        if (goBtn) {
            goBtn.addEventListener('click', (e) => {
                const addressInput = document.getElementById('address-input');
                if (addressInput) {
                    const url = addressInput.value.trim();
                    if (url) {
                        if (e.ctrlKey) {
                            // Ctrl+Klick: Neuer Tab
                            this.tabManager.createNewTabWithUrl(url);
                        } else {
                            // Normal: Aktueller Tab
                            this.navigateToUrl(url);
                        }
                    }
                }
            });
        }

        // PostMessage-Handler für iframe-Kommunikation
        window.addEventListener('message', (event) => {
            if (event.data && event.data.type === 'navigate' && event.data.url) {
                console.log('📨 PostMessage navigation request:', event.data.url);
                this.navigateToUrl(event.data.url);
            }
        });
        
        console.log('✅ Main event listeners configured');
    }

    // Initial UI rendern
    renderInitialUI() {
        console.log('🎨 Rendering initial UI...');
        
        // Rendere Tabs
        this.tabManager.renderTabs();
        
        // Rendere Bookmarks
        this.bookmarkManager.renderBookmarks();
        
        // Update Status
        this.core.updateStatus('Bereit');
        
        console.log('✅ Initial UI rendered');
    }

    // Delegierte Methoden für Kompatibilität
    
    // Navigation
    async navigateToUrl(url, updateTab = true, addToHistory = true) {
        this.currentUrl = url;
        const result = await this.navigationManager.navigateToUrl(url, updateTab, addToHistory);
        
        // Update Tab-Info
        if (updateTab && this.tabManager.activeTabId) {
            this.tabManager.updateActiveTabInfo(this.getPageTitle(), url);
        }
        
        return result;
    }

    // Home-Navigation
    goHome() {
        const homepage = this.core.getSettings().homepage || 'https://www.google.com';
        this.navigateToUrl(homepage);
    }

    // Tab-Methoden
    createNewTab(title, url) {
        return this.tabManager.createNewTab(title, url);
    }

    createNewTabWithUrl(url, title) {
        return this.tabManager.createNewTabWithUrl(url, title);
    }

    switchToTab(tabId) {
        return this.tabManager.switchToTab(tabId);
    }

    closeTab(tabId) {
        return this.tabManager.closeTab(tabId);
    }

    updateActiveTabInfo(title, url) {
        return this.tabManager.updateActiveTabInfo(title, url);
    }

    // Bookmark-Methoden
    async addBookmarkManual(title, url) {
        return await this.bookmarkManager.addBookmark(title, url);
    }

    async removeBookmark(index) {
        return await this.bookmarkManager.removeBookmark(index);
    }

    showBookmarkModal() {
        return this.bookmarkManager.showBookmarkModal();
    }

    showBookmarkManager() {
        return this.bookmarkManager.showBookmarkManager();
    }

    renderBookmarks() {
        return this.bookmarkManager.renderBookmarks();
    }

    async loadBookmarksFromBackend() {
        return await this.bookmarkManager.loadBookmarksFromBackend();
    }

    // Utility-Methoden
    updateStatus(message) {
        return this.core.updateStatus(message);
    }

    setLoading(loading) {
        this.isLoading = loading;
        return this.core.setLoading(loading);
    }

    normalizeUrl(url) {
        return this.core.normalizeUrl(url);
    }

    extractDomain(url) {
        return this.core.extractDomain(url);
    }

    checkTauriAPI() {
        return this.core.checkTauriAPI();
    }

    getSettings() {
        return this.core.getSettings();
    }

    saveSettings(settings) {
        return this.core.saveSettings(settings);
    }

    // Page-Title extrahieren
    getPageTitle() {
        return this.tabManager.getPageTitle();
    }

    // Debug-Methoden
    debugContentDisplay() {
        DebugTools.checkElements();
    }

    testNavigation(url = 'https://google.com') {
        console.log('🧪 Testing navigation to:', url);
        return this.navigateToUrl(url);
    }

    showTestContent() {
        DebugTools.showTestContent();
    }

    // Statistiken
    getStats() {
        return {
            tabs: this.tabManager.getTabStats(),
            bookmarks: {
                total: this.bookmarks.length,
                secured: this.bookmarks.filter(b => b.url.startsWith('https://')).length
            },
            currentUrl: this.currentUrl,
            isLoading: this.isLoading
        };
    }
}

// Browser-Initialisierung
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🌐 DOM Content Loaded - Starting modular OraBrowser...');
    
    try {
        // Erstelle Browser-Instanz
        window.oraBrowser = new OraBrowser();
        
        // Initialisiere Browser
        await window.oraBrowser.init();
        
        console.log('✅ Modular OraBrowser ready');
        
    } catch (error) {
        console.error('❌ OraBrowser initialization failed:', error);
        
        // Fallback-Modus
        console.log('🔄 Starting fallback mode...');
        window.oraBrowser = new OraBrowser();
        window.oraBrowser.core.initializeCore();
    }
});

// Global Debug-Tools
window.testOraBrowser = {
    showStats: () => {
        if (window.oraBrowser) {
            console.table(window.oraBrowser.getStats());
        }
    },
    
    testNavigation: (url = 'https://duckduckgo.com') => {
        if (window.oraBrowser) {
            window.oraBrowser.testNavigation(url);
        }
    },
    
    checkElements: () => {
        DebugTools.checkElements();
    },
    
    showTestContent: () => {
        DebugTools.showTestContent();
    }
};

console.log('📜 Modular OraBrowser script loaded - Ready for initialization');

export default OraBrowser; 