/**
 * ZAKYX Browser Tab Manager
 * Verantwortlich für Tab-Verwaltung, Tab-Switching und Tab-UI
 */

class TabManager {
    constructor(core, navigationManager) {
        this.core = core;
        this.navigationManager = navigationManager;
        this.tabs = [];
        this.nextTabId = 1;
        this.activeTabId = null;
        this.keyboardListenerAdded = false;
        
        console.log('📑 Tab Manager initialized');
    }

    // Neuen Tab erstellen
    createNewTab(title = 'Neuer Tab', url = 'about:blank') {
        const tabId = this.nextTabId++;
        const tab = {
            id: tabId,
            title: title,
            url: url,
            isActive: false,
            history: [],
            historyIndex: -1,
            favicon: '🌐'
        };
        
        this.tabs.push(tab);
        this.renderTabs();
        this.switchToTab(tabId);
        
        console.log(`📑 New tab created: ${tabId} - ${title} - ${url}`);
        return tabId;
    }

    // Tab mit URL erstellen und sofort navigieren
    createNewTabWithUrl(url, title = null) {
        const normalizedUrl = this.core.normalizeUrl(url);
        const tabTitle = title || this.core.extractDomain(normalizedUrl);
        
        console.log(`📑 Creating new tab with URL: ${normalizedUrl}`);
        
        const tabId = this.createNewTab(tabTitle, normalizedUrl);
        
        // Sofort zur URL navigieren
        setTimeout(() => {
            console.log(`📑 Navigating new tab ${tabId} to: ${normalizedUrl}`);
            this.navigationManager.navigateToUrl(normalizedUrl, true, true);
        }, 100);
        
        return tabId;
    }

    // Zu Tab wechseln
    switchToTab(tabId) {
        // Deaktiviere alle Tabs
        this.tabs.forEach(tab => tab.isActive = false);
        
        // Aktiviere den gewählten Tab
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab) {
            tab.isActive = true;
            this.activeTabId = tabId;
            this.core.currentUrl = tab.url;
            this.updateUrlInput(tab.url);
            this.core.updateStatus(`Tab aktiv: ${tab.title}`);
            
            console.log(`📑 Switching to tab ${tabId} with URL: ${tab.url}`);
            
            // Wenn es nicht der Welcome Screen ist, lade die URL
            if (tab.url !== 'about:blank' && tab.url !== '') {
                console.log(`📑 Loading content for tab: ${tab.url}`);
                this.navigationManager.navigateToUrl(tab.url, false, false);
            }
        }
        
