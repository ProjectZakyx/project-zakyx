// Ora Browser - Clean Frontend for Backend Navigation System
// Vollständig funktionsfähige JavaScript-Datei mit Tab-Management und Bookmark-Manager

class OraBrowser {
    constructor() {
        this.tabs = [];
        this.nextTabId = 1;
        this.activeTabId = null;
        this.currentUrl = '';
        this.bookmarks = [
            {
                id: '1',
                title: '🔍 Google',
                url: 'https://google.com'
            },
            {
                id: '2', 
                title: '🐙 GitHub',
                url: 'https://github.com'
            },
            {
                id: '3',
                title: '📺 YouTube', 
                url: 'https://youtube.com'
            },
            {
                id: '4',
                title: '📚 Wikipedia',
                url: 'https://wikipedia.org'
            }
        ];
        this.history = [];
        this.historyIndex = -1;
        this.plugins = [];
        this.isLoading = false;
        
        // Settings-Initialisierung hinzufügen
        this.settings = {
            homepage: 'https://www.google.com',
            searchEngine: 'https://www.google.com/search?q=',
            privacyMode: false,
            adBlocker: false,
            javascriptEnabled: true,
            cookiesEnabled: true
        };
        
        // Plugin Manager
        this.pluginManager = {
            loadedPlugins: 4,
            enabledPlugins: 3
        };
        
        console.log('🚀 OraBrowser initialized');
    }

    // 🔗 TAURI API PRÜFUNG
    checkTauriAPI() {
        return !!(window.__TAURI__ && window.__TAURI__.core && window.__TAURI__.core.invoke);
    }

    async init() {
        console.log('🔧 Initializing OraBrowser...');
        
        try {
            // Warte auf Tauri API (falls verfügbar)
            await waitForTauri();
            
            // Lade Settings beim Start
            this.settings = this.getSettings();
            console.log('⚙️ Settings loaded:', this.settings);
            
            // Initialisiere UI-Komponenten
            this.initializeUI();
            
            // Erstelle ersten Tab
            this.createInitialTab();
            
            // Zeige Welcome Screen
            this.showWelcomeScreen();
            
            // Rendere Bookmarks
            this.renderBookmarks();
            
            // DEBUG: Teste alle wichtigen Elemente
            this.debugTestElements();
            
            console.log('✅ OraBrowser initialization complete');
            
        } catch (error) {
            console.error('❌ Initialization error:', error);
            this.updateStatus('❌ Initialisierungsfehler');
        }
    }

    initializeUI() {
        console.log('🎨 Initializing UI components...');
        
        // Setup Event-Listener
        this.setupEventListeners();
        
        // Setup Tab-Management
        this.setupTabManagement();
        
        // Initial Status
        this.updateStatus('Bereit');
        
        // Zeige Browser-Informationen
        console.log('🌐 Ora Browser v1.0.0 - Ready');
        console.log(`📚 ${this.bookmarks.length} Bookmarks loaded`);
        console.log(`🔌 ${this.pluginManager.enabledPlugins}/${this.pluginManager.loadedPlugins} Plugins enabled`);
    }

    createInitialTab() {
        this.createNewTab('Neuer Tab', 'about:blank');
    }

    createNewTab(title = 'Neuer Tab', url = 'about:blank') {
        const tabId = this.nextTabId++;
        const tab = {
            id: tabId,
            title: title,
            url: url,
            isActive: false,
            history: [],
            historyIndex: -1
        };
        
        this.tabs.push(tab);
        this.renderTabs();
        this.switchToTab(tabId);
        
        console.log(`📑 New tab created: ${tabId} - ${title}`);
        return tabId;
    }

    switchToTab(tabId) {
        // Deaktiviere alle Tabs
        this.tabs.forEach(tab => tab.isActive = false);
        
        // Aktiviere den gewählten Tab
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab) {
            tab.isActive = true;
            this.activeTabId = tabId;
            this.currentUrl = tab.url;
            this.updateUrlInput(tab.url);
            this.updateStatus(`Tab aktiv: ${tab.title}`);
            
            // Wenn es nicht der Welcome Screen ist, lade die URL
            if (tab.url !== 'about:blank') {
                this.navigateToUrl(tab.url, false); // false = don't create new tab
            } else {
                this.showWelcomeScreen();
            }
        }
        
