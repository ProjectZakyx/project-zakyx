/**
 * OraBrowser UI Module
 * Tab-Management und Bookmark-System
 */

import { StorageUtils, EventUtils, DomUtils } from './utils.js';

// Tab Manager
export class TabManager {
    constructor(core, navigationManager) {
        this.core = core;
        this.navigationManager = navigationManager;
        this.tabs = new Map();
        this.activeTabId = null;
        this.tabCounter = 0;
        
        console.log('📑 Tab Manager initialized');
    }

    // Neuen Tab erstellen
    createNewTab(title = 'Neuer Tab', url = 'about:blank') {
        const tabId = `tab-${++this.tabCounter}`;
        
        const tab = {
            id: tabId,
            title: title,
            url: url,
            isActive: false,
            isPinned: false,
            favicon: this.getFaviconForUrl(url),
            created: new Date()
        };
        
        this.tabs.set(tabId, tab);
        
        // Tab zur UI hinzufügen
        this.renderTabs();
        
        console.log('✨ Tab created:', tabId, title);
        return tabId;
    }

    // Tab mit URL erstellen
    createNewTabWithUrl(url, title = null) {
        const tabTitle = title || this.core.extractDomain(url) || 'Neuer Tab';
        const tabId = this.createNewTab(tabTitle, url);
        
        // Automatisch zu neuem Tab wechseln
        this.switchToTab(tabId);
        
        // URL laden
        if (url && url !== 'about:blank') {
            this.navigationManager.navigateToUrl(url);
        }
        
        return tabId;
    }

    // Zu Tab wechseln
    switchToTab(tabId) {
        // Aktuellen Tab deaktivieren
        if (this.activeTabId) {
            const currentTab = this.tabs.get(this.activeTabId);
            if (currentTab) {
                currentTab.isActive = false;
            }
        }
        
        // Neuen Tab aktivieren
        const newTab = this.tabs.get(tabId);
        if (newTab) {
            newTab.isActive = true;
            this.activeTabId = tabId;
            
            // URL laden falls nötig
            if (newTab.url && newTab.url !== 'about:blank') {
                this.navigationManager.navigateToUrl(newTab.url, false);
            }
            
            // Tab-UI aktualisieren
            this.renderTabs();
            this.updateUrlInput(newTab.url);
            
            console.log('🔄 Switched to tab:', tabId, newTab.title);
            return true;
        }
        
        console.warn('⚠️ Tab not found:', tabId);
        return false;
    }

    // Tab schließen
    closeTab(tabId) {
        const tab = this.tabs.get(tabId);
        if (!tab) {
            console.warn('⚠️ Cannot close tab - not found:', tabId);
            return false;
        }
        
        this.tabs.delete(tabId);
        
        // Wenn der aktive Tab geschlossen wird
        if (this.activeTabId === tabId) {
            this.activeTabId = null;
            
            // Zu anderem Tab wechseln
            const remainingTabs = Array.from(this.tabs.keys());
            if (remainingTabs.length > 0) {
                this.switchToTab(remainingTabs[0]);
            } else {
                // Neuen Tab erstellen wenn keine Tabs übrig
                const newTabId = this.createNewTab();
                this.switchToTab(newTabId);
            }
        }
        
        this.renderTabs();
        console.log('❌ Tab closed:', tabId);
        return true;
    }

    // Tabs rendern
    renderTabs() {
        const tabsContainer = DomUtils.safeGetElement('tabs-container');
        if (!tabsContainer) {
            console.warn('⚠️ Tabs container not found');
            return;
        }

        // Tabs HTML generieren
        const tabsArray = Array.from(this.tabs.values());
        const tabsHtml = tabsArray.map(tab => {
            const isActive = tab.isActive ? 'active' : '';
            const isPinned = tab.isPinned ? 'pinned' : '';
            const shortTitle = this.shortenTitle(tab.title);
            
            return `
                <div class="tab ${isActive} ${isPinned}" data-tab-id="${tab.id}" title="${tab.title}">
                    <div class="tab-favicon">${tab.favicon}</div>
                    <div class="tab-title">${shortTitle}</div>
                    <button class="tab-close" data-tab-id="${tab.id}" title="Tab schließen">×</button>
                </div>
            `;
        }).join('');

        tabsContainer.innerHTML = tabsHtml;
        
        // Event-Listener hinzufügen
        this.setupTabEventListeners();
        
        console.log('🖼️ Tabs rendered:', tabsArray.length);
    }

    // Titel kürzen
    shortenTitle(title, maxLength = 25) {
        return title.length > maxLength ? title.substring(0, maxLength) + '...' : title;
    }

    // Favicon für URL
    getFaviconForUrl(url) {
        if (!url || url === 'about:blank') return '📄';
        
        const faviconMap = {
            'google.com': '🔍',
            'github.com': '🐙',
            'stackoverflow.com': '📚',
            'youtube.com': '📺',
            'gmail.com': '📧',
            'wikipedia.org': '📖',
            'localhost': '🏠'
        };
        
        for (const [domain, icon] of Object.entries(faviconMap)) {
            if (url.includes(domain)) {
                return icon;
            }
        }
        
        return '🌐';
    }