        this.renderTabs();
        console.log(`📑 Switched to tab: ${tabId}`);
    }

    // Tab schließen
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

    // Tabs rendern
    renderTabs() {
        const tabsContainer = document.getElementById('tabs-container');
        if (!tabsContainer) return;
        
        tabsContainer.innerHTML = '';
        
        this.tabs.forEach(tab => {
            const tabElement = document.createElement('div');
            tabElement.className = `tab ${tab.isActive ? 'active' : ''}`;
            
            // Tab-Favicon und Titel
            const favicon = this.getFaviconForUrl(tab.url);
            const shortTitle = this.shortenTitle(tab.title);
            
            tabElement.innerHTML = `
                <span class="tab-favicon">${favicon}</span>
                <span class="tab-title" title="${tab.url}">${shortTitle}</span>
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
            
            // Middle-Click zum Schließen
            tabElement.addEventListener('mousedown', (e) => {
                if (e.button === 1) { // Middle mouse button
                    e.preventDefault();
                    this.closeTab(tab.id);
                }
            });
            
            tabsContainer.appendChild(tabElement);
        });
    }

    // Titel verkürzen
    shortenTitle(title, maxLength = 25) {
        if (title.length <= maxLength) return title;
        return title.substring(0, maxLength - 3) + '...';
    }

    // Favicon für URL bestimmen
    getFaviconForUrl(url) {
        if (!url || url === 'about:blank') return '🌐';
        
        if (url.includes('google.com')) return '🔍';
        if (url.includes('github.com')) return '🐙';
        if (url.includes('youtube.com')) return '📺';
        if (url.includes('wikipedia.org')) return '📚';
        if (url.includes('stackoverflow.com')) return '📱';
        if (url.includes('twitter.com') || url.includes('x.com')) return '🐦';
        if (url.includes('facebook.com')) return '📘';
        if (url.includes('linkedin.com')) return '💼';
        if (url.includes('reddit.com')) return '📋';
        if (url.includes('news.')) return '📰';
        
        // HTTPS/HTTP
        if (url.startsWith('https://')) return '🔒';
        if (url.startsWith('http://')) return '🔓';
        
        return '🌐';
    }

    // Tab-Management einrichten
    setupTabManagement() {
        // Neuer Tab Button
        const newTabBtn = document.getElementById('new-tab-btn');
        if (newTabBtn) {
            // Entferne alte Event-Listener
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
                        case '1':
                        case '2':
                        case '3':
                        case '4':
                        case '5':
                        case '6':
                        case '7':
                        case '8':
                        case '9':
                            e.preventDefault();
                            const tabIndex = parseInt(e.key) - 1;
                            if (this.tabs[tabIndex]) {
                                this.switchToTab(this.tabs[tabIndex].id);
                            }
                            break;
                    }
                }
            });
            this.keyboardListenerAdded = true;
        }
    }

    // Zum nächsten Tab wechseln
    switchToNextTab() {
        if (this.tabs.length <= 1) return;
        
        const currentIndex = this.tabs.findIndex(t => t.id === this.activeTabId);
        const nextIndex = (currentIndex + 1) % this.tabs.length;
        this.switchToTab(this.tabs[nextIndex].id);
    }

    // Zum vorherigen Tab wechseln
    switchToPrevTab() {
        if (this.tabs.length <= 1) return;
        
        const currentIndex = this.tabs.findIndex(t => t.id === this.activeTabId);
        const prevIndex = currentIndex === 0 ? this.tabs.length - 1 : currentIndex - 1;
        this.switchToTab(this.tabs[prevIndex].id);
    }

    // Aktive Tab-Info aktualisieren
    updateActiveTabInfo(title, url) {
        const activeTab = this.tabs.find(t => t.id === this.activeTabId);
        if (activeTab) {
            activeTab.title = title || this.getPageTitle() || 'Unbekannte Seite';
            activeTab.url = url || this.core.currentUrl;
            
            // Favicon aktualisieren
            activeTab.favicon = this.getFaviconForUrl(activeTab.url);
            
            this.renderTabs();
            console.log(`📑 Updated tab ${this.activeTabId}: ${activeTab.title}`);
        }
    }

    // Seitentitel extrahieren
    getPageTitle() {
        // Versuche Titel aus dem aktuellen Content zu extrahieren
        const contentArea = document.getElementById('content-area') || 
                           document.getElementById('content-container');
        
        if (contentArea) {
            const titleElement = contentArea.querySelector('title');
            if (titleElement) {
                return titleElement.textContent;
            }
            
            // Fallback: h1-Element suchen
            const h1Element = contentArea.querySelector('h1');
            if (h1Element) {
                return h1Element.textContent.substring(0, 50);
            }
        }
        
        // Fallback: Domain aus URL extrahieren
        if (this.core.currentUrl) {
            return this.core.extractDomain(this.core.currentUrl);
        }
        
        return 'Neue Seite';
    }

    // URL-Eingabe aktualisieren
    updateUrlInput(url) {
        const addressInput = document.getElementById('address-input');
        if (addressInput) {
            addressInput.value = url;
        }
    }

    // Ersten Tab erstellen
    createInitialTab() {
        this.createNewTab('Neuer Tab', 'about:blank');
    }

    // Tab-Kontextmenü
    showTabContextMenu(tabId, event) {
        event.preventDefault();
        
        const tab = this.tabs.find(t => t.id === tabId);
        if (!tab) return;
        
        // Entferne vorhandenes Kontextmenü
        const existingMenu = document.getElementById('tab-context-menu');
        if (existingMenu) {
            existingMenu.remove();
        }
        
        // Erstelle Kontextmenü
        const menu = document.createElement('div');
        menu.id = 'tab-context-menu';
        menu.style.cssText = `
            position: fixed;
            top: ${event.clientY}px;
            left: ${event.clientX}px;
            background: white;
            border: 1px solid #ccc;
            border-radius: 4px;
            box-shadow: 0 2px 8px rgba(0,0,0,0.2);
            z-index: 10000;
            min-width: 150px;
        `;
        
        const menuItems = [
            { text: '🔄 Neu laden', action: () => this.reloadTab(tabId) },
            { text: '📋 Duplizieren', action: () => this.duplicateTab(tabId) },
            { text: '📌 Anheften', action: () => this.pinTab(tabId) },
            { text: '---', action: null },
            { text: '❌ Schließen', action: () => this.closeTab(tabId) },
            { text: '❌ Andere schließen', action: () => this.closeOtherTabs(tabId) },
            { text: '❌ Rechts schließen', action: () => this.closeTabsToRight(tabId) }
        ];
        
        menuItems.forEach(item => {
            if (item.text === '---') {
                const separator = document.createElement('hr');
                separator.style.margin = '5px 0';
                menu.appendChild(separator);
            } else {
                const menuItem = document.createElement('div');
                menuItem.textContent = item.text;
                menuItem.style.cssText = `
                    padding: 8px 12px;
                    cursor: pointer;
                    font-size: 14px;
                `;
                
                menuItem.addEventListener('mouseenter', () => {
                    menuItem.style.backgroundColor = '#f0f0f0';
                });
                
                menuItem.addEventListener('mouseleave', () => {
                    menuItem.style.backgroundColor = 'transparent';
                });
                
                menuItem.addEventListener('click', () => {
                    item.action();
                    menu.remove();
                });
                
                menu.appendChild(menuItem);
            }
        });
        
        document.body.appendChild(menu);
        
        // Schließe Menü beim Klick außerhalb
        const closeMenu = (e) => {
            if (!menu.contains(e.target)) {
                menu.remove();
                document.removeEventListener('click', closeMenu);
            }
        };
        
        setTimeout(() => {
            document.addEventListener('click', closeMenu);
        }, 100);
    }

    // Tab-Aktionen
    reloadTab(tabId) {
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab && tab.url !== 'about:blank') {
            if (tab.isActive) {
                this.navigationManager.reload();
            } else {
                // Markiere Tab für Reload beim nächsten Aktivieren
                tab.needsReload = true;
            }
        }
    }

    duplicateTab(tabId) {
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab) {
            this.createNewTabWithUrl(tab.url, tab.title);
        }
    }

    pinTab(tabId) {
        const tab = this.tabs.find(t => t.id === tabId);
        if (tab) {
            tab.pinned = !tab.pinned;
            this.renderTabs();
        }
    }

    closeOtherTabs(keepTabId) {
        const tabsToClose = this.tabs.filter(t => t.id !== keepTabId);
        tabsToClose.forEach(tab => this.closeTab(tab.id));
    }

    closeTabsToRight(fromTabId) {
        const fromIndex = this.tabs.findIndex(t => t.id === fromTabId);
        if (fromIndex === -1) return;
        
        const tabsToClose = this.tabs.slice(fromIndex + 1);
        tabsToClose.forEach(tab => this.closeTab(tab.id));
    }

    // Tab-Statistiken
    getTabStats() {
        return {
            total: this.tabs.length,
            active: this.activeTabId,
            pinned: this.tabs.filter(t => t.pinned).length,
            hasUnsavedChanges: this.tabs.filter(t => t.hasUnsavedChanges).length
        };
    }
}

// Global verfügbar machen
window.TabManager = TabManager;

export default TabManager; 