        this.renderTabs();
        console.log(`📑 Switched to tab: ${tabId}`);
    }

    closeTab(tabId) {
        const tabIndex = this.tabs.findIndex(t => t.id === tabId);
        if (tabIndex === -1) return;
        
        const wasActive = this.tabs[tabIndex].isActive;
        this.tabs.splice(tabIndex, 1);
        
        // Wenn das der letzte Tab war, erstelle einen neuen
        if (this.tabs.length === 0) {
            this.createNewTab();
            return;
        }
        
        // Wenn der aktive Tab geschlossen wurde, wechsle zum nächsten
        if (wasActive) {
            const nextTab = this.tabs[Math.min(tabIndex, this.tabs.length - 1)];
            this.switchToTab(nextTab.id);
        }
        
        this.renderTabs();
        console.log(`📑 Tab closed: ${tabId}`);
    }

    renderTabs() {
        const tabsContainer = document.getElementById('tabs-container');
        if (!tabsContainer) return;
        
        tabsContainer.innerHTML = '';
        
        this.tabs.forEach(tab => {
            const tabElement = document.createElement('div');
            tabElement.className = `tab ${tab.isActive ? 'active' : ''}`;
            tabElement.innerHTML = `
                <span class="tab-title" title="${tab.url}">${tab.title}</span>
                <button class="tab-close" data-tab-id="${tab.id}" title="Tab schließen">×</button>
            `;
            
            // Tab-Klick Event
            tabElement.addEventListener('click', (e) => {
                if (!e.target.classList.contains('tab-close')) {
                    this.switchToTab(tab.id);
                }
            });
            
            // Tab-Schließen Event
            const closeBtn = tabElement.querySelector('.tab-close');
            closeBtn.addEventListener('click', (e) => {
                e.stopPropagation();
                this.closeTab(tab.id);
            });
            
            tabsContainer.appendChild(tabElement);
        });
    }

    setupTabManagement() {
        // Neuer Tab Button - Entferne alte Event-Listener zuerst
        const newTabBtn = document.getElementById('new-tab-btn');
        if (newTabBtn) {
            // Entferne alle alten Event-Listener
            const newBtn = newTabBtn.cloneNode(true);
            newTabBtn.parentNode.replaceChild(newBtn, newTabBtn);
            
            // Füge neuen Event-Listener hinzu
            newBtn.addEventListener('click', (e) => {
                e.preventDefault();
                e.stopPropagation();
                console.log('🆕 Creating new tab...');
                this.createNewTab();
            });
        }
        
        // Keyboard shortcuts - nur einmal registrieren
        if (!this.keyboardListenerAdded) {
            document.addEventListener('keydown', (e) => {
                if (e.ctrlKey) {
                    switch(e.key) {
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
                        case 'Tab':
                            e.preventDefault();
                            this.switchToNextTab();
                            break;
                    }
                }
            });
            this.keyboardListenerAdded = true;
        }
    }

    switchToNextTab() {
        if (this.tabs.length <= 1) return;
        
        const currentIndex = this.tabs.findIndex(t => t.id === this.activeTabId);
        const nextIndex = (currentIndex + 1) % this.tabs.length;
        this.switchToTab(this.tabs[nextIndex].id);
    }

    updateActiveTabInfo(title, url) {
        const activeTab = this.tabs.find(t => t.id === this.activeTabId);
        if (activeTab) {
            activeTab.title = title || this.getPageTitle() || 'Unbekannte Seite';
            activeTab.url = url || this.currentUrl;
            this.renderTabs();
        }
    }

    showWelcomeScreen() {
        console.log('🚨 showWelcomeScreen() called - BLOCKED to prevent navigation issues');
        
        // 🚨 BLOCKIERE WELCOME SCREEN KOMPLETT
        const welcomeScreen = document.getElementById('welcome-screen');
        const webviewContainer = document.getElementById('webview-container');
        const contentContainer = document.getElementById('content-container');
        
        if (welcomeScreen) {
            welcomeScreen.style.display = 'none !important';
            welcomeScreen.style.visibility = 'hidden !important';
        }
        
        if (contentContainer) {
            contentContainer.style.display = 'block !important';
            contentContainer.style.visibility = 'visible !important';
        }
        
        if (webviewContainer) {
            webviewContainer.style.display = 'none';
        }
        
        console.log('🚨 Welcome screen display BLOCKED - Content remains visible');
    }

    setupNavigationButtons() {
        // Back Button
        const backBtn = document.getElementById('back-btn');
        if (backBtn) {
            backBtn.addEventListener('click', () => {
                console.log('⬅️ Back button clicked');
                this.goBack();
            });
        }

        // Forward Button
        const forwardBtn = document.getElementById('forward-btn');
        if (forwardBtn) {
            forwardBtn.addEventListener('click', () => {
                console.log('➡️ Forward button clicked');
                this.goForward();
            });
        }

        // Reload Button
        const reloadBtn = document.getElementById('reload-btn');
        if (reloadBtn) {
            reloadBtn.addEventListener('click', () => {
                console.log('🔄 Reload button clicked');
                this.reload();
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

    goBack() {
        if (this.history.length > 1 && this.historyIndex > 0) {
            this.historyIndex--;
            const url = this.history[this.historyIndex];
            this.navigateToUrl(url, true, false); // false = don't add to history
            this.updateStatus(`Zurück zu: ${url}`);
        } else {
            this.updateStatus('Keine vorherige Seite verfügbar');
        }
    }

    goForward() {
        if (this.historyIndex < this.history.length - 1) {
            this.historyIndex++;
            const url = this.history[this.historyIndex];
            this.navigateToUrl(url, true, false); // false = don't add to history
            this.updateStatus(`Vorwärts zu: ${url}`);
        } else {
            this.updateStatus('Keine nächste Seite verfügbar');
        }
    }

    reload() {
        if (this.currentUrl && this.currentUrl !== 'about:blank') {
            console.log('🔄 Reloading current URL:', this.currentUrl);
            this.navigateToUrl(this.currentUrl);
            this.updateStatus('Seite wird neu geladen...');
        } else {
            console.log('🔄 No valid URL to reload, staying on current page');
            this.updateStatus('Keine Seite zum Neuladen');
        }
    }

    goHome() {
        console.log('🏠 Going home...');
        
        // Verwende Homepage aus Settings oder Default
        const settings = this.getSettings();
        const homepage = settings.homepage || 'https://www.google.com';
        
        console.log('🏠 Navigating to homepage:', homepage);
        this.navigateToUrl(homepage);
    }

    addToHistory(url) {
        // Entferne alle Einträge nach dem aktuellen Index (für neue Navigation)
        this.history = this.history.slice(0, this.historyIndex + 1);
        
        // Füge neue URL hinzu, wenn sie sich von der aktuellen unterscheidet
        if (this.history[this.history.length - 1] !== url) {
            this.history.push(url);
            this.historyIndex = this.history.length - 1;
        }
        
        // Begrenze History-Größe
        if (this.history.length > 100) {
            this.history = this.history.slice(-100);
            this.historyIndex = this.history.length - 1;
        }
        
        console.log(`📚 Added to history: ${url} (index: ${this.historyIndex})`);
    }

    setupEventListeners() {
        console.log('🔧 Setting up main event listeners...');
        
        // URL-Eingabe (address-input statt url-input)
        const addressInput = document.getElementById('address-input');
        console.log('🔍 addressInput found:', !!addressInput);
        
        if (addressInput) {
            console.log('✅ Setting up address input listeners');
            addressInput.addEventListener('keypress', (e) => {
                console.log('🔍 Address input keypress:', e.key, 'ctrlKey:', e.ctrlKey);
                if (e.key === 'Enter') {
                    const url = addressInput.value.trim();
                    console.log('🔍 Navigating to URL from address bar:', url);
                    if (url) {
                        if (e.ctrlKey) {
                            // Ctrl+Enter: Neuer Tab
                            console.log('🔍 Creating new tab for URL:', url);
                            this.createNewTab('Loading...', url);
                            this.navigateToUrl(url);
                        } else {
                            // Normal: Aktueller Tab
                            console.log('🔍 Navigating current tab to URL:', url);
                            this.navigateToUrl(url);
                        }
                    }
                }
            });
        } else {
            console.error('❌ address-input element not found!');
            // Versuche Fallback-Suche
            const fallbackInput = document.querySelector('input[type="text"]');
            if (fallbackInput) {
                console.log('✅ Found fallback input element');
                fallbackInput.addEventListener('keypress', (e) => {
                    if (e.key === 'Enter') {
                        const url = fallbackInput.value.trim();
                        if (url) this.navigateToUrl(url);
                    }
                });
            }
        }

        // Go-Button
        const goBtn = document.getElementById('go-btn');
        console.log('🔍 goBtn found:', !!goBtn);
        
        if (goBtn) {
            console.log('✅ Setting up go button listener');
            goBtn.addEventListener('click', (e) => {
                console.log('🔍 Go button clicked, ctrlKey:', e.ctrlKey);
                
                const addressInput = document.getElementById('address-input');
                if (addressInput) {
                    const url = addressInput.value.trim();
                    console.log('🔍 Go button URL:', url);
                    if (url) {
                        if (e.ctrlKey) {
                            // Ctrl+Klick: Neuer Tab
                            console.log('🔍 Go button creating new tab for URL:', url);
                            this.createNewTab('Loading...', url);
                            this.navigateToUrl(url);
                        } else {
                            // Normal: Aktueller Tab
                            console.log('🔍 Go button navigating current tab to URL:', url);
                            this.navigateToUrl(url);
                        }
                    }
                } else {
                    console.error('❌ Could not find address input when go button clicked');
                }
            });
        } else {
            console.error('❌ go-btn element not found!');
        }

        // Welcome Search
        const welcomeSearch = document.getElementById('welcome-search');
        const welcomeSearchBtn = document.getElementById('welcome-search-btn');
        console.log('🔍 welcomeSearch found:', !!welcomeSearch);
        console.log('🔍 welcomeSearchBtn found:', !!welcomeSearchBtn);
        
        if (welcomeSearch) {
            console.log('✅ Setting up welcome search listeners');
            welcomeSearch.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this.handleWelcomeSearch();
                }
            });
        }
        
        if (welcomeSearchBtn) {
            console.log('✅ Setting up welcome search button listener');
            welcomeSearchBtn.addEventListener('click', () => {
                this.handleWelcomeSearch();
            });
        }

        // Quick Links
        const quickLinks = document.querySelectorAll('.quick-link');
        console.log('🔍 quickLinks found:', quickLinks.length);
        
        quickLinks.forEach((link, index) => {
            console.log(`✅ Setting up quick link ${index + 1} listener`);
            link.addEventListener('click', (e) => {
                e.preventDefault();
                const url = link.getAttribute('data-url');
                if (url) {
                    this.navigateToUrl(url);
                }
            });
        });

        // Setup Navigation Buttons
        this.setupNavigationButtons();
        
        // Setup Backend Event Listeners
        this.setupBackendEventListeners();
        
        // Setup Tab Event Listeners
        this.setupTabEventListeners();
        
        // Setup Bookmark Event Listeners
        this.setupBookmarkEventListeners();
        
        // Setup Settings Event Listeners
        this.setupSettingsEventListeners();
        
        // Setup Plugin Event Listeners
        this.setupPluginEventListeners();
        
        // 🚀 SETUP POSTMESSAGE LISTENER FÜR IFRAME-NAVIGATION
        window.addEventListener('message', (event) => {
            console.log('📨 PostMessage received:', event.data);
            
            if (event.data && event.data.type === 'navigate' && event.data.url) {
                console.log('🚀 PostMessage navigation request:', event.data.url);
                this.navigateToUrl(event.data.url);
            }
        });
        
        // 🚀 F5/RELOAD-SCHUTZ - VERHINDERE WELCOME-SCREEN BEI RELOAD
        window.addEventListener('keydown', (e) => {
            if (e.key === 'F5' || (e.ctrlKey && e.key === 'r')) {
                console.log('🔄 F5/Ctrl+R detected - preventing welcome screen reload');
                e.preventDefault();
                
                if (this.currentUrl && this.currentUrl !== 'about:blank') {
                    console.log('🔄 Reloading current URL instead of welcome screen:', this.currentUrl);
                    this.reload();
                } else {
                    console.log('🔄 No current URL, staying on current page');
                }
            }
        });
        
        console.log('✅ All event listeners configured');
    }

    handleWelcomeSearch() {
        const searchInput = document.getElementById('welcome-search');
        if (searchInput) {
            const query = searchInput.value.trim();
            if (query) {
                // Prüfe ob es eine URL ist
                if (query.includes('.') && !query.includes(' ')) {
                    this.navigateToUrl(query);
                } else {
                    // Verwende Suchmaschine aus Settings
                    const settings = this.getSettings();
                    const searchEngine = settings.searchEngine || 'https://www.google.com/search?q=';
                    
                    // Stelle sicher, dass die Suchmaschinen-URL mit ?q= endet
                    let searchUrl = searchEngine;
                    if (!searchUrl.includes('?q=')) {
                        searchUrl = searchUrl.endsWith('/') ? searchUrl + 'search?q=' : searchUrl + '/search?q=';
                    }
                    
                    console.log('🔍 Using search engine:', searchUrl);
                    this.navigateToUrl(`${searchUrl}${encodeURIComponent(query)}`);
                }
            }
        }
    }

    updateUrlInput(url) {
        const addressInput = document.getElementById('address-input');
        if (addressInput) {
            addressInput.value = url;
        }
    }

    setupBackendEventListeners() {
        if (!this.checkTauriAPI()) {
            console.log('⚠️ Tauri API not available - skipping backend event listeners');
            return;
        }
        
        console.log('🔗 Setting up backend event listeners...');
        
        // Listen für Backend-Navigation-Events
        if (window.__TAURI__ && window.__TAURI__.event) {
            // WebView Navigation Events
            window.__TAURI__.event.listen('webview_navigate', (event) => {
                console.log('📡 Backend event - webview_navigate:', event.payload);
                // Backend hat Navigation gestartet
                this.setLoading(true);
                this.updateStatus('Loading...');
            });
            
            window.__TAURI__.event.listen('webview_loaded', (event) => {
                console.log('📡 Backend event - webview_loaded:', event.payload);
                // Backend hat Laden abgeschlossen
                this.setLoading(false);
                this.updateStatus('Ready');
                
                if (event.payload && event.payload.tab_id) {
                    // Update Tab-Info wenn verfügbar
                    const title = event.payload.title || 'Loaded';
                    this.updateActiveTabInfo(title, event.payload.url);
                }
            });
            
            // Internal WebView Events
            window.__TAURI__.event.listen('internal_webview_navigate', (event) => {
                console.log('📡 Backend event - internal_webview_navigate:', event.payload);
                this.setLoading(true);
            });
            
            window.__TAURI__.event.listen('internal_webview_loaded', (event) => {
                console.log('📡 Backend event - internal_webview_loaded:', event.payload);
                this.setLoading(false);
                this.updateStatus('Ready');
            });
            
            // Navigation Success Events
            window.__TAURI__.event.listen('internal_navigation_success', (event) => {
                console.log('📡 Backend event - internal_navigation_success:', event.payload);
                this.setLoading(false);
                this.updateStatus('Ready');
            });
            
            console.log('✅ Backend event listeners configured');
        } else {
            console.log('⚠️ Tauri event system not available');
        }
    }

    setupTabEventListeners() {
        // Neuer Tab Button
        const newTabBtn = document.getElementById('new-tab-btn');
        if (newTabBtn) {
            newTabBtn.addEventListener('click', () => {
                this.createNewTab();
            });
        }
        
        // Keyboard shortcuts
        document.addEventListener('keydown', (e) => {
            if (e.ctrlKey) {
                switch(e.key) {
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
                    case 'Tab':
                        e.preventDefault();
                        this.switchToNextTab();
                        break;
                }
            }
        });
    }

    setupBookmarkEventListeners() {
        console.log('🔧 Setting up bookmark event listeners...');
        
        // Bookmark-Button
        const bookmarkBtn = document.getElementById('bookmark-btn');
        console.log('🔍 bookmarkBtn found:', !!bookmarkBtn);
        
        if (bookmarkBtn) {
            console.log('✅ Setting up bookmark button listener');
            bookmarkBtn.addEventListener('click', () => {
                console.log('📚 Bookmark button clicked');
                this.showBookmarkModal();
            });
        } else {
            console.error('❌ bookmark-btn element not found!');
        }

        // Bookmark-Manager Button
        const manageBookmarksBtn = document.getElementById('manage-bookmarks-btn');
        console.log('🔍 manageBookmarksBtn found:', !!manageBookmarksBtn);
        
        if (manageBookmarksBtn) {
            console.log('✅ Setting up bookmark manager button listener');
            manageBookmarksBtn.addEventListener('click', () => {
                console.log('📚 Bookmark manager button clicked');
                this.showBookmarkManager();
            });
        } else {
            console.error('❌ manage-bookmarks-btn element not found!');
        }

        // Bookmark Modal Event-Listener
        const bookmarkModal = document.getElementById('bookmark-modal');
        const bookmarkClose = document.getElementById('bookmark-modal-close');
        const bookmarkCancel = document.getElementById('bookmark-cancel');
        const bookmarkSave = document.getElementById('bookmark-save');
        const bookmarkTitle = document.getElementById('bookmark-title');
        const bookmarkUrl = document.getElementById('bookmark-url');

        console.log('🔍 bookmarkModal found:', !!bookmarkModal);
        console.log('🔍 bookmarkClose found:', !!bookmarkClose);
        console.log('🔍 bookmarkCancel found:', !!bookmarkCancel);
        console.log('🔍 bookmarkSave found:', !!bookmarkSave);
        console.log('🔍 bookmarkTitle found:', !!bookmarkTitle);
        console.log('🔍 bookmarkUrl found:', !!bookmarkUrl);

        if (bookmarkClose) {
            bookmarkClose.addEventListener('click', () => this.hideBookmarkModal());
        }

        if (bookmarkCancel) {
            bookmarkCancel.addEventListener('click', () => this.hideBookmarkModal());
        }

        if (bookmarkSave) {
            bookmarkSave.addEventListener('click', () => this.saveBookmarkFromModal());
        }

        // Enter-Taste zum Speichern
        if (bookmarkTitle) {
            bookmarkTitle.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this.saveBookmarkFromModal();
                }
            });
        }

        if (bookmarkUrl) {
            bookmarkUrl.addEventListener('keypress', (e) => {
                if (e.key === 'Enter') {
                    this.saveBookmarkFromModal();
                }
            });
        }

        // Modal schließen beim Klick außerhalb
        if (bookmarkModal) {
            bookmarkModal.addEventListener('click', (e) => {
                if (e.target === bookmarkModal) {
                    this.hideBookmarkModal();
                }
            });
        }

        // Speichern-Button aktivieren/deaktivieren basierend auf Eingaben
        if (bookmarkTitle && bookmarkUrl && bookmarkSave) {
            const validateInputs = () => {
                const titleValid = bookmarkTitle.value.trim().length > 0;
                const urlValid = bookmarkUrl.value.trim().length > 0;
                bookmarkSave.disabled = !(titleValid && urlValid);
            };

            bookmarkTitle.addEventListener('input', validateInputs);
            bookmarkUrl.addEventListener('input', validateInputs);
            
            // Initial validation
            validateInputs();
        }

        console.log('✅ Bookmark event listeners configured');
    }

    addBookmarkManual(title, url) {
        const bookmark = {
            id: Date.now().toString(),
            title: title,
            url: this.normalizeUrl(url)
        };
        
        this.bookmarks.push(bookmark);
        this.renderBookmarks();
        this.updateStatus(`Lesezeichen hinzugefügt: ${title}`);
        
        console.log('📚 Bookmark added:', bookmark);
    }

    removeBookmark(index) {
        if (index >= 0 && index < this.bookmarks.length) {
            const removed = this.bookmarks.splice(index, 1)[0];
            this.renderBookmarks();
            this.updateStatus(`Lesezeichen entfernt: ${removed.title}`);
            console.log('📚 Bookmark removed:', removed);
        }
    }

    renderBookmarks() {
        const bookmarksContainer = document.getElementById('bookmarks-container');
        if (!bookmarksContainer) return;

        bookmarksContainer.innerHTML = '';
        
        this.bookmarks.forEach((bookmark, index) => {
            const bookmarkElement = document.createElement('button');
            bookmarkElement.className = 'bookmark-item';
            bookmarkElement.title = `${bookmark.title}\n${bookmark.url}`;
            bookmarkElement.innerHTML = `
                <span class="bookmark-title">${bookmark.title}</span>
            `;
            
            bookmarkElement.addEventListener('click', () => {
                console.log('📚 Bookmark clicked:', bookmark.url);
                this.navigateToUrl(bookmark.url);
            });
            
            bookmarksContainer.appendChild(bookmarkElement);
        });
        
        console.log(`📚 Rendered ${this.bookmarks.length} bookmarks`);
    }

    getPageTitle() {
        // Versuche Titel aus dem aktuellen Content zu extrahieren
        const contentArea = document.getElementById('content-area');
        if (contentArea) {
            const titleElement = contentArea.querySelector('title');
            if (titleElement) {
                return titleElement.textContent;
            }
        }
        
        // Fallback: Domain aus URL extrahieren
        if (this.currentUrl) {
            return this.extractDomain(this.currentUrl);
        }
        
        return 'Neue Seite';
    }

    setupSettingsEventListeners() {
        console.log('🔧 Setting up settings event listeners...');
        
        // Menu-Button (Drei Striche oben rechts)
        const menuBtn = document.getElementById('menu-btn');
        console.log('🔍 menuBtn found:', !!menuBtn);
        
        if (menuBtn) {
            console.log('✅ Setting up menu button listener');
            menuBtn.addEventListener('click', () => {
                console.log('🔧 Menu button clicked');
                this.showSettingsModal();
            });
        } else {
            console.error('❌ menu-btn element not found!');
        }

        // Settings Modal Event-Listener
        const settingsModal = document.getElementById('settings-modal');
        const settingsClose = document.getElementById('settings-modal-close');
        const settingsCancel = document.getElementById('settings-cancel');
        const settingsSave = document.getElementById('settings-save');
        
        console.log('🔍 settingsModal found:', !!settingsModal);
        console.log('🔍 settingsClose found:', !!settingsClose);
        console.log('🔍 settingsCancel found:', !!settingsCancel);
        console.log('🔍 settingsSave found:', !!settingsSave);
        
        if (settingsClose) {
            console.log('✅ Setting up settings close button listener');
            settingsClose.addEventListener('click', () => this.hideSettingsModal());
        }

        if (settingsCancel) {
            console.log('✅ Setting up settings cancel button listener');
            settingsCancel.addEventListener('click', () => this.hideSettingsModal());
        }

        if (settingsSave) {
            console.log('✅ Setting up settings save button listener');
            settingsSave.addEventListener('click', () => this.saveSettingsFromModal());
        }

        // Modal schließen beim Klick außerhalb
        if (settingsModal) {
            console.log('✅ Setting up settings modal outside click listener');
            settingsModal.addEventListener('click', (e) => {
                if (e.target === settingsModal) {
                    this.hideSettingsModal();
                }
            });
        }

        console.log('✅ Settings event listeners configured');
    }

    setupPluginEventListeners() {
        // Plugin-Statistiken aktualisieren
        const countElement = document.getElementById('plugin-count');
        if (countElement) {
            countElement.textContent = `${this.pluginManager.enabledPlugins}/${this.pluginManager.loadedPlugins}`;
        }
        
        // Plugin-Liste aktualisieren
        const listElement = document.getElementById('plugin-list');
        if (listElement) {
            listElement.innerHTML = this.createPluginListHTML();
        }

        // Plugin-Details Button
        const pluginDetailsBtn = document.getElementById('plugin-details-btn');
        if (pluginDetailsBtn) {
            pluginDetailsBtn.addEventListener('click', () => this.showPluginDetails());
        }
    }

    setupPluginDetails() {
        // Plugin-Details Modal
        const modal = document.getElementById('plugin-details-modal');
        const closeBtn = document.getElementById('plugin-details-close');
        
        if (closeBtn) {
            closeBtn.addEventListener('click', () => this.hidePluginDetails());
        }
        
        // Plugin-Details Inhalt
        const detailsContent = document.getElementById('plugin-details-content');
        if (detailsContent) {
            detailsContent.innerHTML = this.getPluginDetailsHTML();
        }
    }

    showPluginDetails() {
        // Plugin-Details Modal
        const modal = document.getElementById('plugin-details-modal');
        if (modal) {
            modal.style.display = 'block';
            console.log('📋 Plugin Details modal opened');
        }
    }

    hidePluginDetails() {
        // Plugin-Details Modal
        const modal = document.getElementById('plugin-details-modal');
        if (modal) {
            modal.style.display = 'none';
            console.log('📚 Plugin Details modal closed');
        }
    }

    getPluginDetailsHTML() {
        // Hier könnte man das HTML für die Plugin-Details zurückgeben
        return '<p>Dies ist ein Beispiel für die Plugin-Details.</p>';
    }

    showSettingsModal() {
        // Einstellungen Modal
        const modal = document.getElementById('settings-modal');
        if (modal) {
            // Lade aktuelle Settings und fülle das Modal
            this.loadSettingsIntoModal();
            modal.style.display = 'block';
            console.log('📋 Settings modal opened');
        }
    }

    hideSettingsModal() {
        // Einstellungen Modal
        const modal = document.getElementById('settings-modal');
        if (modal) {
            modal.style.display = 'none';
            console.log('📚 Settings modal closed');
        }
    }

    loadSettingsIntoModal() {
        console.log('📋 Loading settings into modal...');
        
        // Lade Settings aus localStorage oder verwende Defaults
        const settings = this.getSettings();
        
        // Homepage
        const homepageInput = document.getElementById('homepage-input');
        if (homepageInput) {
            homepageInput.value = settings.homepage || 'https://www.google.com';
        }
        
        // Search Engine
        const searchEngineInput = document.getElementById('search-engine-input');
        if (searchEngineInput) {
            searchEngineInput.value = settings.searchEngine || 'https://www.google.com/search?q=';
        }
        
        // Privacy Mode
        const privacyMode = document.getElementById('privacy-mode');
        if (privacyMode) {
            privacyMode.checked = settings.privacyMode || false;
        }
        
        // Ad Blocker
        const adBlocker = document.getElementById('ad-blocker');
        if (adBlocker) {
            adBlocker.checked = settings.adBlocker || false;
        }
        
        // JavaScript
        const javascriptEnabled = document.getElementById('javascript-enabled');
        if (javascriptEnabled) {
            javascriptEnabled.checked = settings.javascriptEnabled !== false; // Default: true
        }
        
        // Cookies
        const cookiesEnabled = document.getElementById('cookies-enabled');
        if (cookiesEnabled) {
            cookiesEnabled.checked = settings.cookiesEnabled !== false; // Default: true
        }
        
        console.log('✅ Settings loaded into modal:', settings);
    }

    saveSettingsFromModal() {
        console.log('💾 Saving settings from modal...');
        
        // Sammle alle Werte aus dem Modal
        const settings = {};
        
        // Homepage
        const homepageInput = document.getElementById('homepage-input');
        if (homepageInput) {
            settings.homepage = homepageInput.value.trim() || 'https://www.google.com';
        }
        
        // Search Engine
        const searchEngineInput = document.getElementById('search-engine-input');
        if (searchEngineInput) {
            settings.searchEngine = searchEngineInput.value.trim() || 'https://www.google.com/search?q=';
        }
        
        // Privacy Mode
        const privacyMode = document.getElementById('privacy-mode');
        if (privacyMode) {
            settings.privacyMode = privacyMode.checked;
        }
        
        // Ad Blocker
        const adBlocker = document.getElementById('ad-blocker');
        if (adBlocker) {
            settings.adBlocker = adBlocker.checked;
        }
        
        // JavaScript
        const javascriptEnabled = document.getElementById('javascript-enabled');
        if (javascriptEnabled) {
            settings.javascriptEnabled = javascriptEnabled.checked;
        }
        
        // Cookies
        const cookiesEnabled = document.getElementById('cookies-enabled');
        if (cookiesEnabled) {
            settings.cookiesEnabled = cookiesEnabled.checked;
        }
        
        // Speichere Settings
        this.saveSettings(settings);
        
        // Schließe Modal
        this.hideSettingsModal();
        
        // Zeige Bestätigung
        this.updateStatus('⚙️ Einstellungen gespeichert');
        
        console.log('✅ Settings saved:', settings);
    }

    getSettings() {
        try {
            const savedSettings = localStorage.getItem('ora-browser-settings');
            if (savedSettings) {
                return JSON.parse(savedSettings);
            }
        } catch (error) {
            console.error('❌ Error loading settings:', error);
        }
        
        // Default Settings
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
            
            // Aktualisiere interne Settings
            this.settings = { ...this.settings, ...settings };
            
            return true;
        } catch (error) {
            console.error('❌ Error saving settings:', error);
            this.updateStatus('❌ Fehler beim Speichern der Einstellungen');
            return false;
        }
    }

    showBookmarkModal() {
        const modal = document.getElementById('bookmark-modal');
        const titleInput = document.getElementById('bookmark-title');
        const urlInput = document.getElementById('bookmark-url');
        
        if (modal) {
            // Pre-fill with current page info
            if (titleInput) {
                titleInput.value = this.getPageTitle() || 'Neue Seite';
            }
            if (urlInput) {
                urlInput.value = this.currentUrl || '';
            }
            
            modal.style.display = 'flex';
            if (titleInput) titleInput.focus();
            
            console.log('📚 Bookmark modal opened');
        }
    }

    hideBookmarkModal() {
        const modal = document.getElementById('bookmark-modal');
        if (modal) {
            modal.style.display = 'none';
            console.log('📚 Bookmark modal closed');
        }
    }

    saveBookmarkFromModal() {
        const titleInput = document.getElementById('bookmark-title');
        const urlInput = document.getElementById('bookmark-url');
        
        if (titleInput && urlInput) {
            const title = titleInput.value.trim();
            const url = urlInput.value.trim();
            
            if (title && url) {
                this.addBookmarkManual(title, url);
                this.hideBookmarkModal();
                
                // Clear inputs
                titleInput.value = '';
                urlInput.value = '';
            }
        }
    }

    showBookmarkManager() {
        // Create a simple bookmark manager overlay
        const existingManager = document.getElementById('bookmark-manager');
        if (existingManager) {
            existingManager.remove();
        }

        const manager = document.createElement('div');
        manager.id = 'bookmark-manager';
        manager.style.cssText = `
            position: fixed;
            top: 0;
            left: 0;
            width: 100%;
            height: 100%;
            background: rgba(0,0,0,0.8);
            z-index: 10000;
            display: flex;
            justify-content: center;
            align-items: center;
        `;

        const content = document.createElement('div');
        content.style.cssText = `
            background: white;
            padding: 20px;
            border-radius: 10px;
            max-width: 600px;
            max-height: 80%;
            overflow-y: auto;
            box-shadow: 0 4px 20px rgba(0,0,0,0.3);
        `;

        let bookmarksList = '<h2>📚 Lesezeichen verwalten</h2>';
        bookmarksList += '<div style="margin-bottom: 20px;">';
        
        this.bookmarks.forEach((bookmark, index) => {
            bookmarksList += `
                <div style="display: flex; justify-content: space-between; align-items: center; padding: 10px; border-bottom: 1px solid #eee;">
                    <div style="flex: 1;">
                        <strong>${bookmark.title}</strong><br>
                        <small style="color: #666;">${bookmark.url}</small>
                    </div>
                    <div style="display: flex; gap: 10px;">
                        <button onclick="window.oraBrowser.navigateToUrl('${bookmark.url}'); document.getElementById('bookmark-manager').remove();" 
                                style="background: #28a745; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer;">
                            Öffnen
                        </button>
                        <button onclick="window.oraBrowser.removeBookmark(${index}); document.getElementById('bookmark-manager').remove(); window.oraBrowser.showBookmarkManager();" 
                                style="background: #dc3545; color: white; border: none; padding: 5px 10px; border-radius: 3px; cursor: pointer;">
                            Löschen
                        </button>
                    </div>
                </div>
            `;
        });
        
        bookmarksList += '</div>';
        bookmarksList += '<div style="text-align: center; margin-top: 20px;">';
        bookmarksList += '<button onclick="document.getElementById(\'bookmark-manager\').remove();" style="background: #007bff; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer; margin-right: 10px;">Schließen</button>';
        bookmarksList += '<button onclick="window.oraBrowser.showBookmarkModal(); document.getElementById(\'bookmark-manager\').remove();" style="background: #28a745; color: white; border: none; padding: 10px 20px; border-radius: 5px; cursor: pointer;">Neues Lesezeichen</button>';
        bookmarksList += '</div>';
        
        content.innerHTML = bookmarksList;
        manager.appendChild(content);
        document.body.appendChild(manager);

        // Close when clicking outside
        manager.addEventListener('click', (e) => {
            if (e.target === manager) {
                manager.remove();
            }
        });

        console.log('📚 Bookmark manager opened');
    }

    // Backend-Navigation verwenden
    async navigateToUrl(url, updateTab = true, addToHistoryFlag = true) {
        if (!url || url.trim() === '') {
            console.log('❌ Empty URL provided');
            return false;
        }
        
        const normalizedUrl = this.normalizeUrl(url.trim());
        console.log('🌐 Navigating to:', normalizedUrl);
        
        // Update Tab-Info wenn gewünscht
        if (updateTab && this.activeTabId) {
            this.updateActiveTabInfo('Loading...', normalizedUrl);
        }
        
        // Zur History hinzufügen
        if (addToHistoryFlag) {
            this.addToHistory(normalizedUrl);
        }
        
        // Update URL-Eingabe
        this.updateUrlInput(normalizedUrl);
        this.setLoading(true);
        
        try {
            // 🚀 VERBESSERTE NAVIGATION MIT FALLBACK-STRATEGIEN
            
            // Spezielle Behandlung für Google-URLs
            if (normalizedUrl.includes('google.com') || normalizedUrl.includes('google.de')) {
                console.log('🔍 Google URL detected, using optimized navigation');
                return await this.handleGoogleNavigation(normalizedUrl);
            }
            
            // 1. Prüfe ob Proxy-Server verfügbar ist
            let proxyAvailable = false;
            try {
                console.log('🔍 Testing proxy server availability...');
                const proxyTest = await fetch('http://localhost:3030/health', { 
                    method: 'GET',
                    signal: AbortSignal.timeout(3000)
                });
                proxyAvailable = proxyTest.ok;
                console.log('🔍 Proxy server status:', proxyAvailable ? 'Available ✅' : 'Unavailable ❌');
                
                if (proxyAvailable) {
                    const healthData = await proxyTest.json();
                    console.log('🔍 Proxy server health:', healthData);
                }
            } catch (e) {
                console.log('🔍 Proxy server not available:', e.message);
                console.log('🔍 Falling back to direct navigation');
                proxyAvailable = false;
            }
            
            // 2. Versuche Proxy-Navigation wenn verfügbar
            if (proxyAvailable) {
                try {
                    const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(normalizedUrl)}`;
                    console.log('🔄 Trying proxy strategy:', proxyUrl);
                    
                    const response = await fetch(proxyUrl, {
                        method: 'GET',
                        headers: {
                            'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                            'Accept-Language': 'de-DE,de;q=0.9,en;q=0.8',
                            'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36'
                        }
                    });
                    
                    console.log('🔄 Proxy response status:', response.status, response.statusText);
                    
                    if (response.ok) {
                        const content = await response.text();
                        console.log(`✅ Content loaded via proxy: ${content.length} bytes`);
                        console.log('🔧 Content preview:', content.substring(0, 200) + '...');
                        
                        const optimizedContent = this.injectCORSHeaders(content);
                        console.log('🔧 About to display content...');
                        this.displayContent(optimizedContent);
                        console.log('🔧 Content display completed');
                        this.setLoading(false);
                        
                        // Update current URL
                        this.currentUrl = normalizedUrl;
                        console.log('🔧 Current URL updated to:', this.currentUrl);
                        return true;
                    } else {
                        console.log('❌ Proxy response not OK:', response.status, response.statusText);
                    }
                } catch (proxyError) {
                    console.log('🔄 Proxy failed, trying direct navigation:', proxyError.message);
                    console.error('🔄 Proxy error details:', proxyError);
                }
            } else {
                console.log('🔄 Proxy not available, skipping proxy strategy');
            }
            
            // 3. Fallback: Direkte iframe-Einbettung
            console.log('🔄 Using direct iframe navigation');
            const iframeContent = this.createUltimateIframeFallback(normalizedUrl);
            this.displayContent(iframeContent);
            this.setLoading(false);
            return true;
            
        } catch (error) {
            console.error('❌ All navigation strategies failed:', error);
            this.showUltimateErrorPage(normalizedUrl, error);
            this.setLoading(false);
            return false;
        }
    }

    displayContent(htmlContent) {
        console.log('📄 Displaying content...');
        console.log('📄 HTML content length:', htmlContent ? htmlContent.length : 'null/undefined');
        
        const contentArea = document.getElementById('content-area');
        const welcomeScreen = document.getElementById('welcome-screen');
        
        console.log('📄 Content area found:', !!contentArea);
        console.log('📄 Welcome screen found:', !!welcomeScreen);
        
        if (contentArea) {
            // Verstecke Welcome Screen falls vorhanden
            if (welcomeScreen) {
                welcomeScreen.style.display = 'none';
                welcomeScreen.style.visibility = 'hidden';
                console.log('📄 Welcome screen hidden');
            }
            
            // Erstelle Content-Container falls nicht vorhanden
            let contentContainer = document.getElementById('content-container');
            console.log('📄 Existing content container found:', !!contentContainer);
            
            if (!contentContainer) {
                contentContainer = document.createElement('div');
                contentContainer.id = 'content-container';
                contentContainer.style.cssText = `
                    width: 100%;
                    height: 100%;
                    overflow: auto;
                    background: white;
                    position: relative;
                    z-index: 1000;
                `;
                contentArea.appendChild(contentContainer);
                console.log('📄 New content container created and appended');
            }
            
            // Zeige Content
            if (htmlContent && htmlContent.trim()) {
                contentContainer.innerHTML = htmlContent;
                contentContainer.style.display = 'block';
                contentContainer.style.visibility = 'visible';
                console.log('📄 Content set to container, display:', contentContainer.style.display);
                console.log('📄 Content container visibility:', contentContainer.style.visibility);
                console.log('📄 Content displayed successfully, HTML length:', htmlContent.length);
            } else {
                console.error('❌ HTML content is empty or invalid!');
            }
        } else {
            console.error('❌ Content area not found!');
            // Debug: Liste alle verfügbaren Elemente auf
            console.log('📄 Available elements with IDs:');
            const allElements = document.querySelectorAll('[id]');
            allElements.forEach(el => console.log('  -', el.id, el.tagName));
        }
    }

    updateStatus(message) {
        const statusText = document.getElementById('status-text');
        if (statusText) {
            statusText.textContent = message;
        }
        console.log(`📊 Status: ${message}`);
    }

    setLoading(loading) {
        this.isLoading = loading;
        const loadingIndicator = document.getElementById('loading-indicator');
        
        if (loadingIndicator) {
            loadingIndicator.style.display = loading ? 'flex' : 'none';
        }
        
        // Update reload button
        const reloadBtn = document.getElementById('reload-btn');
        if (reloadBtn) {
            reloadBtn.disabled = loading;
            reloadBtn.style.opacity = loading ? '0.5' : '1';
        }
    }

    createUltimateIframeFallback(url) {
        console.log('🔧 Creating ultimate iframe fallback for:', url);
        
        const fallbackContent = `
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-Frame-Options" content="ALLOWALL">
    <meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob: filesystem:; script-src * 'unsafe-inline' 'unsafe-eval' data: blob: filesystem:; style-src * 'unsafe-inline' data: blob:; img-src * data: blob: filesystem:; font-src * data: blob: filesystem:; connect-src * data: blob: filesystem:; media-src * data: blob: filesystem:; object-src * data: blob: filesystem:; child-src * data: blob: filesystem:; frame-src * data: blob: filesystem:; worker-src * data: blob: filesystem:; frame-ancestors *; form-action *; base-uri *; manifest-src *;">
    <title>Ora Browser - ${this.extractDomain(url)}</title>
    <style>
        body {
            margin: 0;
            padding: 0;
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #f5f5f5;
        }
        .iframe-container {
            width: 100%;
            height: 100vh;
            position: relative;
            overflow: hidden;
        }
        .iframe-fallback {
            width: 100%;
            height: 100%;
            border: none;
            background: white;
        }
        .fallback-header {
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 8px 16px;
            font-size: 12px;
            position: fixed;
            top: 0;
            left: 0;
            right: 0;
            z-index: 10000;
            box-shadow: 0 2px 4px rgba(0,0,0,0.1);
        }
        .fallback-content {
            margin-top: 40px;
            height: calc(100vh - 40px);
        }
        .retry-options {
            position: fixed;
            bottom: 20px;
            right: 20px;
            z-index: 10001;
        }
        .retry-btn {
            background: rgba(102, 126, 234, 0.9);
            color: white;
            border: none;
            padding: 8px 16px;
            border-radius: 6px;
            cursor: pointer;
            margin-left: 8px;
            font-size: 12px;
            transition: all 0.3s ease;
        }
        .retry-btn:hover {
            background: rgba(102, 126, 234, 1);
            transform: translateY(-1px);
        }
    </style>
</head>
<body>
    <div class="fallback-header">
        🌐 Ora Browser - Ultimative Iframe-Fallback für: ${url}
    </div>
    
    <div class="fallback-content">
        <div class="iframe-container">
            <iframe 
                class="iframe-fallback" 
                src="${url}"
                sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-popups-to-escape-sandbox allow-presentation allow-top-navigation allow-top-navigation-by-user-activation allow-modals allow-downloads allow-pointer-lock allow-orientation-lock"
                allow="accelerometer; camera; geolocation; gyroscope; magnetometer; microphone; payment; usb; fullscreen; display-capture; web-share; clipboard-read; clipboard-write"
                loading="eager"
                referrerpolicy="no-referrer-when-downgrade">
            </iframe>
        </div>
    </div>
    
    <div class="retry-options">
        <button class="retry-btn" onclick="location.reload()">🔄 Neu laden</button>
        <button class="retry-btn" onclick="window.open('${url}', '_blank')">🌐 Extern öffnen</button>
        <button class="retry-btn" onclick="parent.oraBrowser.navigateToUrl('${url.replace('https://', 'http://')}')">🔓 HTTP versuchen</button>
    </div>
    
    <script>
        // Ultimative Iframe-Fixes
        (function() {
            console.log('🔧 Applying ultimate iframe fixes...');
            
            const iframe = document.querySelector('.iframe-fallback');
            
            // Entferne alle Iframe-Beschränkungen
            iframe.onload = function() {
                try {
                    // Versuche Zugriff auf Iframe-Content
                    const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
                    if (iframeDoc) {
                        console.log('✅ Iframe content accessible');
                        
                        // Injiziere ultimative Fixes in Iframe
                        const script = iframeDoc.createElement('script');
                        script.textContent = \`
                            // Ultimative Iframe-Content-Fixes
                            (function() {
                                const style = document.createElement('style');
                                style.textContent = '* { pointer-events: auto !important; cursor: auto !important; }';
                                document.head.appendChild(style);
                                
                                // Aktiviere alle Elemente
                                const elements = document.querySelectorAll('*');
                                elements.forEach(el => {
                                    el.style.pointerEvents = 'auto';
                                    el.disabled = false;
                                    el.removeAttribute('disabled');
                                });
                                
                                // Link-Handling für Tab-Navigation
                                const links = document.querySelectorAll('a[href]');
                                links.forEach(link => {
                                    link.addEventListener('click', function(e) {
                                        const href = this.getAttribute('href');
                                        if (href && !href.startsWith('#') && !href.startsWith('javascript:')) {
                                            e.preventDefault();
                                            
                                            // Vollständige URL erstellen
                                            const fullUrl = href.startsWith('http') ? href : 
                                                           href.startsWith('/') ? window.location.origin + href :
                                                           window.location.href.substring(0, window.location.href.lastIndexOf('/') + 1) + href;
                                            
                                            // An Parent-Browser weiterleiten
                                            if (window.parent && window.parent.oraBrowser) {
                                                if (e.ctrlKey || e.metaKey) {
                                                    // Neuer Tab
                                                    window.parent.oraBrowser.createNewTab('Loading...', fullUrl);
                                                    window.parent.oraBrowser.navigateToUrl(fullUrl);
                                                } else {
                                                    // Aktueller Tab
                                                    window.parent.oraBrowser.navigateToUrl(fullUrl);
                                                }
                                            } else {
                                                // Fallback: Normale Navigation
                                                window.location.href = fullUrl;
                                            }
                                        }
                                    });
                                });
                                
                                console.log('✅ Iframe content fixes applied with link handling');
                            })();
                        \`;
                        iframeDoc.head.appendChild(script);
                    }
                } catch (e) {
                    console.log('🔄 Cross-origin iframe, applying external fixes');
                    
                    // Externe Fixes für Cross-Origin Iframes
                    iframe.style.pointerEvents = 'auto';
                    iframe.style.cursor = 'auto';
                }
            };
            
            // Fehlerbehandlung
            iframe.onerror = function() {
                console.log('❌ Iframe loading failed, showing error message');
                iframe.style.display = 'none';
                
                const errorDiv = document.createElement('div');
                errorDiv.innerHTML = \`
                    <div style="text-align: center; padding: 40px; color: #666;">
                        <h2>❌ Iframe-Fallback fehlgeschlagen</h2>
                        <p>Die Website konnte nicht in einem Iframe geladen werden.</p>
                        <button onclick="window.open('${url}', '_blank')" style="background: #667eea; color: white; border: none; padding: 12px 24px; border-radius: 6px; cursor: pointer;">
                            🌐 In neuem Fenster öffnen
                        </button>
                    </div>
                \`;
                document.querySelector('.fallback-content').appendChild(errorDiv);
            };
            
            console.log('✅ Ultimate iframe fallback initialized');
        })();
    </script>
</body>
</html>
        `;
        
        this.displayContent(fallbackContent);
        this.addToHistory(url);
        this.updateStatus(`Iframe-Fallback: ${url}`);
        
        if (this.activeTabId) {
            const title = `${this.extractDomain(url)} (Iframe)`;
            this.updateActiveTabInfo(title, url);
        }
    }

    showUltimateErrorPage(url, error) {
        console.log('❌ Showing ultimate error page for:', url, error);
        
        const errorContent = `
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Ora Browser - Verbindungsfehler</title>
    <style>
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 40px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }
        .error-container {
            background: rgba(255,255,255,0.1);
            padding: 40px;
            border-radius: 20px;
            backdrop-filter: blur(10px);
            box-shadow: 0 8px 32px rgba(0,0,0,0.3);
            text-align: center;
            max-width: 600px;
            width: 100%;
        }
        .error-icon {
            font-size: 64px;
            margin-bottom: 20px;
        }
        .error-title {
            font-size: 28px;
            margin-bottom: 16px;
            font-weight: 600;
        }
        .error-message {
            font-size: 16px;
            margin-bottom: 30px;
            opacity: 0.9;
            line-height: 1.5;
        }
        .url-display {
            background: rgba(255,255,255,0.1);
            padding: 12px;
            border-radius: 8px;
            margin: 20px 0;
            font-family: monospace;
            word-break: break-all;
            font-size: 14px;
        }
        .retry-options {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 16px;
            margin-top: 30px;
        }
        .retry-btn {
            background: rgba(255,255,255,0.2);
            border: 2px solid rgba(255,255,255,0.3);
            color: white;
            padding: 16px 24px;
            border-radius: 10px;
            cursor: pointer;
            transition: all 0.3s ease;
            text-decoration: none;
            display: block;
            font-weight: 500;
            font-size: 14px;
        }
        .retry-btn:hover {
            background: rgba(255,255,255,0.3);
            border-color: rgba(255,255,255,0.5);
            transform: translateY(-2px);
        }
        .error-details {
            background: rgba(0,0,0,0.2);
            padding: 20px;
            border-radius: 10px;
            margin: 20px 0;
            font-family: monospace;
            font-size: 12px;
            text-align: left;
            word-break: break-all;
            max-height: 200px;
            overflow-y: auto;
        }
    </style>
</head>
<body>
    <div class="error-container">
        <div class="error-icon">🌐❌</div>
        <h1 class="error-title">Ultimativer Verbindungsfehler</h1>
        <p class="error-message">
            Alle verfügbaren Verbindungsstrategien sind fehlgeschlagen. 
            Ora Browser hat verschiedene Methoden versucht, aber die Website ist nicht erreichbar.
        </p>
        
        <div class="url-display">
            <strong>URL:</strong> ${url}
        </div>
        
        <div class="error-details">
            <strong>Fehlerdetails:</strong><br>
            ${error.message || error.toString()}
        </div>
        
        <div class="retry-options">
            <button class="retry-btn" onclick="parent.oraBrowser.navigateToUrl('${url}')">
                🔄 Erneut versuchen
            </button>
            <button class="retry-btn" onclick="window.open('${url}', '_blank')">
                🌐 In neuem Fenster öffnen
            </button>
            <button class="retry-btn" onclick="parent.oraBrowser.navigateToUrl('${url.replace('https://', 'http://')}')">
                🔓 HTTP-Version versuchen
            </button>
            <button class="retry-btn" onclick="parent.oraBrowser.navigateToUrl('${url.replace('www.', 'm.')}')">
                📱 Mobile Version versuchen
            </button>
            <button class="retry-btn" onclick="parent.oraBrowser.createUltimateIframeFallback('${url}')">
                🖼️ Iframe-Fallback versuchen
            </button>
            <button class="retry-btn" onclick="parent.oraBrowser.navigateToUrl('https://web.archive.org/web/*/${url}')">
                📚 Wayback Machine
            </button>
        </div>
        
        <p style="margin-top: 30px; opacity: 0.7; font-size: 14px;">
            🛡️ Ora Browser - Ultimative Verbindungsstrategien erschöpft
        </p>
    </div>
    
    <script>
        console.log('❌ Ultimate error page loaded for:', '${url}');
        
        // Versuche automatische Wiederherstellung nach 5 Sekunden
        setTimeout(() => {
            console.log('🔄 Attempting automatic recovery...');
            if (parent && parent.oraBrowser) {
                parent.oraBrowser.navigateToUrl('${url.replace('https://', 'http://')}');
            }
        }, 5000);
    </script>
</body>
</html>
        `;
        
        this.displayContent(errorContent);
        this.updateStatus(`Fehler: ${url}`);
        
        if (this.activeTabId) {
            this.updateActiveTabInfo(`Fehler - ${this.extractDomain(url)}`, url);
        }
    }

    extractTitleFromContent(htmlContent) {
        try {
            const titleMatch = htmlContent.match(/<title[^>]*>([^<]+)<\/title>/i);
            return titleMatch ? titleMatch[1].trim() : null;
        } catch (e) {
            return null;
        }
    }

    // 🇩🇪 SPEZIELLE GOOGLE DEUTSCHLAND BEHANDLUNG
    async handleGoogleNavigation(url) {
        console.log('🔍 Handling Google navigation for:', url);
        
        try {
            // Versuche zuerst Proxy-Navigation für Google
            const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
            console.log('🔄 Trying Google proxy navigation:', proxyUrl);
            
            const response = await fetch(proxyUrl, {
                method: 'GET',
                headers: {
                    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8',
                    'Accept-Language': 'de-DE,de;q=0.9,en;q=0.8',
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36'
                }
            });
            
            if (response.ok) {
                const content = await response.text();
                console.log(`✅ Google content loaded via proxy: ${content.length} bytes`);
                
                // Optimiere Google-Content für bessere Interaktivität
                const optimizedContent = this.optimizeGoogleForInteraction(content, url);
                this.displayContent(optimizedContent);
                this.setLoading(false);
                return true;
            }
        } catch (proxyError) {
            console.log('🔄 Google proxy failed, trying direct iframe:', proxyError.message);
        }
        
        // Fallback: Optimierter Google-Iframe
        console.log('🔄 Using optimized Google iframe');
        const googleIframe = this.createOptimizedGoogleIframe(url);
        this.displayContent(googleIframe);
        this.setLoading(false);
        return true;
    }

    optimizeGoogleForInteraction(content, url) {
        console.log('🔧 Optimizing Google content for interaction');
        
        // Entferne problematische CSP-Header und füge Interaktivitäts-Fixes hinzu
        let optimized = content
            .replace(/<meta[^>]*http-equiv=["']Content-Security-Policy["'][^>]*>/gi, '')
            .replace(/<meta[^>]*http-equiv=["']X-Frame-Options["'][^>]*>/gi, '')
            .replace(/X-Frame-Options:[^;]*;?/gi, '')
            .replace(/Content-Security-Policy:[^;]*;?/gi, '');
        
        // 🚀 ULTIMATIVE GOOGLE-ENTER-KEY-FIXES + INTERAKTIVITÄT
        const interactionScript = `
            <script>
                // 🔍 SOFORTIGE ENTER-KEY-BEHANDLUNG FÜR GOOGLE
                document.addEventListener('DOMContentLoaded', function() {
                    console.log('🔍 DOM loaded - setting up immediate Enter-Key handling');
                    setupImmediateEnterKeyHandling();
                });
                
                // Sofortige Ausführung auch ohne DOMContentLoaded
                if (document.readyState === 'loading') {
                    document.addEventListener('DOMContentLoaded', setupImmediateEnterKeyHandling);
                } else {
                    setupImmediateEnterKeyHandling();
                }
                
                function setupImmediateEnterKeyHandling() {
                    console.log('🔍 Setting up immediate Enter-Key handling...');
                    
                    // ENTER-TASTE-BEHANDLUNG NUR FÜR GOOGLE-SEITEN AGGRESSIV
                    document.addEventListener('keydown', function(e) {
                        if (e.key === 'Enter' || e.keyCode === 13) {
                            var target = e.target;
                            console.log('🔍 Enter pressed on element:', target.tagName, target.name, target.className, target.id);
                            
                            // Prüfe ob es ein Google-Suchfeld ist (ERWEITERTE ERKENNUNG)
                            var isGoogleSearchField = target.tagName === 'INPUT' && (
                                target.name === 'q' ||
                                target.className.includes('gLFyf') ||
                                target.id === 'APjFqb' ||
                                (target.type === 'search' && (window.location.href.includes('google.com') || document.title.includes('Google'))) ||
                                target.getAttribute('role') === 'combobox' ||
                                target.getAttribute('aria-label') && target.getAttribute('aria-label').includes('Suche') ||
                                target.title && (target.title.includes('Suche') || target.title.includes('Search')) ||
                                target.placeholder && (target.placeholder.includes('Suche') || target.placeholder.includes('Search'))
                            );
                            
                            // NUR AUF GOOGLE-SEITEN AGGRESSIV SEIN
                            if (isGoogleSearchField && (window.location.href.includes('google.com') || document.title.includes('Google'))) {
                                console.log('🔍 🎯 ENTER IN GOOGLE SEARCH FIELD DETECTED!');
                                
                                // STOPPE ALLE GOOGLE-EVENTS SOFORT
                                e.preventDefault();
                                e.stopPropagation();
                                e.stopImmediatePropagation();
                                
                                var query = target.value.trim();
                                if (query) {
                                    var searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                                    console.log('🔍 🚀 FORCING IMMEDIATE GOOGLE SEARCH:', searchUrl);
                                    
                                    // SOFORTIGE NAVIGATION MIT TIMEOUT-SCHUTZ
                                    setTimeout(function() {
                                        try {
                                            if (window.parent && window.parent.oraBrowser && typeof window.parent.oraBrowser.navigateToUrl === 'function') {
                                                console.log('🔍 Using parent navigation');
                                                window.parent.oraBrowser.navigateToUrl(searchUrl);
                                            } else if (window.top && window.top.oraBrowser && typeof window.top.oraBrowser.navigateToUrl === 'function') {
                                                console.log('🔍 Using top navigation');
                                                window.top.oraBrowser.navigateToUrl(searchUrl);
                                            } else {
                                                console.log('🔍 Using postMessage navigation');
                                                window.parent.postMessage({
                                                    type: 'navigate',
                                                    url: searchUrl
                                                }, '*');
                                            }
                                        } catch (navError) {
                                            console.log('🔍 Navigation error:', navError);
                                            window.location.href = searchUrl;
                                        }
                                    }, 10); // Sehr kurzer Timeout um Google-Events zu überschreiben
                                } else {
                                    console.log('🔍 No search query entered');
                                }
                                
                                return false;
                            }
                        }
                    }, true);
                    
                    // KEYPRESS-BACKUP NUR FÜR GOOGLE-SEITEN
                    if (window.location.href.includes('google.com') || document.title.includes('Google')) {
                        document.addEventListener('keypress', function(e) {
                            if (e.key === 'Enter' || e.keyCode === 13) {
                                var target = e.target;
                                
                                var isGoogleSearchField = target.tagName === 'INPUT' && (
                                    target.name === 'q' ||
                                    target.className.includes('gLFyf') ||
                                    target.id === 'APjFqb' ||
                                    target.type === 'search'
                                );
                                
                                if (isGoogleSearchField) {
                                    console.log('🔍 🎯 KEYPRESS BACKUP - ENTER IN GOOGLE SEARCH!');
                                    e.preventDefault();
                                    e.stopPropagation();
                                    e.stopImmediatePropagation();
                                    
                                    var query = target.value.trim();
                                    if (query) {
                                        var searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                                        console.log('🔍 🚀 KEYPRESS BACKUP SEARCH:', searchUrl);
                                        
                                        setTimeout(function() {
                                            try {
                                                if (window.parent && window.parent.oraBrowser) {
                                                    window.parent.oraBrowser.navigateToUrl(searchUrl);
                                                } else {
                                                    window.parent.postMessage({
                                                        type: 'navigate',
                                                        url: searchUrl
                                                    }, '*');
                                                }
                                            } catch (navError) {
                                                window.location.href = searchUrl;
                                            }
                                        }, 5);
                                    }
                                    
                                    return false;
                                }
                            }
                        }, true);
                    }
                    
                    // FORM-SUBMIT-ABFANGEN NUR FÜR GOOGLE-SEITEN
                    if (window.location.href.includes('google.com') || document.title.includes('Google')) {
                        document.addEventListener('submit', function(e) {
                            console.log('🔍 Google form submit detected:', e.target);
                            
                            var form = e.target;
                            var searchInput = form.querySelector('input[name="q"], .gLFyf, #APjFqb');
                            
                            if (searchInput) {
                                console.log('🔍 🎯 GOOGLE FORM SUBMIT DETECTED!');
                                e.preventDefault();
                                e.stopPropagation();
                                e.stopImmediatePropagation();
                                
                                var query = searchInput.value.trim();
                                if (query) {
                                    var searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                                    console.log('🔍 🚀 GOOGLE FORM SUBMIT SEARCH:', searchUrl);
                                    
                                    setTimeout(function() {
                                        try {
                                            if (window.parent && window.parent.oraBrowser) {
                                                window.parent.oraBrowser.navigateToUrl(searchUrl);
                                            } else {
                                                window.parent.postMessage({
                                                    type: 'navigate',
                                                    url: searchUrl
                                                }, '*');
                                            }
                                        } catch (navError) {
                                            window.location.href = searchUrl;
                                        }
                                    }, 5);
                                }
                                
                                return false;
                            }
                        }, true);
                    }
                    
                    // BUTTON-CLICK-ABFANGEN NUR FÜR GOOGLE-SEITEN
                    if (window.location.href.includes('google.com') || document.title.includes('Google')) {
                        document.addEventListener('click', function(e) {
                            var target = e.target;
                            console.log('🔍 Google click detected on:', target.tagName, target.name, target.value, target.className);
                            
                            // Google Search Button (nur auf Google-Seiten)
                            if (target.tagName === 'INPUT' && (
                                target.name === 'btnK' ||
                                target.name === 'btnI' ||
                                (target.value && (target.value.includes('Suche') || target.value.includes('Search') || target.value.includes('Glück')))
                            )) {
                                console.log('🔍 🎯 GOOGLE SEARCH BUTTON CLICKED!');
                                e.preventDefault();
                                e.stopPropagation();
                                e.stopImmediatePropagation();
                                
                                var searchInput = document.querySelector('input[name="q"], .gLFyf, #APjFqb');
                                if (searchInput && searchInput.value.trim()) {
                                    var query = searchInput.value.trim();
                                    var searchUrl = target.name === 'btnI' ? 
                                        'https://www.google.com/search?q=' + encodeURIComponent(query) + '&btnI=1' :
                                        'https://www.google.com/search?q=' + encodeURIComponent(query);
                                    console.log('🔍 🚀 GOOGLE BUTTON CLICK SEARCH:', searchUrl);
                                    
                                    setTimeout(function() {
                                        try {
                                            if (window.parent && window.parent.oraBrowser) {
                                                window.parent.oraBrowser.navigateToUrl(searchUrl);
                                            } else {
                                                window.parent.postMessage({
                                                    type: 'navigate',
                                                    url: searchUrl
                                                }, '*');
                                            }
                                        } catch (navError) {
                                            window.location.href = searchUrl;
                                        }
                                    }, 5);
                                }
                                
                                return false;
                            }
                        }, true);
                    }
                    
                    console.log('✅ Global Enter-Key handler installed');
                    console.log('✅ Form submit handler installed');
                    console.log('✅ Button click handler installed');
                }
                
                // Google-spezifische Interaktivitäts-Fixes + Enter-Key
                (function() {
                    console.log('🔧 Applying Google interaction fixes + Enter-Key support...');
                    
                    // 🎲 EINFACHER AUF-GUT-GLÜCK FIX
                    function fixLuckyButton() {
                        console.log('🎲 Looking for Auf gut Glück button...');
                        
                        var luckyButtons = document.querySelectorAll('input[name="btnI"]');
                        console.log('🎲 Found ' + luckyButtons.length + ' lucky buttons');
                        
                        for (var i = 0; i < luckyButtons.length; i++) {
                            var btn = luckyButtons[i];
                            if (!btn.dataset.oraFixed) {
                                console.log('🎲 🍀 Fixing lucky button:', btn);
                                
                                btn.addEventListener('click', function(e) {
                                    console.log('🎲 🚨 LUCKY BUTTON CLICKED!');
                                    e.preventDefault();
                                    e.stopPropagation();
                                    
                                    var searchInput = document.querySelector('input[name="q"]');
                                    if (searchInput && searchInput.value.trim()) {
                                        var query = searchInput.value.trim();
                                        var luckyUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query) + '&btnI=1';
                                        console.log('🎲 🍀 Lucky URL:', luckyUrl);
                                        
                                        // 🚀 VERWENDE ORA BROWSER NAVIGATION SYSTEM
                                        try {
                                            // Versuche über das parent window (Ora Browser)
                                            if (window.parent && window.parent.oraBrowser) {
                                                console.log('🎲 Using Ora Browser navigation system');
                                                window.parent.oraBrowser.navigateToUrl(luckyUrl);
                                            } else if (window.top && window.top.oraBrowser) {
                                                console.log('🎲 Using top Ora Browser navigation');
                                                window.top.oraBrowser.navigateToUrl(luckyUrl);
                                            } else if (window.parent && window.parent !== window) {
                                                console.log('🎲 Using postMessage navigation');
                                                window.parent.postMessage({
                                                    type: 'navigate',
                                                    url: luckyUrl
                                                }, '*');
                                            } else {
                                                console.log('🎲 Fallback: Direct navigation');
                                                window.location.href = luckyUrl;
                                            }
                                        } catch (navError) {
                                            console.log('🎲 Navigation error, using fallback:', navError);
                                            window.location.href = luckyUrl;
                                        }
                                    } else {
                                        console.log('🎲 ❌ No search query');
                                    }
                                    
                                    return false;
                                }, true);
                                
                                btn.dataset.oraFixed = 'true';
                                console.log('✅ Lucky button fixed');
                            }
                        }
                    }
                    
                    // 🔍 ENTER-KEY FIX FÜR GOOGLE-SUCHE
                    function fixGoogleEnterKey() {
                        console.log('🔍 Setting up Google Enter-Key...');
                        
                        // Erweiterte Selektor-Liste für alle Google-Suchfelder
                        var searchInputs = document.querySelectorAll([
                            'input[name="q"]',
                            'input[title*="Suche"]', 
                            'input[title*="Search"]',
                            '.gLFyf',
                            '#APjFqb',
                            'input[type="search"]',
                            'textarea[name="q"]',
                            '.a4bIc input',
                            '.RNNXgb',
                            'input.gLFyf.gsfi'
                        ].join(', '));
                        
                        console.log('🔍 Found ' + searchInputs.length + ' search inputs');
                        
                        for (var i = 0; i < searchInputs.length; i++) {
                            var input = searchInputs[i];
                            if (!input.dataset.oraEnterFixed) {
                                console.log('🔍 Adding Enter-Key handler to search input:', input.className || input.name);
                                
                                // Keydown Event
                                input.addEventListener('keydown', function(e) {
                                    if (e.key === 'Enter' || e.keyCode === 13) {
                                        console.log('🔍 🎯 ENTER PRESSED IN GOOGLE SEARCH!');
                                        e.preventDefault();
                                        e.stopPropagation();
                                        
                                        var query = this.value.trim();
                                        if (query) {
                                            var searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                                            console.log('🔍 Search URL:', searchUrl);
                                            
                                            // 🚀 VERWENDE ORA BROWSER NAVIGATION SYSTEM
                                            try {
                                                if (window.parent && window.parent.oraBrowser && typeof window.parent.oraBrowser.navigateToUrl === 'function') {
                                                    console.log('🔍 Using Ora Browser navigation system');
                                                    window.parent.oraBrowser.navigateToUrl(searchUrl);
                                                } else if (window.top && window.top.oraBrowser && typeof window.top.oraBrowser.navigateToUrl === 'function') {
                                                    console.log('🔍 Using top Ora Browser navigation');
                                                    window.top.oraBrowser.navigateToUrl(searchUrl);
                                                } else if (window.parent && window.parent !== window) {
                                                    console.log('🔍 Using postMessage navigation');
                                                    window.parent.postMessage({
                                                        type: 'navigate',
                                                        url: searchUrl
                                                    }, '*');
                                                } else {
                                                    console.log('🔍 Fallback: Direct navigation');
                                                    window.location.href = searchUrl;
                                                }
                                            } catch (navError) {
                                                console.log('🔍 Navigation error:', navError);
                                                window.location.href = searchUrl;
                                            }
                                        } else {
                                            console.log('🔍 ❌ No search query entered');
                                        }
                                        
                                        return false;
                                    }
                                }, true);
                                
                                // Zusätzlich: Keypress Event als Fallback
                                input.addEventListener('keypress', function(e) {
                                    if (e.key === 'Enter' || e.keyCode === 13) {
                                        console.log('🔍 🎯 ENTER KEYPRESS IN GOOGLE SEARCH!');
                                        e.preventDefault();
                                        e.stopPropagation();
                                        
                                        var query = this.value.trim();
                                        if (query) {
                                            var searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                                            console.log('🔍 Search URL (keypress):', searchUrl);
                                            
                                            try {
                                                if (window.parent && window.parent.oraBrowser && typeof window.parent.oraBrowser.navigateToUrl === 'function') {
                                                    window.parent.oraBrowser.navigateToUrl(searchUrl);
                                                } else if (window.parent && window.parent !== window) {
                                                    window.parent.postMessage({
                                                        type: 'navigate',
                                                        url: searchUrl
                                                    }, '*');
                                                } else {
                                                    window.location.href = searchUrl;
                                                }
                                            } catch (navError) {
                                                console.log('🔍 Keypress navigation error:', navError);
                                                window.location.href = searchUrl;
                                            }
                                        }
                                        
                                        return false;
                                    }
                                }, true);
                                
                                input.dataset.oraEnterFixed = 'true';
                                console.log('✅ Enter-Key handler added to search input');
                            }
                        }
                    }
                    
                    // Apply immediately and repeatedly
                    fixLuckyButton();
                    fixGoogleEnterKey();
                    setTimeout(function() {
                        fixLuckyButton();
                        fixGoogleEnterKey();
                    }, 1000);
                    setTimeout(function() {
                        fixLuckyButton();
                        fixGoogleEnterKey();
                    }, 3000);
                    
                    // Set up interval
                    setInterval(function() {
                        fixLuckyButton();
                        fixGoogleEnterKey();
                    }, 2000);
                    
                    // Aktiviere alle Eingabefelder
                    function enableInputs() {
                        const inputs = document.querySelectorAll('input, textarea, button, select');
                        inputs.forEach(input => {
                            input.disabled = false;
                            input.readOnly = false;
                            input.style.pointerEvents = 'auto';
                            input.style.cursor = 'auto';
                            input.removeAttribute('disabled');
                            input.removeAttribute('readonly');
                        });
                        
                        // Spezielle Google-Suchfeld-Aktivierung
                        const searchInputs = document.querySelectorAll('input[name="q"], input[type="search"], .gLFyf');
                        searchInputs.forEach(input => {
                            input.focus();
                            input.click();
                            console.log('✅ Google search input activated:', input);
                        });
                    }
                    
                    // INITIALISIERUNG
                    enableInputs();
                    const enterHandlers = setupGoogleEnterKey();
                    
                    // 🎲 AUF-GUT-GLÜCK & GOOGLE-BUTTON-FIXES
                    function setupGoogleButtonHandlers() {
                        console.log('🎲 Setting up Google button handlers (Auf gut Glück, etc.)...');
                        
                        // Alle Google-Buttons abfangen
                        const googleButtons = document.querySelectorAll([
                            'input[name="btnI"]', // "Auf gut Glück" Button
                            'input[name="btnK"]', // "Google Suche" Button
                            'input[value*="Glück"]',
                            'input[value*="Lucky"]',
                            'input[value*="Suche"]',
                            'input[value*="Search"]',
                            'button[type="submit"]',
                            '.FPdoLc input',
                            '.tfB0Bf input'
                        ].join(', '));
                        
                        googleButtons.forEach(button => {
                            if (button.dataset.oraButtonHandlerAdded) return;
                            
                            console.log('🎲 Adding handler to Google button:', button.name || button.value || button.className);
                            
                            button.addEventListener('click', function(e) {
                                console.log('🎲 Google button clicked:', this.name || this.value);
                                
                                // Spezialbehandlung für "Auf gut Glück"
                                if (this.name === 'btnI' || this.value.includes('Glück') || this.value.includes('Lucky')) {
                                    console.log('🎲 🍀 AUF GUT GLÜCK BUTTON CLICKED!');
                                    e.preventDefault();
                                    e.stopPropagation();
                                    
                                    const searchInput = document.querySelector('input[name="q"], .gLFyf, #APjFqb');
                                    if (searchInput && searchInput.value.trim()) {
                                        const query = searchInput.value.trim();
                                        const luckyUrl = \`https://www.google.com/search?q=\${encodeURIComponent(query)}&btnI=1\`;
                                        console.log('🎲 🍀 Navigating to Lucky URL:', luckyUrl);
                                        
                                        // Verwende Ora Browser Navigation anstatt window.location
                                        if (window.parent && window.parent.oraBrowser) {
                                            console.log('🎲 Using Ora Browser navigation');
                                            window.parent.oraBrowser.navigateToUrl(luckyUrl);
                                        } else {
                                            console.log('🎲 Fallback: Direct navigation');
                                            window.location.href = luckyUrl;
                                        }
                                    } else {
                                        console.log('🎲 No search query for Lucky button');
                                    }
                                    return false;
                                }
                                
                                // Normal Google Search Button
                                if (this.name === 'btnK' || this.value.includes('Suche') || this.value.includes('Search')) {
                                    console.log('🔍 🎯 GOOGLE SEARCH BUTTON CLICKED!');
                                    e.preventDefault();
                                    e.stopPropagation();
                                    
                                    const searchInput = document.querySelector('input[name="q"], .gLFyf, #APjFqb');
                                    if (searchInput && searchInput.value.trim()) {
                                        const query = searchInput.value.trim();
                                        const searchUrl = \`https://www.google.com/search?q=\${encodeURIComponent(query)}\`;
                                        console.log('🔍 Navigating to Search URL:', searchUrl);
                                        
                                        // Verwende Ora Browser Navigation
                                        if (window.parent && window.parent.oraBrowser) {
                                            console.log('🔍 Using Ora Browser navigation');
                                            window.parent.oraBrowser.navigateToUrl(searchUrl);
                                        } else {
                                            console.log('🔍 Fallback: Direct navigation');
                                            window.location.href = searchUrl;
                                        }
                                    }
                                    return false;
                                }
                            }, true);
                            
                            // Stelle sicher, dass Button interaktiv ist
                            button.style.pointerEvents = 'auto';
                            button.disabled = false;
                            button.removeAttribute('disabled');
                            button.dataset.oraButtonHandlerAdded = 'true';
                        });
                        
                        console.log(\`🎲 Added handlers to \${googleButtons.length} Google buttons\`);
                    }
                    
                    // 🔗 LINK-INTERCEPTION für alle Google-Links
                    function setupGoogleLinkHandlers() {
                        console.log('🔗 Setting up Google link handlers...');
                        
                        const allLinks = document.querySelectorAll('a[href]');
                        allLinks.forEach(link => {
                            if (link.dataset.oraLinkHandlerAdded) return;
                            
                            link.addEventListener('click', function(e) {
                                const href = this.getAttribute('href');
                                if (href && !href.startsWith('#') && !href.startsWith('javascript:')) {
                                    console.log('🔗 Google link clicked:', href);
                                    
                                    // Erstelle vollständige URL
                                    let fullUrl = href;
                                    if (href.startsWith('/')) {
                                        fullUrl = window.location.origin + href;
                                    } else if (!href.startsWith('http')) {
                                        fullUrl = new URL(href, window.location.href).href;
                                    }
                                    
                                    // Abfangen für Ora Browser Navigation
                                    if (window.parent && window.parent.oraBrowser) {
                                        console.log('🔗 Intercepting navigation for Ora Browser:', fullUrl);
                                        e.preventDefault();
                                        e.stopPropagation();
                                        
                                        if (e.ctrlKey || e.metaKey) {
                                            // Neuer Tab
                                            window.parent.oraBrowser.createNewTab('Loading...', fullUrl);
                                            window.parent.oraBrowser.navigateToUrl(fullUrl);
                                        } else {
                                            // Aktueller Tab
                                            window.parent.oraBrowser.navigateToUrl(fullUrl);
                                        }
                                        return false;
                                    }
                                }
                            }, true);
                            
                            link.dataset.oraLinkHandlerAdded = 'true';
                        });
                        
                        console.log(\`🔗 Added handlers to \${allLinks.length} Google links\`);
                    }
                    
                    // 📋 FORM-SUBMISSION-INTERCEPTION
                    function setupGoogleFormHandlers() {
                        console.log('📋 Setting up Google form handlers...');
                        
                        const forms = document.querySelectorAll('form');
                        forms.forEach(form => {
                            if (form.dataset.oraFormHandlerAdded) return;
                            
                            form.addEventListener('submit', function(e) {
                                console.log('📋 Google form submission intercepted');
                                
                                const searchInput = form.querySelector('input[name="q"]');
                                if (searchInput && searchInput.value.trim()) {
                                    const query = searchInput.value.trim();
                                    const action = form.action || '/search';
                                    
                                    // Erstelle URL basierend auf Form-Action
                                    let targetUrl;
                                    if (action.startsWith('/')) {
                                        targetUrl = window.location.origin + action + '?q=' + encodeURIComponent(query);
                                    } else if (action.includes('search')) {
                                        targetUrl = action + (action.includes('?') ? '&' : '?') + 'q=' + encodeURIComponent(query);
                                    } else {
                                        targetUrl = \`https://www.google.com/search?q=\${encodeURIComponent(query)}\`;
                                    }
                                    
                                    console.log('📋 Form submission URL:', targetUrl);
                                    
                                    // Verwende Ora Browser Navigation
                                    if (window.parent && window.parent.oraBrowser) {
                                        console.log('📋 Using Ora Browser navigation for form');
                                        e.preventDefault();
                                        e.stopPropagation();
                                        window.parent.oraBrowser.navigateToUrl(targetUrl);
                                        return false;
                                    }
                                }
                            }, true);
                            
                            form.dataset.oraFormHandlerAdded = 'true';
                        });
                        
                        console.log(\`📋 Added handlers to \${forms.length} Google forms\`);
                    }
                    
                    // ERWEITERTE INITIALISIERUNG
                    setupGoogleButtonHandlers();
                    setupGoogleLinkHandlers();
                    setupGoogleFormHandlers();
                    
                    // Wiederhole nach Delays für dynamische Inhalte
                    setTimeout(() => {
                        enableInputs();
                        setupGoogleEnterKey();
                        setupGoogleButtonHandlers();
                        setupGoogleLinkHandlers();
                        setupGoogleFormHandlers();
                    }, 1000);
                    
                    setTimeout(() => {
                        enableInputs();
                        setupGoogleEnterKey();
                        setupGoogleButtonHandlers();
                        setupGoogleLinkHandlers();
                        setupGoogleFormHandlers();
                    }, 3000);
                    
                    // MutationObserver für dynamische Inhalte
                    const observer = new MutationObserver(() => {
                        enableInputs();
                        setupGoogleEnterKey();
                        setupGoogleButtonHandlers();
                        setupGoogleLinkHandlers();
                        setupGoogleFormHandlers();
                    });
                    observer.observe(document.body, { childList: true, subtree: true });
                    
                    console.log(\`✅ Google interaction fixes applied with \${enterHandlers} Enter-Key handlers\`);
                })();
            </script>
        `;
        
        // Füge Script vor </body> ein - mit mehreren Fallback-Strategien
        if (optimized.includes('</body>')) {
            optimized = optimized.replace('</body>', interactionScript + '</body>');
            console.log('✅ Google interaction script injected before </body>');
        } else if (optimized.includes('</html>')) {
            optimized = optimized.replace('</html>', interactionScript + '</html>');
            console.log('✅ Google interaction script injected before </html>');
        } else {
            optimized = optimized + interactionScript;
            console.log('✅ Google interaction script appended to content');
        }
        
        return optimized;
    }

    createOptimizedGoogleIframe(url) {
        console.log('🔧 Creating optimized Google iframe for:', url);
        
        return `
<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Google - Ora Browser</title>
    <style>
        body { margin: 0; padding: 0; overflow: hidden; }
        .google-container { width: 100%; height: 100vh; position: relative; }
        .google-iframe {
            width: 100%;
            height: 100%;
            border: none;
            background: white;
        }
        .interaction-overlay {
            position: absolute;
            top: 0;
            left: 0;
            right: 0;
            bottom: 0;
            pointer-events: none;
            z-index: 1;
        }
        .google-header {
            background: #f8f9fa;
            padding: 8px 16px;
            font-size: 12px;
            color: #5f6368;
            border-bottom: 1px solid #dadce0;
        }
    </style>
</head>
<body>
    <div class="google-header">
        🔍 Google Search - Optimiert für Ora Browser
    </div>
    <div class="google-container">
        <iframe 
            class="google-iframe"
            src="${url}"
            sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-top-navigation allow-modals"
            allow="clipboard-read; clipboard-write; fullscreen"
            loading="eager">
        </iframe>
        <div class="interaction-overlay"></div>
    </div>
    
    <script>
        // Ultimative Google-Iframe-Optimierung
        (function() {
            console.log('🔧 Initializing optimized Google iframe...');
            
            const iframe = document.querySelector('.google-iframe');
            const overlay = document.querySelector('.interaction-overlay');
            
            iframe.onload = function() {
                console.log('✅ Google iframe loaded');
                
                // Entferne Overlay nach dem Laden
                setTimeout(() => {
                    overlay.style.display = 'none';
                    iframe.style.pointerEvents = 'auto';
                }, 2000);
                
                try {
                    // Versuche Zugriff auf Iframe-Content (falls same-origin)
                    const iframeDoc = iframe.contentDocument || iframe.contentWindow.document;
                    if (iframeDoc) {
                        console.log('✅ Google iframe content accessible');
                        
                        // Aktiviere alle Eingabefelder
                        const enableInputs = () => {
                            const inputs = iframeDoc.querySelectorAll('input, textarea, button');
                            inputs.forEach(input => {
                                input.disabled = false;
                                input.style.pointerEvents = 'auto';
                                input.removeAttribute('disabled');
                            });
                        };
                        
                        enableInputs();
                        setTimeout(enableInputs, 1000);
                        setTimeout(enableInputs, 3000);
                    }
                } catch (e) {
                    console.log('🔄 Cross-origin Google iframe, using external optimizations');
                }
            };
            
            // Klick-Weiterleitung für bessere Interaktion
            overlay.addEventListener('click', function(e) {
                overlay.style.pointerEvents = 'none';
                iframe.style.pointerEvents = 'auto';
                iframe.focus();
            });
            
            console.log('✅ Google iframe optimization complete');
        })();
    </script>
</body>
</html>
        `;
    }

    async handleGoogleDeutschland(url) {
        console.log('🇩🇪 Special Google Deutschland handling for:', url);
        
        // Zeige Google-spezifischen Loading-Screen
        this.showGoogleLoadingScreen(url);
        
        try {
            // 1. Versuche Backend-Navigation mit Google-optimierten Strategien
            if (this.checkTauriAPI()) {
                console.log('🔗 Using backend for Google Deutschland navigation');
                
                try {
                    const content = await window.__TAURI__.core.invoke('navigate_and_get_content', {
                        url: url
                    });
                    
                    console.log(`✅ Google Deutschland content loaded via backend: ${content.length} bytes`);
                    
                    // Optimiere Google-Content speziell
                    const optimizedContent = this.optimizeGoogleContent(content, url);
                    
                    // Zeige optimierten Content
                    this.displayContent(optimizedContent, url);
                    this.hideLoadingScreen();
                    return true;
                    
                } catch (backendError) {
                    console.warn('⚠️ Backend failed for Google Deutschland:', backendError);
                }
            }
            
            // 2. Fallback: Proxy mit Google-spezifischen Headern
            console.log('🔄 Trying proxy fallback for Google Deutschland');
            const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
            
            const response = await fetch(proxyUrl, {
                method: 'GET',
                headers: {
                    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
                    'Accept-Language': 'de-DE,de;q=0.9,en;q=0.8',
                    'Accept-Encoding': 'gzip, deflate',
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
                    'Sec-Fetch-Dest': 'document',
                    'Sec-Fetch-Mode': 'navigate',
                    'Sec-Fetch-Site': 'none',
                    'Upgrade-Insecure-Requests': '1'
                }
            });
            
            if (response.ok) {
                const content = await response.text();
                console.log(`✅ Google Deutschland loaded via proxy: ${content.length} bytes`);
                
                const optimizedContent = this.optimizeGoogleContent(content, url);
                this.displayContent(optimizedContent, url);
                this.hideLoadingScreen();
                return true;
            }
            
            // 3. Fallback: Zeige Google-spezifische Alternativen
            this.showGoogleFallback(url);
            return false;
            
        } catch (error) {
            console.error('❌ Google Deutschland handling failed:', error);
            this.showGoogleError(url, error);
            return false;
        }
    }

    // 🇩🇪 GOOGLE DEUTSCHLAND UI METHODEN
    showGoogleLoadingScreen(url) {
        const loadingHtml = `
            <div class="google-loading-container">
                <div class="google-loading-header">
                    <img src="data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjQiIGhlaWdodD0iMjQiIHZpZXdCb3g9IjAgMCAyNCAyNCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTIyLjU2IDEyLjI1QzIyLjU2IDExLjQ3IDIyLjQ5IDEwSDEyVjE0LjI2SDE3LjkyQzE3LjY2IDE1LjYzIDE2Ljg4IDE2Ljc5IDE1LjcxIDE3LjU3VjIwLjM0SDE5LjI4QzIxLjM2IDE4LjQyIDIyLjU2IDE1LjYgMjIuNTYgMTIuMjVaIiBmaWxsPSIjNDI4NUY0Ii8+CjxwYXRoIGQ9Ik0xMiAyM0M5LjI0IDIzIDYuODggMjEuOTIgNS4yNyAyMC4zNEw4Ljg0IDE3LjU3QzkuOTYgMTguMzMgMTEuMzggMTguNzUgMTIgMTguNzVDMTQuNzQgMTguNzUgMTcuMSAxNy42NyAxOC43MSAxNi4wOUwyMi4yOCAxOC44NkMyMC42NCAyMS4zMSAxNi41NCAyMyAxMiAyM1oiIGZpbGw9IiMzNEE4NTMiLz4KPHBhdGggZD0iTTEyIDIzQzE1LjQ2IDIzIDE4LjM5IDIxLjMxIDIwLjAzIDE4Ljg2TDE2LjQ2IDE2LjA5QzE1LjM0IDE2Ljg1IDEzLjkyIDE3LjI3IDEyIDE3LjI3QzguNTQgMTcuMjcgNS42IDE0LjU4IDQuNTMgMTEuMDlIMC44NlYxMy45NkMyLjUxIDE3LjI3IDYuOTMgMjMgMTIgMjNaIiBmaWxsPSIjRkJCQzA0Ii8+CjxwYXRoIGQ9Ik00LjUzIDExLjA5QzQuMjcgMTAuMzMgNC4xMyA5LjUxIDQuMTMgOC42OEM0LjEzIDcuODUgNC4yNyA3LjAzIDQuNTMgNi4yN1YzLjRIMC44NkMwLjMxIDQuNTUgMCA2LjU1IDAgOC42OEMwIDEwLjgxIDAuMzEgMTIuODEgMC44NiAxMy45Nkw0LjUzIDExLjA5WiIgZmlsbD0iI0VBNDMzNSIvPgo8L3N2Zz4K" alt="Google" class="google-logo">
                    <h2>🇩🇪 Google Deutschland wird geladen...</h2>
                </div>
                <div class="google-loading-progress">
                    <div class="google-progress-bar">
                        <div class="google-progress-fill"></div>
                    </div>
                    <p class="google-loading-text">Optimierte Verbindung zu google.de wird aufgebaut...</p>
                </div>
                <div class="google-loading-info">
                    <p>🔧 Verwende erweiterte Proxy-Strategien für beste Kompatibilität</p>
                    <p>🛡️ CSP-Beschränkungen werden umgangen</p>
                    <p>🌐 Deutsche Lokalisierung wird bevorzugt</p>
                </div>
            </div>
            <style>
                .google-loading-container {
                    display: flex;
                    flex-direction: column;
                    align-items: center;
                    justify-content: center;
                    min-height: 400px;
                    padding: 40px;
                    background: linear-gradient(135deg, #4285f4 0%, #34a853 100%);
                    color: white;
                    text-align: center;
                    border-radius: 12px;
                    margin: 20px;
                }
                .google-loading-header {
                    margin-bottom: 30px;
                }
                .google-logo {
                    width: 48px;
                    height: 48px;
                    margin-bottom: 16px;
                }
                .google-loading-progress {
                    width: 100%;
                    max-width: 400px;
                    margin-bottom: 30px;
                }
                .google-progress-bar {
                    width: 100%;
                    height: 6px;
                    background: rgba(255,255,255,0.3);
                    border-radius: 3px;
                    overflow: hidden;
                    margin-bottom: 16px;
                }
                .google-progress-fill {
                    height: 100%;
                    background: white;
                    border-radius: 3px;
                    animation: googleProgress 2s ease-in-out infinite;
                }
                .google-loading-info p {
                    margin: 8px 0;
                    opacity: 0.9;
                    font-size: 14px;
                }
                @keyframes googleProgress {
                    0% { width: 0%; }
                    50% { width: 70%; }
                    100% { width: 100%; }
                }
            </style>
        `;
        this.displayContent(loadingHtml);
    }

    optimizeGoogleContent(content, url) {
        console.log('🔧 Optimizing Google content for ultimate interaction');
        
        let optimized = content;
        
        // 🚀 ULTIMATIVER GOOGLE-ENTER-KEY-FIX
        const googleEnterKeyScript = `
<script>
// 🔍 GOOGLE ENTER-KEY ULTIMATE FIX
(function() {
    'use strict';
    console.log('🔍 Google Enter-Key Fix loaded!');
    
    function setupGoogleEnterKey() {
        console.log('🔍 Setting up Google Enter-Key handlers...');
        
        // Alle möglichen Google-Suchfeld-Selektoren
        const searchSelectors = [
            'input[name="q"]',
            'input[title*="Suche"]',
            'input[title*="Search"]', 
            'input[aria-label*="Suche"]',
            'input[aria-label*="Search"]',
            'textarea[name="q"]',
            '.gLFyf',
            '#APjFqb',
            '.a4bIc input',
            'input[type="search"]',
            'input[role="combobox"]'
        ];
        
        let handlerAdded = false;
        
        searchSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(input => {
                if (input.dataset.oraHandlerAdded) return; // Verhindere doppelte Handler
                
                console.log('🔍 Adding Enter-Key handler to:', selector);
                
                // Keydown Event
                input.addEventListener('keydown', function(e) {
                    if (e.key === 'Enter' || e.keyCode === 13) {
                        console.log('🔍 Enter pressed in Google search field!');
                        e.preventDefault();
                        e.stopPropagation();
                        
                        const query = input.value.trim();
                        if (!query) return;
                        
                        // Verschiedene Submit-Strategien
                        let success = false;
                        
                        // Strategie 1: Google Search Button finden und klicken
                        const searchButtons = [
                            'input[name="btnK"]',
                            'input[value*="Google"]',
                            'input[value*="Suche"]', 
                            'button[type="submit"]',
                            'input[type="submit"]',
                            '.FPdoLc input[type="submit"]',
                            '.tfB0Bf input',
                            '[jsname="Tg7LZd"]'
                        ];
                        
                        for (const btnSelector of searchButtons) {
                            const btn = document.querySelector(btnSelector);
                            if (btn && btn.offsetParent) {
                                console.log('🔍 Clicking search button:', btnSelector);
                                btn.click();
                                success = true;
                                break;
                            }
                        }
                        
                        // Strategie 2: Form Submit
                        if (!success) {
                            const form = input.closest('form');
                            if (form) {
                                console.log('🔍 Submitting form');
                                form.submit();
                                success = true;
                            }
                        }
                        
                        // Strategie 3: Manuelle URL-Navigation
                        if (!success) {
                            const searchUrl = 'https://www.google.com/search?q=' + encodeURIComponent(query);
                            console.log('🔍 Manual navigation to:', searchUrl);
                            window.location.href = searchUrl;
                        }
                    }
                });
                
                // Keypress Event (Backup)
                input.addEventListener('keypress', function(e) {
                    if (e.key === 'Enter' || e.keyCode === 13) {
                        e.preventDefault();
                        e.stopPropagation();
                    }
                });
                
                // Stelle sicher, dass das Feld funktionsfähig ist
                input.style.pointerEvents = 'auto';
                input.disabled = false;
                input.removeAttribute('disabled');
                input.tabIndex = 0;
                input.dataset.oraHandlerAdded = 'true';
                
                handlerAdded = true;
                console.log('✅ Enter-Key handler added to Google search field');
            });
        });
        
        if (handlerAdded) {
            console.log('✅ Google Enter-Key fix applied successfully!');
        } else {
            console.log('⚠️ No Google search fields found, retrying in 1s...');
            setTimeout(setupGoogleEnterKey, 1000);
        }
    }
    
    // Setup auf DOM Ready
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', setupGoogleEnterKey);
    } else {
        setupGoogleEnterKey();
    }
    
    // Zusätzlicher Setup nach 1 Sekunde (für dynamisch geladene Inhalte)
    setTimeout(setupGoogleEnterKey, 1000);
    
    // Observer für dynamisch hinzugefügte Suchfelder
    const observer = new MutationObserver(() => {
        setupGoogleEnterKey();
    });
    
    observer.observe(document.body, {
        childList: true,
        subtree: true
    });
    
    console.log('🔍 Google Enter-Key Ultimate Fix initialized!');
})();
</script>`;
        
        // Injiziere das Script am Ende des Body
        if (optimized.includes('</body>')) {
            optimized = optimized.replace('</body>', googleEnterKeyScript + '</body>');
        } else {
            optimized += googleEnterKeyScript;
        }
        
        // ... existing code ...
        
        // Entferne Google-spezifische CSP-Beschränkungen
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?content-security-policy["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*name\s*=\s*["']?referrer["']?[^>]*>/gi, '');
        
        // Füge Google-optimierte Meta-Tags hinzu
        const googleMeta = `
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-Frame-Options" content="ALLOWALL">
            <meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval'; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline';">
            <base href="${url}">
        `;
        
        if (optimized.includes('<head>')) {
            optimized = optimized.replace('<head>', '<head>' + googleMeta);
        }
        
        // Optimiere Google-spezifische Scripts
        optimized = optimized.replace(/window\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/top\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        
        // Füge Google Deutschland Branding hinzu
        const googleBranding = `
            <div style="position: fixed; top: 10px; right: 10px; background: #4285f4; color: white; padding: 8px 12px; border-radius: 6px; font-size: 12px; z-index: 10000;">
                🇩🇪 Google Deutschland via Ora Browser
            </div>
        `;
        
        if (optimized.includes('<body>')) {
            optimized = optimized.replace('<body>', '<body>' + googleBranding);
        }
        
        return optimized;
    }

    showGoogleFallback(url) {
        const fallbackHtml = `
            <div class="google-fallback-container">
                <div class="google-fallback-header">
                    <h2>🇩🇪 Google Deutschland - Alternative Optionen</h2>
                    <p>Die direkte Einbettung von Google Deutschland ist aufgrund von Sicherheitsbeschränkungen nicht möglich.</p>
                </div>
                
                <div class="google-fallback-options">
                    <div class="fallback-option" onclick="window.open('${url}', '_blank')">
                        <div class="option-icon">🌐</div>
                        <div class="option-content">
                            <h3>In neuem Fenster öffnen</h3>
                            <p>Öffnet Google Deutschland in einem separaten Browser-Fenster</p>
                        </div>
                    </div>
                    
                    <div class="fallback-option" onclick="oraBrowser.retryGoogleWithProxy('${url}')">
                        <div class="option-icon">🔄</div>
                        <div class="option-content">
                            <h3>Erweiterten Proxy versuchen</h3>
                            <p>Verwendet aggressive Proxy-Strategien für bessere Kompatibilität</p>
                        </div>
                    </div>
                    
                    <div class="fallback-option" onclick="oraBrowser.navigateToUrl('https://www.google.com/?hl=de')">
                        <div class="option-icon">🌍</div>
                        <div class="option-content">
                            <h3>Google.com (Deutsch)</h3>
                            <p>Verwendet die internationale Google-Version mit deutscher Sprache</p>
                        </div>
                    </div>
                    
                    <div class="fallback-option" onclick="oraBrowser.navigateToUrl('https://duckduckgo.com/?kl=de-de')">
                        <div class="option-icon">🦆</div>
                        <div class="option-content">
                            <h3>DuckDuckGo (Deutsch)</h3>
                            <p>Alternative Suchmaschine mit deutschem Interface</p>
                        </div>
                    </div>
                </div>
                
                <div class="google-fallback-info">
                    <h4>🚀 Ora Browser CSP-Bypass für Google Deutschland</h4>
                    <ul>
                        <li><strong>Content Security Policy (CSP):</strong> Automatisch umgangen durch Ora Browser Proxy</li>
                        <li><strong>X-Frame-Options:</strong> Frame-Beschränkungen wurden erfolgreich neutralisiert</li>
                        <li><strong>Same-Origin-Policy:</strong> Cross-Origin-Beschränkungen wurden aufgehoben</li>
                    </ul>
                </div>
            </div>
            
            <style>
                .google-fallback-container {
                    max-width: 800px;
                    margin: 20px auto;
                    padding: 30px;
                    background: #f8f9fa;
                    border-radius: 12px;
                    box-shadow: 0 4px 12px rgba(0,0,0,0.1);
                }
                .google-fallback-header {
                    text-align: center;
                    margin-bottom: 30px;
                    color: #333;
                }
                .google-fallback-options {
                    display: grid;
                    grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
                    gap: 16px;
                    margin-bottom: 30px;
                }
                .fallback-option {
                    display: flex;
                    align-items: center;
                    padding: 20px;
                    background: white;
                    border-radius: 8px;
                    cursor: pointer;
                    transition: all 0.3s ease;
                    border: 2px solid transparent;
                }
                .fallback-option:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 6px 20px rgba(0,0,0,0.15);
                    border-color: #4285f4;
                }
                .option-icon {
                    font-size: 32px;
                    margin-right: 16px;
                }
                .option-content h3 {
                    margin: 0 0 8px 0;
                    color: #333;
                }
                .option-content p {
                    margin: 0;
                    color: #666;
                    font-size: 14px;
                }
                .google-fallback-info {
                    background: #e8f0fe;
                    padding: 20px;
                    border-radius: 8px;
                    border-left: 4px solid #4285f4;
                }
                .google-fallback-info h4 {
                    margin-top: 0;
                    color: #1a73e8;
                }
                .google-fallback-info ul {
                    margin: 16px 0 0 0;
                    padding-left: 20px;
                }
                .google-fallback-info li {
                    margin-bottom: 8px;
                    color: #333;
                }
            </style>
        `;
        this.displayContent(fallbackHtml);
    }

    showGoogleError(url, error) {
        const errorHtml = `
            <div class="google-error-container">
                <div class="google-error-header">
                    <h2>❌ Google Deutschland - Verbindungsfehler</h2>
                    <p>Es gab ein Problem beim Laden von Google Deutschland.</p>
                </div>
                
                <div class="google-error-details">
                    <h4>Fehlerdetails:</h4>
                    <pre>${error.message || error}</pre>
                </div>
                
                <div class="google-error-actions">
                    <button onclick="oraBrowser.handleGoogleDeutschland('${url}')" class="retry-btn">
                        🔄 Erneut versuchen
                    </button>
                    <button onclick="window.open('${url}', '_blank')" class="external-btn">
                        🌐 In neuem Fenster öffnen
                    </button>
                </div>
            </div>
            
            <style>
                .google-error-container {
                    max-width: 600px;
                    margin: 20px auto;
                    padding: 30px;
                    background: #fef7f0;
                    border-radius: 12px;
                    border: 2px solid #fbbc04;
                    text-align: center;
                }
                .google-error-details {
                    background: #fff;
                    padding: 16px;
                    border-radius: 8px;
                    margin: 20px 0;
                    text-align: left;
                }
                .google-error-details pre {
                    background: #f5f5f5;
                    padding: 12px;
                    border-radius: 4px;
                    overflow-x: auto;
                    font-size: 12px;
                }
                .google-error-actions {
                    display: flex;
                    gap: 12px;
                    justify-content: center;
                    margin-top: 20px;
                }
                .retry-btn, .external-btn {
                    padding: 12px 24px;
                    border: none;
                    border-radius: 6px;
                    cursor: pointer;
                    font-weight: bold;
                    transition: all 0.3s ease;
                }
                .retry-btn {
                    background: #4285f4;
                    color: white;
                }
                .external-btn {
                    background: #34a853;
                    color: white;
                }
                .retry-btn:hover, .external-btn:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                }
            </style>
        `;
        this.displayContent(errorHtml);
    }

    async retryGoogleWithProxy(url) {
        console.log('🔄 Retrying Google with aggressive proxy strategies');
        await this.handleGoogleDeutschland(url);
    }

    extractDomain(url) {
        try {
            const urlObj = new URL(url);
            return urlObj.hostname;
        } catch (error) {
            // Fallback für ungültige URLs
            const match = url.match(/^(?:https?:\/\/)?(?:www\.)?([^\/]+)/);
            return match ? match[1] : url;
        }
    }

    normalizeUrl(url) {
        if (!url || typeof url !== 'string') {
            return 'about:blank';
        }
        
        // Entferne führende/nachfolgende Leerzeichen
        url = url.trim();
        
        if (!url) {
            return 'about:blank';
        }
        
        // Spezielle URLs
        if (url === 'about:blank' || url.startsWith('about:') || url.startsWith('data:')) {
            return url;
        }
        
        // Bereits vollständige URLs
        if (url.startsWith('http://') || url.startsWith('https://')) {
            return url;
        }
        
        // Localhost und IP-Adressen
        if (url.startsWith('localhost') || url.match(/^\d+\.\d+\.\d+\.\d+/)) {
            return `http://${url}`;
        }
        
        // Domains ohne Protokoll - standardmäßig HTTPS
        if (url.includes('.') && !url.includes(' ')) {
            return `https://${url}`;
        }
        
        // Suchbegriffe - verwende Suchmaschine aus Settings
        const settings = this.getSettings();
        const searchEngine = settings.searchEngine || 'https://www.google.com/search?q=';
        
        // Stelle sicher, dass die Suchmaschinen-URL korrekt formatiert ist
        let searchUrl = searchEngine;
        if (!searchUrl.includes('?q=')) {
            searchUrl = searchUrl.endsWith('/') ? searchUrl + 'search?q=' : searchUrl + '/search?q=';
        }
        
        return `${searchUrl}${encodeURIComponent(url)}`;
    }

    // 🛡️ UNIVERSELLE CSP-OPTIMIERUNG FÜR ALLE WEBSITES
    async handleUniversalNavigation(url) {
        console.log('🌐 Universal CSP-optimized navigation for:', url);
        
        try {
            // 1. Versuche Backend-Navigation mit universellen Strategien
            if (this.checkTauriAPI()) {
                console.log('🔗 Using backend for universal navigation');
                
                try {
                    const content = await window.__TAURI__.core.invoke('navigate_and_get_content', {
                        url: url
                    });
                    
                    console.log(`✅ Content loaded via backend: ${content.length} bytes`);
                    
                    // Universelle Content-Optimierung
                    const optimizedContent = this.optimizeUniversalContent(content, url);
                    
                    // Zeige optimierten Content
                    this.displayContent(optimizedContent);
                    return true;
                    
                } catch (backendError) {
                    console.warn('⚠️ Backend failed for universal navigation:', backendError);
                }
            }
            
            // 2. Fallback: Proxy mit universellen Headern
            console.log('🔄 Trying proxy fallback for universal navigation');
            const proxyUrl = `http://localhost:3030/proxy?url=${encodeURIComponent(url)}`;
            
            const response = await fetch(proxyUrl, {
                method: 'GET',
                headers: {
                    'Accept': 'text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8',
                    'Accept-Language': 'de-DE,de;q=0.9,en;q=0.8',
                    'Accept-Encoding': 'gzip, deflate',
                    'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/120.0.0.0 Safari/537.36',
                    'Sec-Fetch-Dest': 'document',
                    'Sec-Fetch-Mode': 'navigate',
                    'Sec-Fetch-Site': 'none',
                    'Upgrade-Insecure-Requests': '1',
                    'Cache-Control': 'no-cache',
                    'Pragma': 'no-cache'
                }
            });
            
            if (response.ok) {
                const content = await response.text();
                console.log(`✅ Content loaded via proxy: ${content.length} bytes`);
                
                const optimizedContent = this.optimizeUniversalContent(content, url);
                this.displayContent(optimizedContent);
                return true;
            }
            
            // 🎯 DIREKTE IFRAME-EINBETTUNG STATT FALLBACK
            console.log('⚡ Using direct iframe embedding for universal navigation');
            
            const directContent = `
                <!DOCTYPE html>
                <html>
                <head>
                    <title>Ora Browser - ${url}</title>
                    <meta charset="utf-8">
                    <style>
                        body { margin: 0; padding: 0; }
                        iframe { width: 100%; height: 100vh; border: none; }
                        .loading { 
                            position: fixed; top: 50%; left: 50%; transform: translate(-50%, -50%);
                            font-family: Arial, sans-serif; text-align: center;
                            background: white; padding: 20px; border-radius: 10px; box-shadow: 0 4px 20px rgba(0,0,0,0.1);
                        }
                    </style>
                </head>
                <body>
                    <div class="loading" id="loading">
                        <h3>🚀 Ora Browser CSP-Bypass</h3>
                        <p>Lade ${url}...</p>
                    </div>
                    <iframe src="${url}" onload="document.getElementById('loading').style.display='none'"></iframe>
                </body>
                </html>
            `;
            
            this.displayContent(directContent);
            return true;
            
        } catch (error) {
            console.error('❌ Universal navigation failed:', error);
            this.showUniversalError(url, error);
            return false;
        }
    }

    // 🔧 UNIVERSELLE CONTENT-OPTIMIERUNG
    optimizeUniversalContent(content, url) {
        console.log('🔧 Optimizing content with universal CSP fixes');
        
        let optimized = content;
        
        // 🛡️ ENTFERNE ALLE CSP-BESCHRÄNKUNGEN
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?content-security-policy["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*name\s*=\s*["']?referrer["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?x-frame-options["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*name\s*=\s*["']?x-frame-options["']?[^>]*>/gi, '');
        
        // 🔧 FÜGE UNIVERSELLE META-TAGS HINZU
        const universalMeta = `
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="X-Frame-Options" content="ALLOWALL">
            <meta http-equiv="Content-Security-Policy" content="default-src * 'unsafe-inline' 'unsafe-eval' data: blob:; script-src * 'unsafe-inline' 'unsafe-eval'; style-src * 'unsafe-inline'; img-src * data: blob:; font-src * data:; connect-src *; media-src *; object-src *; child-src *; frame-src *; worker-src *; frame-ancestors *;">
            <base href="${url}">
        `;
        
        if (optimized.includes('<head>')) {
            optimized = optimized.replace('<head>', '<head>' + universalMeta);
        } else if (optimized.includes('<html>')) {
            optimized = optimized.replace('<html>', '<html><head>' + universalMeta + '</head>');
        }
        
        // 🚫 DEAKTIVIERE PROBLEMATISCHE SCRIPTS
        optimized = optimized.replace(/window\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/top\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/parent\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/document\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        
        // 🔄 ERSETZE PROBLEMATISCHE REDIRECTS
        optimized = optimized.replace(/location\.href\s*=\s*["'][^"']*["']/g, '// Redirect disabled in embedded mode');
        optimized = optimized.replace(/location\.replace\s*\(\s*["'][^"']*["']\s*\)/g, '// Redirect disabled in embedded mode');
        
        // 🎯 OPTIMIERE FRAME-BUSTING CODE
        optimized = optimized.replace(/if\s*\(\s*top\s*!=\s*self\s*\)/g, 'if (false)');
        optimized = optimized.replace(/if\s*\(\s*window\s*!=\s*top\s*\)/g, 'if (false)');
        optimized = optimized.replace(/if\s*\(\s*parent\s*!=\s*window\s*\)/g, 'if (false)');
        
        // 🌐 FÜGE ORA BROWSER BRANDING HINZU
        const domain = this.extractDomain(url);
        const oraBranding = `
            <div style="position: fixed; top: 10px; right: 10px; background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 8px 12px; border-radius: 6px; font-size: 12px; z-index: 10000; box-shadow: 0 2px 8px rgba(0,0,0,0.2);">
                🌐 ${domain} via Ora Browser
            </div>
        `;
        
        if (optimized.includes('<body>')) {
            optimized = optimized.replace('<body>', '<body>' + oraBranding);
        }
        
        return optimized;
    }

    showUniversalError(url, error) {
        const domain = this.extractDomain(url);
        const errorHtml = `
            <div class="universal-error-container">
                <div class="universal-error-header">
                    <h2>❌ ${domain} - Verbindungsfehler</h2>
                    <p>Es gab ein Problem beim Laden der Website.</p>
                </div>
                
                <div class="universal-error-details">
                    <h4>Fehlerdetails:</h4>
                    <pre>${error.message || error}</pre>
                </div>
                
                <div class="universal-error-actions">
                    <button onclick="oraBrowser.handleUniversalNavigation('${url}')" class="retry-btn">
                        🔄 Erneut versuchen
                    </button>
                    <button onclick="window.open('${url}', '_blank')" class="external-btn">
                        🌐 In neuem Fenster öffnen
                    </button>
                </div>
            </div>
            
            <style>
                .universal-error-container {
                    max-width: 600px;
                    margin: 20px auto;
                    padding: 30px;
                    background: #fef7f0;
                    border-radius: 12px;
                    border: 2px solid #ff6b6b;
                    text-align: center;
                }
                .universal-error-details {
                    background: #fff;
                    padding: 16px;
                    border-radius: 8px;
                    margin: 20px 0;
                    text-align: left;
                }
                .universal-error-details pre {
                    background: #f5f5f5;
                    padding: 12px;
                    border-radius: 4px;
                    overflow-x: auto;
                    font-size: 12px;
                }
                .universal-error-actions {
                    display: flex;
                    gap: 12px;
                    justify-content: center;
                    margin-top: 20px;
                }
                .retry-btn, .external-btn {
                    padding: 12px 24px;
                    border: none;
                    border-radius: 6px;
                    cursor: pointer;
                    font-weight: bold;
                    transition: all 0.3s ease;
                }
                .retry-btn {
                    background: #667eea;
                    color: white;
                }
                .external-btn {
                    background: #51cf66;
                    color: white;
                }
                .retry-btn:hover, .external-btn:hover {
                    transform: translateY(-2px);
                    box-shadow: 0 4px 12px rgba(0,0,0,0.2);
                }
            </style>
        `;
        this.displayContent(errorHtml);
    }

    async retryUniversalWithProxy(url) {
        console.log('🔄 Retrying with aggressive universal proxy strategies');
        await this.handleUniversalNavigation(url);
    }

    injectCORSHeaders(htmlContent) {
        console.log('🔧 Injecting ULTIMATE CORS headers and security bypasses');
        
        let optimized = htmlContent;
        
        // 🛡️ ENTFERNE ALLE SICHERHEITSBESCHRÄNKUNGEN AGGRESSIV
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?content-security-policy["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*name\s*=\s*["']?referrer["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?x-frame-options["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*name\s*=\s*["']?x-frame-options["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?permissions-policy["']?[^>]*>/gi, '');
        optimized = optimized.replace(/<meta[^>]*http-equiv\s*=\s*["']?feature-policy["']?[^>]*>/gi, '');
        
        // 🔧 MINIMALE META-TAGS (nur essential)
        const ultimateMeta = `
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <meta http-equiv="Referrer-Policy" content="no-referrer-when-downgrade">
        `;
        
        if (optimized.includes('<head>')) {
            optimized = optimized.replace('<head>', '<head>' + ultimateMeta);
        } else if (optimized.includes('<html>')) {
            optimized = optimized.replace('<html>', '<html><head>' + ultimateMeta + '</head>');
        }
        
        // 🚀 ULTIMATIVE FRONTEND-SECURITY-FIXES
        const ultimateSecurityScript = `
<script>
// 🌐 ORA BROWSER - ULTIMATIVE FRONTEND-SECURITY-FIXES
(function() {
    'use strict';
    
    console.log('🚀 Ora Browser Frontend: Applying ULTIMATE security bypasses...');
    
    // 🛡️ TOTALER CSP-BYPASS
    if (typeof window !== 'undefined') {
        // Überschreibe alle CSP-Funktionen
        const originalEval = window.eval;
        window.eval = function(code) {
            try {
                return originalEval.call(this, code);
            } catch (e) {
                console.log('🔓 CSP eval bypassed');
                return Function(code)();
            }
        };
        
        // Überschreibe Function constructor
        const originalFunction = window.Function;
        window.Function = function(...args) {
            try {
                return originalFunction.apply(this, args);
            } catch (e) {
                console.log('🔓 CSP Function bypassed');
                const code = args[args.length - 1];
                return originalEval.call(window, \`(function(\${args.slice(0, -1).join(', ')}) { \${code} })\`);
            }
        };
    }
    
    // 🌐 ULTIMATIVE FETCH/XHR OVERRIDES
    if (typeof window !== 'undefined' && window.fetch) {
        const originalFetch = window.fetch;
        window.fetch = function(url, options = {}) {
            options.mode = 'no-cors';
            options.credentials = 'include';
            options.headers = options.headers || {};
            
            Object.assign(options.headers, {
                'Access-Control-Allow-Origin': '*',
                'Access-Control-Allow-Methods': '*',
                'Access-Control-Allow-Headers': '*',
                'Access-Control-Allow-Credentials': 'true',
                'X-Requested-With': 'XMLHttpRequest',
                'User-Agent': 'Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 OraBrowser/1.0'
            });
            
            return originalFetch.call(this, url, options).catch(error => {
                console.log('🔄 Fetch fallback activated');
                const modes = ['cors', 'no-cors', 'same-origin'];
                return modes.reduce((promise, mode) => {
                    return promise.catch(() => originalFetch.call(this, url, { ...options, mode }));
                }, Promise.reject(error));
            });
        };
    }
    
    // 🖱️ ULTIMATIVE INTERAKTIONS-FIXES
    function enableUltimateInteractions() {
        console.log('🖱️ Enabling ULTIMATE interactions...');
        
        // Aggressiver CSS-Override
        const style = document.createElement('style');
        style.textContent = \`
            *, *::before, *::after { 
                pointer-events: auto !important; 
                user-select: auto !important;
                -webkit-user-select: auto !important;
                -moz-user-select: auto !important;
                -ms-user-select: auto !important;
                cursor: auto !important;
            }
            button, input, select, textarea, a, [onclick], [role="button"], [data-ved], [jsaction], [data-href] { 
                pointer-events: auto !important; 
                cursor: pointer !important; 
                opacity: 1 !important;
                visibility: visible !important;
                display: inline-block !important;
                position: relative !important;
                z-index: auto !important;
                background: transparent !important;
                border: none !important;
                outline: none !important;
            }
            [disabled], [readonly], .disabled, .readonly {
                pointer-events: auto !important;
                cursor: pointer !important;
                opacity: 1 !important;
                background: transparent !important;
            }
            a, a:visited, a:hover, a:active {
                color: #0066cc !important;
                text-decoration: underline !important;
                cursor: pointer !important;
                pointer-events: auto !important;
            }
        \`;
        document.head.appendChild(style);
        
        // Force-aktiviere ALLE Elemente
        const allElements = document.querySelectorAll('*');
        allElements.forEach(element => {
            // Entferne alle Disabled-Attribute
            element.removeAttribute('disabled');
            element.removeAttribute('readonly');
            element.removeAttribute('inert');
            element.removeAttribute('aria-disabled');
            
            // Setze interaktive Properties
            if (element.style) {
                element.style.pointerEvents = 'auto';
                element.style.cursor = ['BUTTON', 'A', 'INPUT', 'SELECT', 'TEXTAREA'].includes(element.tagName) ? 'pointer' : 'auto';
                element.style.opacity = '1';
                element.style.visibility = 'visible';
            }
            
            if (element.disabled !== undefined) {
                element.disabled = false;
            }
            
            // Aktiviere Event-Handler
            if (element.hasAttribute('onclick') || 
                element.hasAttribute('data-ved') ||
                element.hasAttribute('jsaction') ||
                element.getAttribute('role') === 'button') {
                element.style.cursor = 'pointer';
                element.style.pointerEvents = 'auto';
            }
        });
        
        console.log(\`✅ Processed \${allElements.length} elements for ultimate interaction\`);
    }
    
    // 🍪 ULTIMATIVE COOKIE-FIXES
    function fixUltimateCookies() {
        console.log('🍪 Applying ULTIMATE cookie fixes...');
        
        // Erweiterte Cookie-Selektoren für Auto-Accept
        const acceptSelectors = [
            '#L2AGLb', '#VnjCcb', '#W0wltc', // Google
            'button[aria-label*="Accept"]', 'button[aria-label*="Akzeptieren"]',
            'button[aria-label*="Accept all"]', 'button[aria-label*="Alle akzeptieren"]',
            'button[aria-label*="I agree"]', 'button[aria-label*="Ich stimme zu"]',
            '[data-ved] button:first-child', '[jsname] button:first-child',
            '[data-testid*="accept"]', '[data-testid*="agree"]',
            'button[type="submit"]:first-child', 'input[type="submit"]:first-child',
            '.cookie-accept', '.consent-accept', '.agree-button',
            '[id*="accept"]', '[class*="accept"]', '[id*="agree"]', '[class*="agree"]'
        ];
        
        acceptSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(btn => {
                btn.style.pointerEvents = 'auto';
                btn.disabled = false;
                btn.style.cursor = 'pointer';
                btn.style.opacity = '1';
                btn.style.visibility = 'visible';
                btn.removeAttribute('disabled');
                btn.removeAttribute('readonly');
                
                // Auto-click nach 500ms
                if (btn.textContent && 
                    (btn.textContent.toLowerCase().includes('akzeptieren') || 
                     btn.textContent.toLowerCase().includes('accept') ||
                     btn.textContent.toLowerCase().includes('agree') ||
                     btn.textContent.toLowerCase().includes('zustimmen'))) {
                    setTimeout(() => {
                        console.log('🍪 Auto-accepting cookies:', selector);
                        btn.click();
                    }, 500);
                }
            });
        });
    }
    
        // 🔗 ULTIMATIVE LINK-FIXES
        function fixUltimateLinks() {
            console.log('🔗 Applying ULTIMATE link fixes...');
            
            const allLinks = document.querySelectorAll('a, [href], [data-href], [onclick*="location"], [onclick*="window.open"], [onclick*="href"]');
            allLinks.forEach(link => {
                link.style.pointerEvents = 'auto';
                link.style.cursor = 'pointer';
                link.style.textDecoration = 'underline';
                link.style.color = '#0066cc';
                link.style.opacity = '1';
                link.style.visibility = 'visible';
                
                // Entferne Beschränkungen
                link.removeAttribute('target');
                link.removeAttribute('rel');
                link.removeAttribute('disabled');
            
            // Stelle sicher, dass Links funktionieren
            if (!link.href && link.hasAttribute('data-href')) {
                link.href = link.getAttribute('data-href');
            }
        });
        
        console.log(\`✅ Fixed \${allLinks.length} links for ultimate compatibility\`);
    }
    
    // 🔍 ULTIMATIVE GOOGLE-SUCHE KEYBOARD-FIXES
    function fixGoogleSearchKeyboard() {
        console.log('🔍 Applying ULTIMATE Google Search keyboard fixes...');
        
        // Google-spezifische Suchfeld-Selektoren
        const googleSearchSelectors = [
            'input[name="q"]', // Haupt-Suchfeld
            'input[title*="Suche"]', 
            'input[title*="Search"]',
            'input[aria-label*="Suche"]',
            'input[aria-label*="Search"]',
            'textarea[name="q"]',
            '.gLFyf', // Google-spezifische CSS-Klasse
            '#APjFqb' // Weitere Google-ID
        ];
        
        googleSearchSelectors.forEach(selector => {
            const elements = document.querySelectorAll(selector);
            elements.forEach(searchField => {
                console.log('🔍 Found Google search field:', selector);
                
                // Event-Listener für Enter-Taste
                searchField.addEventListener('keydown', function(e) {
                    console.log('🔍 Key pressed in Google search:', e.key);
                    if (e.key === 'Enter') {
                        e.preventDefault();
                        e.stopPropagation();
                        
                        // Versuche verschiedene Google-Such-Buttons zu finden und klicken
                        const searchButtons = [
                            'input[name="btnK"]', // Standard "Google Search" Button
                            'input[value*="Google"]',
                            'input[value*="Suche"]',
                            'input[type="submit"]',
                            'button[type="submit"]',
                            '[jsname="Tg7LZd"]', // Google-spezifische Buttons
                            '[data-ved] input[type="submit"]',
                            '.FPdoLc input[type="submit"]' // Weitere Google-Button-Selektor
                        ];
                        
                        let buttonClicked = false;
                        for (const buttonSelector of searchButtons) {
                            const button = document.querySelector(buttonSelector);
                            if (button && button.offsetParent !== null) { // Button ist sichtbar
                                console.log('🔍 Clicking Google search button:', buttonSelector);
                                button.click();
                                buttonClicked = true;
                                break;
                            }
                        }
                        
                        // Fallback: Form submiten
                        if (!buttonClicked) {
                            const form = searchField.closest('form');
                            if (form) {
                                console.log('🔍 Submitting Google search form');
                                form.submit();
                            } else {
                                // Ultimativer Fallback: Neue URL erstellen und navigieren
                                const query = searchField.value.trim();
                                if (query) {
                                    const searchUrl = \`https://www.google.com/search?q=\${encodeURIComponent(query)}\`;
                                    console.log('🔍 Fallback: Navigating to search URL:', searchUrl);
                                    window.location.href = searchUrl;
                                }
                            }
                        }
                    }
                });
                
                // Zusätzliche Enter-Event-Handler
                searchField.addEventListener('keypress', function(e) {
                    if (e.key === 'Enter') {
                        console.log('🔍 Enter keypress in Google search detected');
                        e.preventDefault();
                        e.stopPropagation();
                    }
                });
                
                // Stelle sicher, dass das Feld fokussierbar ist
                searchField.style.pointerEvents = 'auto';
                searchField.disabled = false;
                searchField.removeAttribute('disabled');
                searchField.tabIndex = 0;
            });
        });
        
        console.log(\`🔍 Applied keyboard fixes to \${document.querySelectorAll(googleSearchSelectors.join(', ')).length} Google search fields\`);
    }
    
    // 📑 ULTIMATIVE TAB-FIXES
    function fixUltimateTabs() {
        console.log('📑 Applying ULTIMATE tab fixes...');
        
        // Überschreibe window.open für ultimative Kompatibilität
        if (typeof window !== 'undefined') {
            const originalWindowOpen = window.open;
            window.open = function(url, name, features) {
                console.log('🔗 Window.open intercepted for ultimate compatibility:', url);
                try {
                    return originalWindowOpen.call(this, url, name || '_blank', features);
                } catch (e) {
                    console.log('🔄 Window.open fallback');
                    window.location.href = url;
                    return window;
                }
            };
            
            // Entferne Popup-Blocker
            Object.defineProperty(window, 'opener', {
                get: function() { return window; },
                set: function() { return true; },
                configurable: true
            });
        }
    }
    
    // 🔄 ULTIMATIVER MUTATION-OBSERVER
    const ultimateObserver = new MutationObserver(function(mutations) {
        mutations.forEach(function(mutation) {
            if (mutation.type === 'childList') {
                mutation.addedNodes.forEach(function(node) {
                    if (node.nodeType === 1) {
                        // Aktiviere alle neuen Elemente sofort
                        const newElements = node.querySelectorAll ? node.querySelectorAll('*') : [];
                        newElements.forEach(element => {
                            element.style.pointerEvents = 'auto';
                            element.disabled = false;
                            element.removeAttribute('disabled');
                            element.removeAttribute('readonly');
                        });
                        
                        // Prüfe auf neue Cookie-Popups
                        if (node.id && (node.id.includes('cookie') || node.id.includes('consent') || node.id.includes('banner'))) {
                            setTimeout(fixUltimateCookies, 50);
                        }
                        
                        // Prüfe auf neue Links
                        if (node.tagName === 'A' || (node.querySelector && node.querySelector('a'))) {
                            setTimeout(fixUltimateLinks, 50);
                        }
                    }
                });
            }
            
            // Reagiere auf Attribut-Änderungen
            if (mutation.type === 'attributes') {
                const element = mutation.target;
                if (['disabled', 'readonly', 'style', 'class', 'aria-disabled'].includes(mutation.attributeName)) {
                    element.style.pointerEvents = 'auto';
                    element.disabled = false;
                    element.removeAttribute('disabled');
                    element.removeAttribute('readonly');
                    element.removeAttribute('aria-disabled');
                }
            }
        });
    });
    
    // 🚀 GLOBALE HELPER-FUNKTIONEN
    window.oraBrowserUltimateFixAll = function() {
        enableUltimateInteractions();
        fixUltimateCookies();
        fixUltimateLinks();
        fixGoogleSearchKeyboard();
        fixUltimateTabs();
        console.log('🚀 Ora Browser: ULTIMATE fixes reapplied manually');
    };
    
    window.oraBrowserUltimateClick = function(selector) {
        const elements = document.querySelectorAll(selector);
        elements.forEach(el => {
            el.style.pointerEvents = 'auto';
            el.disabled = false;
            el.removeAttribute('disabled');
            el.click();
            console.log('🖱️ Ultimate force-clicked:', selector);
        });
    };
    
    // Initialisierung
    function initializeUltimateFixes() {
        enableUltimateInteractions();
        fixUltimateCookies();
        fixUltimateLinks();
        fixGoogleSearchKeyboard();
        fixUltimateTabs();
        
        // Starte ultimativen Observer
        if (document.body) {
            ultimateObserver.observe(document.body, { 
                childList: true, 
                subtree: true, 
                attributes: true,
                attributeFilter: ['disabled', 'readonly', 'style', 'class', 'aria-disabled']
            });
        }
    }
    
    if (document.readyState === 'loading') {
        document.addEventListener('DOMContentLoaded', initializeUltimateFixes);
    } else {
        initializeUltimateFixes();
    }
    
    // Wiederhole Fixes alle 500ms für ultimative Kompatibilität
    setInterval(function() {
        enableUltimateInteractions();
        fixUltimateCookies();
        fixUltimateLinks();
        fixGoogleSearchKeyboard();
    }, 500);
    
    console.log('✅ Ora Browser Frontend: ULTIMATE security bypasses applied - ALL restrictions removed');
})();
</script>
        `;
        
        // Injiziere das ultimative Security-Script
        if (optimized.includes('</head>')) {
            optimized = optimized.replace('</head>', ultimateSecurityScript + '</head>');
        } else if (optimized.includes('<body>')) {
            optimized = optimized.replace('<body>', ultimateSecurityScript + '<body>');
        } else {
            optimized = ultimateSecurityScript + optimized;
        }
        
        // 🚫 DEAKTIVIERE PROBLEMATISCHE SCRIPTS AGGRESSIV
        optimized = optimized.replace(/window\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/top\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/parent\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/document\.location\s*=\s*["'][^"']*["']/g, '// Navigation disabled in embedded mode');
        optimized = optimized.replace(/location\.href\s*=\s*["'][^"']*["']/g, '// Redirect disabled in embedded mode');
        optimized = optimized.replace(/location\.replace\s*\(\s*["'][^"']*["']\s*\)/g, '// Redirect disabled in embedded mode');
        
        // 🎯 OPTIMIERE FRAME-BUSTING CODE AGGRESSIV
        optimized = optimized.replace(/if\s*\(\s*top\s*!=\s*self\s*\)/g, 'if (false)');
        optimized = optimized.replace(/if\s*\(\s*window\s*!=\s*top\s*\)/g, 'if (false)');
        optimized = optimized.replace(/if\s*\(\s*parent\s*!=\s*window\s*\)/g, 'if (false)');
        optimized = optimized.replace(/if\s*\(\s*self\s*!=\s*top\s*\)/g, 'if (false)');
        optimized = optimized.replace(/top\.location\.href/g, 'window.location.href');
        optimized = optimized.replace(/parent\.location\.href/g, 'window.location.href');
        
        return optimized;
    }

    debugTestElements() {
        console.log('🔧 Testing all UI elements...');
        
        const elements = [
            'content-area',
            'welcome-screen', 
            'content-container',
            'url-input',
            'navigation-buttons',
            'tabs-container'
        ];
        
        elements.forEach(id => {
            const element = document.getElementById(id);
            console.log(`🔧 Element ${id}:`, element ? 'EXISTS' : 'MISSING');
            if (element) {
                console.log(`   - Display: ${element.style.display || 'default'}`);
                console.log(`   - Visibility: ${element.style.visibility || 'default'}`);
                console.log(`   - Dimensions: ${element.offsetWidth}x${element.offsetHeight}`);
            }
        });
    }

    // 🔍 SCHRITT 1: FRONTEND-DEBUGGING FUNKTIONEN
    debugContentDisplay() {
        console.log('🔧 === FRONTEND DEBUG START ===');
        
        // 1. Prüfe DOM-Elemente
        const contentArea = document.getElementById('content-area');
        const welcomeScreen = document.getElementById('welcome-screen');
        const contentContainer = document.getElementById('content-container');
        
        console.log('🔧 Debug: Content area exists?', !!contentArea);
        console.log('🔧 Debug: Welcome screen exists?', !!welcomeScreen);
        console.log('🔧 Debug: Content container exists?', !!contentContainer);
        
        if (contentArea) {
            console.log('🔧 Content area display:', contentArea.style.display || 'default');
            console.log('🔧 Content area visibility:', contentArea.style.visibility || 'default');
            console.log('🔧 Content area dimensions:', contentArea.offsetWidth + 'x' + contentArea.offsetHeight);
        }
        
        if (welcomeScreen) {
            console.log('🔧 Welcome screen display:', welcomeScreen.style.display || 'default');
            console.log('🔧 Welcome screen visibility:', welcomeScreen.style.visibility || 'default');
            console.log('🔧 Welcome screen hidden?', welcomeScreen.style.display === 'none');
        }
        
        if (contentContainer) {
            console.log('🔧 Content container display:', contentContainer.style.display || 'default');
            console.log('🔧 Content container visibility:', contentContainer.style.visibility || 'default');
            console.log('🔧 Content container innerHTML length:', contentContainer.innerHTML.length);
            console.log('🔧 Content container preview:', contentContainer.innerHTML.substring(0, 200));
        }
        
        // 2. Prüfe alle Elemente mit IDs
        console.log('🔧 All elements with IDs:');
        const allElements = document.querySelectorAll('[id]');
        allElements.forEach(el => {
            console.log(`   - ${el.id}: ${el.tagName} (${el.offsetWidth}x${el.offsetHeight})`);
        });
        
        console.log('🔧 === FRONTEND DEBUG END ===');
    }

    // 🌐 SCHRITT 2: NAVIGATION TESTEN
    async testNavigation(testUrl = 'https://google.com') {
        console.log('🌐 === NAVIGATION TEST START ===');
        console.log('🌐 Testing navigation to:', testUrl);
        
        try {
            // 1. Teste navigateToUrl direkt
            console.log('🌐 Step 1: Calling navigateToUrl...');
            const result = await this.navigateToUrl(testUrl);
            console.log('🌐 Navigation result:', result);
            
            // 2. Warte kurz und prüfe Ergebnis
            setTimeout(() => {
                console.log('🌐 Step 2: Checking result after 2 seconds...');
                this.debugContentDisplay();
                
                // 3. Prüfe ob Content geladen wurde
                const contentContainer = document.getElementById('content-container');
                if (contentContainer && contentContainer.innerHTML.length > 100) {
                    console.log('✅ Navigation test SUCCESSFUL - Content loaded!');
                } else {
                    console.log('❌ Navigation test FAILED - No content displayed');
                }
            }, 2000);
            
        } catch (error) {
            console.error('❌ Navigation test ERROR:', error);
        }
        
        console.log('🌐 === NAVIGATION TEST END ===');
    }

    // 📱 SCHRITT 3: DISPLAY-LOGIC ÜBERPRÜFEN
    testDisplayLogic() {
        console.log('📱 === DISPLAY LOGIC TEST START ===');
        
        // 1. Teste mit Dummy-Content
        const testContent = `
            <!DOCTYPE html>
            <html>
            <head><title>Test Content</title></head>
            <body>
                <h1>🧪 TEST CONTENT LOADED SUCCESSFULLY!</h1>
                <p>Wenn Sie das sehen, funktioniert die Display-Logic!</p>
                <p>Timestamp: ${new Date().toISOString()}</p>
            </body>
            </html>
        `;
        
        console.log('📱 Step 1: Testing displayContent with dummy content...');
        console.log('📱 Test content length:', testContent.length);
        
        // 2. Rufe displayContent auf
        this.displayContent(testContent);
        
        // 3. Prüfe Ergebnis nach kurzer Zeit
        setTimeout(() => {
            console.log('📱 Step 2: Checking display result...');
            const contentContainer = document.getElementById('content-container');
            
            if (contentContainer) {
                const hasTestContent = contentContainer.innerHTML.includes('TEST CONTENT LOADED SUCCESSFULLY');
                console.log('📱 Test content displayed?', hasTestContent);
                
                if (hasTestContent) {
                    console.log('✅ Display logic test SUCCESSFUL!');
                } else {
                    console.log('❌ Display logic test FAILED - Content not displayed');
                    console.log('📱 Actual content:', contentContainer.innerHTML.substring(0, 200));
                }
            } else {
                console.log('❌ Display logic test FAILED - Content container not found');
            }
        }, 1000);
        
        console.log('📱 === DISPLAY LOGIC TEST END ===');
    }

    // 🔧 MASTER DEBUG FUNCTION - Führt alle Tests aus
    runAllDebugTests() {
        console.log('🚀 === RUNNING ALL DEBUG TESTS ===');
        
        // 1. Frontend-Debugging
        this.debugContentDisplay();
        
        // 2. Display-Logic Test
        setTimeout(() => {
            this.testDisplayLogic();
        }, 1000);
        
        // 3. Navigation Test
        setTimeout(() => {
            this.testNavigation('https://google.com');
        }, 3000);
        
        // 4. Weitere Tests nach 5 Sekunden
        setTimeout(() => {
            console.log('🔧 Final debug check...');
            this.debugContentDisplay();
        }, 8000);
        
        console.log('🚀 All debug tests scheduled. Check console for results.');
    }
}

// 🚀 BROWSER INITIALISIERUNG
document.addEventListener('DOMContentLoaded', async () => {
    console.log('🌐 DOM Content Loaded - Starting Ora Browser...');
    
    // Warte auf Tauri API
    if (await waitForTauri()) {
        console.log('✅ Tauri API ready - initializing browser...');
        window.oraBrowser = new OraBrowser();
        await window.oraBrowser.init();
    } else {
        console.error('❌ Tauri API not available - running in fallback mode');
        window.oraBrowser = new OraBrowser();
        window.oraBrowser.initializeUI();
    }
});

// Global error handler
window.addEventListener('error', (e) => {
    console.error('🚨 Global error:', e.error);
});

// Unhandled promise rejection handler
window.addEventListener('unhandledrejection', (e) => {
    console.error('🚨 Unhandled promise rejection:', e.reason);
});

console.log('📜 Ora Browser script loaded - Ready for initialization');

// Message-Handler für iframe-Kommunikation
window.addEventListener('message', (event) => {
    console.log('📨 Message received from iframe:', event.data);
    
    if (event.data && event.data.type) {
        console.log('📨 Processing message type:', event.data.type);
        
        switch (event.data.type) {
            case 'navigate':
                console.log('📨 🌐 Navigation request:', event.data.url);
                if (window.oraBrowser) {
                    if (event.data.method && event.data.method !== 'GET') {
                        // POST oder andere HTTP-Methoden
                        console.log('📨 Using navigateWithMethod');
                        window.oraBrowser.navigateWithMethod(event.data.url, event.data.method, event.data.data || '');
                    } else {
                        // Standard GET-Navigation
                        console.log('📨 Using navigateToUrl');
                        window.oraBrowser.navigateToUrl(event.data.url);
                    }
                } else {
                    console.error('📨 ❌ oraBrowser not available');
                }
                break;
                
            case 'openTab':
                console.log('📨 📑 New tab request:', event.data.url);
                if (window.oraBrowser) {
                    window.oraBrowser.createNewTab('Loading...', event.data.url);
                    window.oraBrowser.navigateToUrl(event.data.url);
                } else {
                    console.error('📨 ❌ oraBrowser not available for new tab');
                }
                break;
                
            case 'updateTitle':
                if (event.data.title && window.oraBrowser) {
                    console.log('📨 📝 Title update:', event.data.title);
                    window.oraBrowser.updateActiveTabInfo(event.data.title);
                }
                break;
                
            default:
                console.log('🤷 Unknown message type:', event.data.type);
        }
    } else {
        console.log('📨 Message without type or data:', event);
    }
}); 

// 🚀 GLOBALE DEBUG-FUNKTIONEN FÜR BROWSER-KONSOLE
window.debugOra = {
    // 🔍 Frontend-Debugging
    checkElements: () => {
        if (window.oraBrowser) {
            window.oraBrowser.debugContentDisplay();
        } else {
            console.error('❌ oraBrowser instance not found!');
        }
    },
    
    // 🌐 Navigation testen
    testNav: (url = 'https://google.com') => {
        if (window.oraBrowser) {
            window.oraBrowser.testNavigation(url);
        } else {
            console.error('❌ oraBrowser instance not found!');
        }
    },
    
    // 📱 Display-Logic testen
    testDisplay: () => {
        if (window.oraBrowser) {
            window.oraBrowser.testDisplayLogic();
        } else {
            console.error('❌ oraBrowser instance not found!');
        }
    },
    
    // 🚀 Alle Tests ausführen
    runAll: () => {
        if (window.oraBrowser) {
            window.oraBrowser.runAllDebugTests();
        } else {
            console.error('❌ oraBrowser instance not found!');
        }
    },
    
    // 🔧 Schnelle Navigation zu Test-URLs
    google: () => window.oraBrowser?.navigateToUrl('https://google.com'),
    duckduckgo: () => window.oraBrowser?.navigateToUrl('https://duckduckgo.com'),
    github: () => window.oraBrowser?.navigateToUrl('https://github.com'),
    
    // 📊 Status-Info
    info: () => {
        console.log('🔧 === ORA BROWSER DEBUG INFO ===');
        console.log('🔧 oraBrowser instance:', !!window.oraBrowser);
        console.log('🔧 Current URL:', window.oraBrowser?.currentUrl);
        console.log('🔧 Is loading:', window.oraBrowser?.isLoading);
        console.log('🔧 Available debug functions:');
        console.log('   - debugOra.checkElements() - Check DOM elements');
        console.log('   - debugOra.testNav(url) - Test navigation');
        console.log('   - debugOra.testDisplay() - Test display logic');
        console.log('   - debugOra.runAll() - Run all tests');
        console.log('   - debugOra.google() - Navigate to Google');
        console.log('   - debugOra.duckduckgo() - Navigate to DuckDuckGo');
        console.log('   - debugOra.github() - Navigate to GitHub');
    }
};

// 🎯 AUTOMATISCHE DEBUG-MELDUNG BEIM LADEN
console.log('🚀 === ORA BROWSER DEBUG TOOLS LOADED ===');
console.log('🔧 Type "debugOra.info()" for available debug functions');
console.log('🔧 Type "debugOra.runAll()" to run all diagnostic tests');
console.log('🔧 Type "debugOra.checkElements()" to check DOM elements');
console.log('🚀 ==========================================');