/**
 * ZAKYXBrowser Bookmark Manager
 * Verantwortlich für Lesezeichen-Verwaltung, Backend-Sync und Bookmark-UI
 */

class BookmarkManager {
    constructor(core, navigationManager) {
        this.core = core;
        this.navigationManager = navigationManager;
        this.bookmarks = [];
        
        console.log('📚 Bookmark Manager initialized');
    }

    // Bookmark hinzufügen
    async addBookmark(title, url) {
        console.log('📚 Adding bookmark:', title, url);
        
        const normalizedUrl = this.core.normalizeUrl(url);
        const bookmark = {
            id: Date.now().toString(),
            title: title,
            url: normalizedUrl
        };
        
        this.bookmarks.push(bookmark);
        this.renderBookmarks();
        this.core.updateStatus(`Lesezeichen hinzugefügt: ${title}`);
        
        return true;
    }

    // Bookmark entfernen
    async removeBookmark(index) {
        if (index >= 0 && index < this.bookmarks.length) {
            const bookmark = this.bookmarks[index];
            
            // Versuche über Tauri Backend zu löschen
            if (this.core.checkTauriAPI()) {
                try {
                    await window.__TAURI__.core.invoke('remove_bookmark', {
                        bookmarkId: bookmark.id
                    });
                    
                    console.log('✅ Bookmark removed from backend:', bookmark.id);
                    this.core.updateStatus(`Lesezeichen gelöscht: ${bookmark.title}`);
                    
                    // Lade alle Bookmarks neu vom Backend
                    await this.loadBookmarksFromBackend();
                    return;
                    
                } catch (error) {
                    console.warn('⚠️ Backend bookmark removal failed, using frontend only:', error);
                }
            }
            
            // Fallback: Nur Frontend
            const removed = this.bookmarks.splice(index, 1)[0];
            this.renderBookmarks();
            this.core.updateStatus(`Lesezeichen entfernt: ${removed.title}`);
            console.log('📚 Bookmark removed (frontend only):', removed);
        }
    }

    // Bookmarks vom Backend laden
    async loadBookmarksFromBackend() {
        console.log('📚 === LOADING BOOKMARKS FROM BACKEND ===');
        
        const tauriAvailable = this.core.checkTauriAPI();
        
        if (!tauriAvailable) {
            console.log('🔄 No Tauri API - using default bookmarks');
            if (this.bookmarks.length === 0) {
                this.bookmarks = [
                    { id: '1', title: '🔍 Google', url: 'https://google.com' },
                    { id: '2', title: '👨‍💻 GitHub', url: 'https://github.com' },
                    { id: '3', title: '📖 Wikipedia', url: 'https://wikipedia.org' },
                    { id: '4', title: '🏠 HTML GUI', url: 'gui' }
                ];
                this.renderBookmarks();
                console.log('📚 Default bookmarks loaded and rendered');
            }
            return;
        }
        
        try {
            console.log('📚 Attempting to sync bookmarks...');
            // Verwende sync_bookmarks für bessere Synchronisation
            const backendBookmarks = await window.__TAURI__.core.invoke('sync_bookmarks');
            console.log('📚 Sync response:', backendBookmarks);
            
            if (backendBookmarks && Array.isArray(backendBookmarks)) {
                this.bookmarks = backendBookmarks;
                this.renderBookmarks();
                console.log(`✅ Synchronized ${backendBookmarks.length} bookmarks from persistent storage`);
                console.log('📚 Bookmarks loaded:', this.bookmarks);
            } else {
                console.log('⚠️ No bookmarks received from sync, trying get_bookmarks...');
                // Fallback: Versuche get_bookmarks
                const fallbackBookmarks = await window.__TAURI__.core.invoke('get_bookmarks');
                console.log('📚 Get bookmarks response:', fallbackBookmarks);
                
                if (fallbackBookmarks && Array.isArray(fallbackBookmarks)) {
                    this.bookmarks = fallbackBookmarks;
                    this.renderBookmarks();
                    console.log(`✅ Loaded ${fallbackBookmarks.length} bookmarks from backend (fallback)`);
                } else {
                    console.log('⚠️ No bookmarks from fallback either, using defaults');
                    this.useDefaultBookmarks();
                }
            }
            
        } catch (error) {
            console.warn('⚠️ Failed to sync bookmarks from backend:', error);
            this.useDefaultBookmarks();
        }
        
        console.log('📚 === END LOADING BOOKMARKS ===');
        console.log('📚 Final bookmark count:', this.bookmarks.length);
    }

    // Default Bookmarks verwenden
    useDefaultBookmarks() {
        this.bookmarks = [
            { id: '1', title: '🔍 Google', url: 'https://google.com' },
            { id: '2', title: '👨‍💻 GitHub', url: 'https://github.com' },
            { id: '3', title: '📖 Wikipedia', url: 'https://wikipedia.org' },
            { id: '4', title: '🏠 HTML GUI', url: 'gui' }
        ];
        this.renderBookmarks();
        console.log('📚 Default bookmarks loaded as fallback');
    }

    // Bookmarks rendern
    renderBookmarks() {
        console.log('📚 Rendering bookmarks:', this.bookmarks.length);
        
        const bookmarksContainer = document.getElementById('bookmarks-container');
        if (!bookmarksContainer) return;

        bookmarksContainer.innerHTML = '';
        
        this.bookmarks.forEach((bookmark) => {
            const bookmarkElement = document.createElement('button');
            bookmarkElement.className = 'bookmark-item';
            bookmarkElement.title = bookmark.title;
            bookmarkElement.textContent = bookmark.title;
            
            bookmarkElement.addEventListener('click', () => {
                this.navigationManager.navigateToUrl(bookmark.url);
            });
            
            bookmarksContainer.appendChild(bookmarkElement);
        });
    }

    // Event Listeners einrichten
    setupEventListeners() {
        const bookmarkBtn = document.getElementById('bookmark-btn');
        if (bookmarkBtn) {
            bookmarkBtn.addEventListener('click', () => {
                this.showBookmarkModal();
            });
        }
    }

    // Bookmark Modal anzeigen
    showBookmarkModal() {
        console.log('📚 Showing bookmark modal');
        // Implementation für Modal
    }
}

// Global verfügbar machen
window.BookmarkManager = BookmarkManager;

export default BookmarkManager; 