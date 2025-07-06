/**
 * OraBrowser Main Module
 * Browser-Initialisierung und Event-Setup
 */

import { waitForTauri, ErrorHandler, EventUtils, DomUtils, DebugTools } from './utils.js';
import { OraBrowserCore } from './core.js';
import { NavigationManager } from './navigation.js';
import { TabManager, BookmarkManager } from './ui.js';

class OraBrowser {
    constructor() {
        this.core = null;
        this.navigationManager = null;
        this.tabManager = null;
        this.bookmarkManager = null;
        this.isInitialized = false;
        
        console.log('🌐 OraBrowser instance created');
    }

    // Browser initialisieren
    async init() {
        try {
            console.log('🚀 Initializing OraBrowser modules...');
            
            // Core initialisieren
            this.core = new OraBrowserCore();
            this.core.initializeCore();
            
            // Navigation Manager
            this.navigationManager = new NavigationManager(this.core);
            
            // UI Manager
            this.tabManager = new TabManager(this.core, this.navigationManager);
            this.bookmarkManager = new BookmarkManager(this.core, this.navigationManager);
            
            // UI-Setup
            this.setupUIComponents();
            this.setupEventListeners();
            this.setupTauriEventListeners();
            
            // Bookmarks laden
            await this.bookmarkManager.loadBookmarksFromBackend();
            
            // Initial Tab erstellen
            this.tabManager.createInitialTab();
            
            // UI rendern
            this.renderInitialUI();
            
            // Tauri API prüfen
            await waitForTauri(3000);
            
            this.isInitialized = true;
            console.log('✅ OraBrowser fully initialized');
            
        } catch (error) {
            console.error('❌ OraBrowser initialization failed:', error);
            this.handleInitializationError(error);
        }
    }

    // UI-Komponenten setup
    setupUIComponents() {
        console.log('🎨 Setting up UI components...');
        
        // Navigation Buttons
        this.setupNavigationButtons();
        
        // Address Bar
        this.setupAddressBar();
        
        // New Tab Button
        this.setupNewTabButton();
        
        // Bookmark Button
        this.setupBookmarkButton();
        
        console.log('✅ UI components setup complete');
    }

    // Navigation Buttons
    setupNavigationButtons() {
        const backBtn = DomUtils.safeGetElement('back-btn');
        const forwardBtn = DomUtils.safeGetElement('forward-btn');
        const reloadBtn = DomUtils.safeGetElement('reload-btn');
        const homeBtn = DomUtils.safeGetElement('home-btn');

        if (backBtn) {
            EventUtils.addListener(backBtn, 'click', () => {
                this.navigationManager.goBack();
            });
        }

        if (forwardBtn) {
            EventUtils.addListener(forwardBtn, 'click', () => {
                this.navigationManager.goForward();
            });
        }

        if (reloadBtn) {
            EventUtils.addListener(reloadBtn, 'click', () => {
                this.navigationManager.reload();
            });
        }

        if (homeBtn) {
            EventUtils.addListener(homeBtn, 'click', () => {
                this.goHome();
            });
        }
    }

    // Address Bar setup
    setupAddressBar() {
        const addressInput = DomUtils.safeGetElement('address-input');
        const goBtn = DomUtils.safeGetElement('go-btn');

        if (addressInput) {
            // Enter-Taste
            EventUtils.addListener(addressInput, 'keydown', (e) => {
                if (e.key === 'Enter') {
                    const url = addressInput.value.trim();
                    if (url) {
                        if (e.ctrlKey) {
                            // Ctrl+Enter: Neuer Tab
                            this.tabManager.createNewTabWithUrl(url);
                        } else {
                            // Normal: Aktueller Tab
                            this.navigationManager.navigateToUrl(url);
                        }
                    }
                }
            });

            // Focus-Effekte
            EventUtils.addListener(addressInput, 'focus', () => {
                addressInput.select();
            });
        }

        if (goBtn) {
            EventUtils.addListener(goBtn, 'click', (e) => {
                const url = addressInput ? addressInput.value.trim() : '';
                if (url) {
                    if (e.ctrlKey) {
                        this.tabManager.createNewTabWithUrl(url);
                    } else {
                        this.navigationManager.navigateToUrl(url);
                    }
                }
            });
        }
    }

    // New Tab Button
    setupNewTabButton() {
        const newTabBtn = DomUtils.safeGetElement('new-tab-btn');
        if (newTabBtn) {
            EventUtils.addListener(newTabBtn, 'click', () => {
                const tabId = this.tabManager.createNewTab();
                this.tabManager.switchToTab(tabId);
            });
        }
    }