    // Tab Event-Listeners
    setupTabEventListeners() {
        const tabsContainer = DomUtils.safeGetElement('tabs-container');
        if (!tabsContainer) return;

        // Tab-Klicks
        EventUtils.addListener(tabsContainer, 'click', (e) => {
            const tab = e.target.closest('.tab');
            if (tab && !e.target.classList.contains('tab-close')) {
                const tabId = tab.dataset.tabId;
                this.switchToTab(tabId);
            }
        });

        // Tab schließen
        EventUtils.addListener(tabsContainer, 'click', (e) => {
            if (e.target.classList.contains('tab-close')) {
                e.stopPropagation();
                const tabId = e.target.dataset.tabId;
                this.closeTab(tabId);
            }
        });

        // Rechtsklick-Menü
        EventUtils.addListener(tabsContainer, 'contextmenu', (e) => {
            const tab = e.target.closest('.tab');
            if (tab) {
                e.preventDefault();
                const tabId = tab.dataset.tabId;
                this.showTabContextMenu(tabId, e);
            }
        });

        console.log('🎯 Tab event listeners setup');
    }

    // URL-Input aktualisieren
    updateUrlInput(url) {
        const addressInput = DomUtils.safeGetElement('address-input');
        if (addressInput && url && url !== 'about:blank') {
            addressInput.value = url;
        }
    }

    // Initial Tab erstellen
    createInitialTab() {
        const tabId = this.createNewTab('Startseite', 'about:blank');
        this.switchToTab(tabId);
        return tabId;
    }

    // Tab Kontext-Menü
    showTabContextMenu(tabId, event) {
        const tab = this.tabs.get(tabId);
        if (!tab) return;

        const menuItems = [
            { label: '🔄 Tab neu laden', action: () => this.reloadTab(tabId) },
            { label: '📋 Tab duplizieren', action: () => this.duplicateTab(tabId) },
            { label: '📌 Tab anheften', action: () => this.pinTab(tabId) },
            { label: '❌ Tab schließen', action: () => this.closeTab(tabId) },
            { label: '❌ Andere Tabs schließen', action: () => this.closeOtherTabs(tabId) }
        ];

        // Einfaches Kontext-Menü
        const menu = document.createElement('div');
        menu.className = 'tab-context-menu';
        menu.style.cssText = `
            position: fixed;
            top: ${event.clientY}px;
            left: ${event.clientX}px;
            background: white;
            border: 1px solid #ccc;
            border-radius: 4px;
            box-shadow: 0 2px 10px rgba(0,0,0,0.1);
            z-index: 1000;
            min-width: 150px;
        `;

        menu.innerHTML = menuItems.map(item => 
            `<div class="menu-item" style="padding: 8px 12px; cursor: pointer; border-bottom: 1px solid #eee;">
                ${item.label}
            </div>`
        ).join('');

        // Event-Listener für Menü-Items
        menuItems.forEach((item, index) => {
            const menuItem = menu.children[index];
            EventUtils.addListener(menuItem, 'click', () => {
                item.action();
                document.body.removeChild(menu);
            });
        });

        // Menü schließen bei Klick außerhalb
        const closeMenu = (e) => {
            if (!menu.contains(e.target)) {
                document.body.removeChild(menu);
                document.removeEventListener('click', closeMenu);
            }
        };

        setTimeout(() => {
            document.addEventListener('click', closeMenu);
        }, 100);

        document.body.appendChild(menu);
    }

    // Tab neu laden
    reloadTab(tabId) {
        const tab = this.tabs.get(tabId);
        if (tab && tab.url) {
            if (this.activeTabId === tabId) {
                this.navigationManager.reload();
            } else {
                // Tab URL merken für späteren Reload
                tab.needsReload = true;
            }
        }
    }

    // Tab duplizieren
    duplicateTab(tabId) {
        const tab = this.tabs.get(tabId);
        if (tab) {
            this.createNewTabWithUrl(tab.url, tab.title + ' (Kopie)');
        }
    }

    // Tab anheften
    pinTab(tabId) {
        const tab = this.tabs.get(tabId);
        if (tab) {
            tab.isPinned = !tab.isPinned;
            this.renderTabs();
        }
    }

    // Andere Tabs schließen
    closeOtherTabs(keepTabId) {
        const tabsToClose = Array.from(this.tabs.keys()).filter(id => id !== keepTabId);
        tabsToClose.forEach(tabId => this.closeTab(tabId));
    }

    // Tab-Statistiken
    getTabStats() {
        const stats = {
            total: this.tabs.size,
            active: this.activeTabId,
            pinned: Array.from(this.tabs.values()).filter(tab => tab.isPinned).length
        };
        
        return stats;
    }
}

// Bookmark Manager
export class BookmarkManager {
    constructor(core, navigationManager) {
        this.core = core;
        this.navigationManager = navigationManager;
        this.bookmarks = [];
        
        console.log('📚 Bookmark Manager initialized');
    }