    // Bookmark Button
    setupBookmarkButton() {
        const bookmarkBtn = DomUtils.safeGetElement('bookmark-btn');
        if (bookmarkBtn) {
            EventUtils.addListener(bookmarkBtn, 'click', () => {
                this.bookmarkManager.showBookmarkModal();
            });
        }

        // Bookmark Modal Event-Listeners
        this.setupBookmarkModal();
    }

    // Bookmark Modal
    setupBookmarkModal() {
        const modal = DomUtils.safeGetElement('bookmark-modal');
        const closeBtn = DomUtils.safeGetElement('bookmark-modal-close');
        const saveBtn = DomUtils.safeGetElement('bookmark-save');
        const cancelBtn = DomUtils.safeGetElement('bookmark-cancel');
        const titleInput = DomUtils.safeGetElement('bookmark-title');
        const urlInput = DomUtils.safeGetElement('bookmark-url');

        // Close button
        if (closeBtn) {
            EventUtils.addListener(closeBtn, 'click', () => {
                if (modal) modal.style.display = 'none';
            });
        }

        // Cancel button
        if (cancelBtn) {
            EventUtils.addListener(cancelBtn, 'click', () => {
                if (modal) modal.style.display = 'none';
            });
        }

        // Save button aktivieren/deaktivieren basierend auf Input
        const updateSaveButton = () => {
            if (saveBtn && titleInput && urlInput) {
                const hasTitle = titleInput.value.trim().length > 0;
                const hasUrl = urlInput.value.trim().length > 0;
                saveBtn.disabled = !(hasTitle && hasUrl);
                console.log('📚 Save button state:', { hasTitle, hasUrl, disabled: saveBtn.disabled });
            }
        };

        // Input Event-Listener
        if (titleInput) {
            EventUtils.addListener(titleInput, 'input', updateSaveButton);
            EventUtils.addListener(titleInput, 'keyup', updateSaveButton);
        }

        if (urlInput) {
            EventUtils.addListener(urlInput, 'input', updateSaveButton);
            EventUtils.addListener(urlInput, 'keyup', updateSaveButton);
        }

        // Save button
        if (saveBtn) {
            EventUtils.addListener(saveBtn, 'click', async () => {
                if (titleInput && urlInput) {
                    const title = titleInput.value.trim();
                    const url = urlInput.value.trim();
                    
                    console.log('📚 Attempting to save bookmark:', { title, url });
                    
                    if (title && url) {
                        try {
                            const success = await this.bookmarkManager.addBookmark(title, url);
                            console.log('📚 Bookmark save result:', success);
                            
                            if (success && modal) {
                                modal.style.display = 'none';
                                titleInput.value = '';
                                urlInput.value = '';
                                updateSaveButton();
                                console.log('📚 Bookmark modal closed');
                            }
                        } catch (error) {
                            console.error('📚 Bookmark save error:', error);
                        }
                    }
                }
            });
        }

        // Modal schließen bei Klick außerhalb
        if (modal) {
            EventUtils.addListener(modal, 'click', (e) => {
                if (e.target === modal) {
                    modal.style.display = 'none';
                }
            });
        }

        // Initial save button state
        updateSaveButton();
        
        console.log('📚 Bookmark modal setup complete');
    }

    // Event-Listeners setup
    setupEventListeners() {
        console.log('🎯 Setting up event listeners...');
        
        // PostMessage-Listener für iframe-Kommunikation
        window.addEventListener('message', (event) => {
            // Sicherheitscheck
            if (event.origin !== window.location.origin && event.origin !== 'null') {
                return;
            }
            
            if (event.data && event.data.type === 'oraBrowser_navigate') {
                console.log('📨 Received navigation message from iframe:', event.data.url);
                if (this.navigationManager) {
                    this.navigationManager.navigateToUrl(event.data.url);
                }
            }
        });
        
        // Keyboard Shortcuts
        EventUtils.addListener(document, 'keydown', (e) => {
            // Ctrl+T: Neuer Tab
            if (e.ctrlKey && e.key === 't') {
                e.preventDefault();
                const tabId = this.tabManager.createNewTab();
                this.tabManager.switchToTab(tabId);
            }
            
            // Ctrl+W: Tab schließen
            if (e.ctrlKey && e.key === 'w') {
                e.preventDefault();
                if (this.tabManager.activeTabId) {
                    this.tabManager.closeTab(this.tabManager.activeTabId);
                }
            }
            
            // Ctrl+L: Address Bar fokussieren
            if (e.ctrlKey && e.key === 'l') {
                e.preventDefault();
                const addressInput = DomUtils.safeGetElement('address-input');
                if (addressInput) {
                    addressInput.focus();
                    addressInput.select();
                }
            }
            
            // Alt+Home: Home
            if (e.altKey && e.key === 'Home') {
                e.preventDefault();
                this.goHome();
            }
        });

        console.log('✅ Event listeners setup complete');
    }

    // Tauri Event-Listeners
    setupTauriEventListeners() {
        if (window.__TAURI__ && window.__TAURI__.event) {
            // Navigation Events
            window.__TAURI__.event.listen('webview_navigate', (event) => {
                console.log('🌐 Backend navigation event:', event.payload);
                if (event.payload) {
                    this.navigationManager.navigateToUrl(event.payload, false);
                }
            });

            window.__TAURI__.event.listen('webview_loaded', (event) => {
                console.log('✅ Backend webview loaded:', event.payload);
                if (event.payload && event.payload.url) {
                    this.core.updateStatus(`Geladen: ${this.core.extractDomain(event.payload.url)}`);
                }
            });

            window.__TAURI__.event.listen('navigation_success', (event) => {
                console.log('🎯 Backend navigation success:', event.payload);
                this.core.updateStatus('Navigation erfolgreich');
            });

            console.log('🎧 Tauri event listeners registered');
        } else {
            console.warn('⚠️ Tauri event API not available - events will be ignored');
        }
    }

    // Initial UI rendern
    renderInitialUI() {
        console.log('🎨 Rendering initial UI...');
        
        // Tabs rendern
        this.tabManager.renderTabs();
        
        // Bookmarks rendern
        this.bookmarkManager.renderBookmarks();
        
        // Status aktualisieren
        this.core.updateStatus('Bereit');
        
        console.log('✅ Initial UI rendered');
    }

    // Home-Navigation
    goHome() {
        const homepage = this.core.getSettings().homepage || 'https://www.google.com';
        this.navigationManager.navigateToUrl(homepage);
    }

    // Initialisierungs-Error behandeln
    handleInitializationError(error) {
        console.error('🚨 Initialization error:', error);
        
        // Fallback-Mode
        document.body.innerHTML = `
            <div style="
                padding: 40px; 
                text-align: center; 
                font-family: Arial, sans-serif;
                background: linear-gradient(135deg, #ff6b6b 0%, #ee5a24 100%);
                color: white;
                min-height: 100vh;
                display: flex;
                flex-direction: column;
                justify-content: center;
            ">
                <h1>🚨 OraBrowser Initialisierungsfehler</h1>
                <p>Der Browser konnte nicht korrekt initialisiert werden.</p>
                <div style="background: rgba(255,255,255,0.1); padding: 20px; border-radius: 10px; margin: 20px 0;">
                    <strong>Fehler:</strong> ${error.message || error}
                </div>
                <button onclick="location.reload()" style="
                    padding: 15px 30px; 
                    background: rgba(255,255,255,0.2); 
                    border: none; 
                    border-radius: 8px; 
                    color: white; 
                    cursor: pointer;
                    font-size: 1.1rem;
                ">
                    🔄 Seite neu laden
                </button>
            </div>
        `;
    }

    // Browser-Statistiken
    getStats() {
        if (!this.isInitialized) {
            return { status: 'not_initialized' };
        }

        return {
            core: this.core.getSystemStatus(),
            tabs: this.tabManager.getTabStats(),
            bookmarks: {
                total: this.bookmarkManager.bookmarks.length
            },
            performance: this.core.getPerformanceMetrics(),
            initialized: this.isInitialized
        };
    }
}

// Browser-Initialisierung
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🌐 DOM Content Loaded - Starting OraBrowser...');
    
    // Error-Handling setup
    ErrorHandler.setupGlobalErrorHandling();
    DebugTools.startPerformanceMonitor();
    
    try {
        // Browser-Instanz erstellen
        window.oraBrowser = new OraBrowser();
        
        // Initialisieren
        await window.oraBrowser.init();
        
        console.log('✅ OraBrowser ready for use');
        
    } catch (error) {
        console.error('❌ OraBrowser startup failed:', error);
        
        // Fallback-Initialisierung
        console.log('🔄 Attempting fallback initialization...');
        window.oraBrowser = new OraBrowser();
        window.oraBrowser.handleInitializationError(error);
    }
});

// Global Debug-Tools
window.testOraBrowser = {
    getStats: () => window.oraBrowser?.getStats(),
    checkElements: () => DebugTools.checkElements(),
    navigateTo: (url) => window.oraBrowser?.navigationManager?.navigateToUrl(url),
    createTab: (url) => window.oraBrowser?.tabManager?.createNewTabWithUrl(url)
};

console.log('📦 Main module loaded - Ready for initialization'); 