    // Bookmark hinzufügen
    async addBookmark(title, url) {
        if (!title || !url) {
            console.warn('⚠️ Invalid bookmark data');
            return false;
        }

        const bookmark = {
            id: Date.now().toString(),
            title: title.trim(),
            url: url.trim(),
            created: new Date(),
            folder: 'default'
        };

        this.bookmarks.push(bookmark);
        
        // Speichern
        const success = StorageUtils.set('zakyx-browser-bookmarks', this.bookmarks);
        if (success) {
            this.renderBookmarks();
            console.log('📌 Bookmark added:', title);
            return true;
        } else {
            this.bookmarks.pop(); // Rückgängig machen
            console.error('❌ Failed to save bookmark');
            return false;
        }
    }

    // Bookmark entfernen
    async removeBookmark(index) {
        if (index < 0 || index >= this.bookmarks.length) {
            console.warn('⚠️ Invalid bookmark index');
            return false;
        }

        const bookmark = this.bookmarks[index];
        this.bookmarks.splice(index, 1);
        
        // Speichern
        const success = StorageUtils.set('zakyx-browser-bookmarks', this.bookmarks);
        if (success) {
            this.renderBookmarks();
            console.log('🗑️ Bookmark removed:', bookmark.title);
            return true;
        } else {
            this.bookmarks.splice(index, 0, bookmark); // Rückgängig machen
            console.error('❌ Failed to remove bookmark');
            return false;
        }
    }

    // Bookmarks aus Backend laden
    async loadBookmarksFromBackend() {
        try {
            if (window.__TAURI__ && window.__TAURI__.core) {
                const result = await window.__TAURI__.core.invoke('get_bookmarks');
                if (result && Array.isArray(result)) {
                    this.bookmarks = result;
                    this.renderBookmarks();
                    console.log('📚 Bookmarks loaded from backend:', result.length);
                    return true;
                }
            }
            
            // Fallback: localStorage laden
            const savedBookmarks = StorageUtils.get('zakyx-browser-bookmarks');
            if (savedBookmarks && Array.isArray(savedBookmarks)) {
                this.bookmarks = savedBookmarks;
                this.renderBookmarks();
                console.log('📚 Bookmarks loaded from localStorage:', savedBookmarks.length);
                return true;
            } else {
                this.useDefaultBookmarks();
                return true;
            }
            
        } catch (error) {
            console.error('❌ Error loading bookmarks:', error);
            this.useDefaultBookmarks();
            return false;
        }
    }

    // Standard-Bookmarks verwenden
    useDefaultBookmarks() {
        this.bookmarks = [
            { id: '1', title: '🔍 Google', url: 'https://google.com', folder: 'default' },
            { id: '2', title: '👨‍💻 GitHub', url: 'https://github.com', folder: 'default' },
            { id: '3', title: '📖 Wikipedia', url: 'https://wikipedia.org', folder: 'default' }
        ];
        
        StorageUtils.set('zakyx-browser-bookmarks', this.bookmarks);
        this.renderBookmarks();
        console.log('📚 Default bookmarks loaded');
    }

    // Bookmarks rendern
    renderBookmarks() {
        const bookmarksContainer = DomUtils.safeGetElement('bookmarks-container');
        if (!bookmarksContainer) return;

        const bookmarksHtml = this.bookmarks.map((bookmark, index) => {
            const domain = this.core.extractDomain(bookmark.url);
            return `
                <div class="bookmark" data-url="${bookmark.url}" data-index="${index}" title="${bookmark.title} - ${bookmark.url}">
                    <span class="bookmark-title">${bookmark.title}</span>
                    <span class="bookmark-domain">${domain}</span>
                </div>
            `;
        }).join('');

        bookmarksContainer.innerHTML = bookmarksHtml;
        this.setupBookmarkEventListeners();
        
        console.log('🖼️ Bookmarks rendered:', this.bookmarks.length);
    }

    // Bookmark Event-Listeners
    setupBookmarkEventListeners() {
        const bookmarksContainer = DomUtils.safeGetElement('bookmarks-container');
        if (!bookmarksContainer) return;

        // Bookmark-Klicks
        EventUtils.addListener(bookmarksContainer, 'click', (e) => {
            const bookmark = e.target.closest('.bookmark');
            if (bookmark) {
                const url = bookmark.dataset.url;
                this.navigationManager.navigateToUrl(url);
                console.log('📌 Bookmark clicked:', url);
            }
        });
    }

    // Bookmark-Modal anzeigen
    showBookmarkModal() {
        const modal = DomUtils.safeGetElement('bookmark-modal');
        if (modal) {
            // Aktuelle URL und Titel vorausfüllen
            const titleInput = DomUtils.safeGetElement('bookmark-title');
            const urlInput = DomUtils.safeGetElement('bookmark-url');
            
            if (titleInput && urlInput) {
                const currentUrl = this.core.currentUrl;
                titleInput.value = this.core.extractDomain(currentUrl) || 'Neue Seite';
                urlInput.value = currentUrl || '';
            }
            
            modal.style.display = 'flex';
            if (titleInput) titleInput.focus();
        }
    }
}

console.log('📦 UI module loaded'); 