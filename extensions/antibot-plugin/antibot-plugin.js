/**
 * 🤖 Anti-Bot Strategies Plugin für Ora Browser
 * Erweiterte Anti-Bot-Strategien als nachladbares Plugin
 */

class AntiBotPlugin {
    constructor() {
        this.id = 'antibot-strategies';
        this.name = '🤖 Anti-Bot Strategies';
        this.version = '1.0.0';
        this.enabled = true;
        
        // Plugin-spezifische Konfiguration
        this.config = {
            enabled: true,
            strategies: new Set(['google', 'cloudflare', 'yandex', 'banking', 'social']),
            ethicalMode: true,
            autoDetect: true,
            statisticsEnabled: true
        };
        
        // Statistiken
        this.stats = {
            totalRequests: 0,
            successfulBypasses: 0,
            blockedAttempts: 0,
            ethicalViolationsPrevented: 0,
            strategiesUsed: new Map()
        };
        
        // Event-Handler
        this.eventHandlers = new Map();
        
        console.log('🔌 Anti-Bot Plugin initialized');
    }
    
    /**
     * Plugin-Aktivierung
     */
    async activate() {
        try {
            console.log('🔌 Activating Anti-Bot Plugin...');
            
            // Konfiguration laden
            await this.loadConfiguration();
            
            // Event-Listener registrieren
            this.registerEventHandlers();
            
            // UI-Integration
            await this.integrateUI();
            
            // Backend-Integration
            await this.registerBackendHandlers();
            
            console.log('✅ Anti-Bot Plugin activated successfully');
            return true;
            
        } catch (error) {
            console.error('❌ Failed to activate Anti-Bot Plugin:', error);
            return false;
        }
    }
    
    /**
     * Plugin-Deaktivierung
     */
    async deactivate() {
        try {
            console.log('🔌 Deactivating Anti-Bot Plugin...');
            
            // Event-Listener entfernen
            this.unregisterEventHandlers();
            
            // UI-Elemente entfernen
            await this.removeUI();
            
            // Konfiguration speichern
            await this.saveConfiguration();
            
            console.log('✅ Anti-Bot Plugin deactivated successfully');
            return true;
            
        } catch (error) {
            console.error('❌ Failed to deactivate Anti-Bot Plugin:', error);
            return false;
        }
    }
    
    /**
     * Konfiguration laden
     */
    async loadConfiguration() {
        try {
            // Aus Ora Browser Storage laden
            const savedConfig = await oraBrowser.storage.get('antibot-plugin-config');
            if (savedConfig) {
                this.config = { ...this.config, ...savedConfig };
                this.config.strategies = new Set(savedConfig.strategies || []);
            }
            
            // Statistiken laden
            const savedStats = await oraBrowser.storage.get('antibot-plugin-stats');
            if (savedStats) {
                this.stats = { ...this.stats, ...savedStats };
                this.stats.strategiesUsed = new Map(savedStats.strategiesUsed || []);
            }
            
            console.log('📊 Plugin configuration loaded:', this.config);
            
        } catch (error) {
            console.error('❌ Failed to load configuration:', error);
        }
    }
    
    /**
     * Konfiguration speichern
     */
    async saveConfiguration() {
        try {
            const configToSave = {
                ...this.config,
                strategies: Array.from(this.config.strategies)
            };
            
            const statsToSave = {
                ...this.stats,
                strategiesUsed: Array.from(this.stats.strategiesUsed.entries())
            };
            
            await oraBrowser.storage.set('antibot-plugin-config', configToSave);
            await oraBrowser.storage.set('antibot-plugin-stats', statsToSave);
            
            console.log('💾 Plugin configuration saved');
            
        } catch (error) {
            console.error('❌ Failed to save configuration:', error);
        }
    }
    
    /**
     * Event-Handler registrieren
     */
    registerEventHandlers() {
        // Navigation-Events
        this.eventHandlers.set('beforeNavigate', this.onBeforeNavigate.bind(this));
        this.eventHandlers.set('navigationCompleted', this.onNavigationCompleted.bind(this));
        this.eventHandlers.set('navigationFailed', this.onNavigationFailed.bind(this));
        
        // Proxy-Events
        this.eventHandlers.set('proxyRequest', this.onProxyRequest.bind(this));
        this.eventHandlers.set('proxyResponse', this.onProxyResponse.bind(this));
        
        // UI-Events
        this.eventHandlers.set('configChanged', this.onConfigChanged.bind(this));
        
        // Event-Listener bei Ora Browser registrieren
        for (const [event, handler] of this.eventHandlers) {
            oraBrowser.events.on(event, handler);
        }
        
        console.log('🎯 Event handlers registered:', Array.from(this.eventHandlers.keys()));
    }
    
    /**
     * Event-Handler entfernen
     */
    unregisterEventHandlers() {
        for (const [event, handler] of this.eventHandlers) {
            oraBrowser.events.off(event, handler);
        }
        this.eventHandlers.clear();
        console.log('🗑️ Event handlers unregistered');
    }
    
    /**
     * UI-Integration
     */
    async integrateUI() {
        try {
            // Plugin-Panel zur Browser-UI hinzufügen
            const pluginPanel = await this.createPluginPanel();
            oraBrowser.ui.addPanel('antibot-plugin', pluginPanel);
            
            // Toolbar-Button hinzufügen
            const toolbarButton = await this.createToolbarButton();
            oraBrowser.ui.addToolbarButton('antibot-toggle', toolbarButton);
            
            // Kontext-Menü-Einträge hinzufügen
            const contextMenuItems = await this.createContextMenuItems();
            oraBrowser.ui.addContextMenuItems('antibot-plugin', contextMenuItems);
            
            console.log('🎨 UI integration completed');
            
        } catch (error) {
            console.error('❌ Failed to integrate UI:', error);
        }
    }
    
    /**
     * Plugin-Panel erstellen
     */
    async createPluginPanel() {
        return {
            id: 'antibot-plugin-panel',
            title: '🤖 Anti-Bot Strategies',
            content: `
                <div class="antibot-plugin-panel">
                    <div class="plugin-header">
                        <h3>🤖 Anti-Bot Strategies</h3>
                        <div class="plugin-status ${this.config.enabled ? 'enabled' : 'disabled'}">
                            ${this.config.enabled ? '✅ Aktiv' : '❌ Inaktiv'}
                        </div>
                    </div>
                    
                    <div class="plugin-controls">
                        <label class="plugin-toggle">
                            <input type="checkbox" id="antibot-plugin-enabled" ${this.config.enabled ? 'checked' : ''}>
                            <span class="toggle-slider"></span>
                            <span class="toggle-label">Plugin aktivieren</span>
                        </label>
                    </div>
                    
                    <div class="strategy-selection" ${!this.config.enabled ? 'style="display:none"' : ''}>
                        <h4>🎯 Strategien auswählen:</h4>
                        <div class="strategy-grid">
                            ${this.createStrategyCheckboxes()}
                        </div>
                    </div>
                    
                    <div class="ethical-controls" ${!this.config.enabled ? 'style="display:none"' : ''}>
                        <label class="ethical-toggle">
                            <input type="checkbox" id="ethical-mode" ${this.config.ethicalMode ? 'checked' : ''}>
                            <span>⚖️ Ethik-Modus aktivieren</span>
                        </label>
                    </div>
                    
                    <div class="plugin-statistics" ${!this.config.enabled ? 'style="display:none"' : ''}>
                        <h4>📊 Statistiken:</h4>
                        <div class="stats-grid">
                            <div class="stat-item">
                                <span class="stat-value">${this.stats.totalRequests}</span>
                                <span class="stat-label">Anfragen</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-value">${this.stats.successfulBypasses}</span>
                                <span class="stat-label">Erfolgreiche Bypasses</span>
                            </div>
                            <div class="stat-item">
                                <span class="stat-value">${this.stats.blockedAttempts}</span>
                                <span class="stat-label">Blockierte Versuche</span>
                            </div>
                        </div>
                    </div>
                </div>
            `,
            styles: this.getPluginStyles(),
            scripts: this.getPluginScripts()
        };
    }
    
    /**
     * Strategie-Checkboxen erstellen
     */
    createStrategyCheckboxes() {
        const strategies = [
            { id: 'google', name: '🔍 Google', description: 'Google Anti-Bot Protection' },
            { id: 'cloudflare', name: '🌩️ Cloudflare', description: 'Cloudflare Bypass' },
            { id: 'yandex', name: '🇷🇺 Yandex', description: 'Russische Sites' },
            { id: 'banking', name: '🏦 Banking', description: 'Finanz- und Bankenseiten' },
            { id: 'social', name: '📱 Social', description: 'Social Media Plattformen' }
        ];
        
        return strategies.map(strategy => `
            <label class="strategy-checkbox">
                <input type="checkbox" value="${strategy.id}" 
                       ${this.config.strategies.has(strategy.id) ? 'checked' : ''}>
                <span class="checkbox-label">
                    <span class="strategy-name">${strategy.name}</span>
                    <span class="strategy-description">${strategy.description}</span>
                </span>
            </label>
        `).join('');
    }
    
    /**
     * Backend-Handler registrieren
     */
    async registerBackendHandlers() {
        try {
            // Tauri Commands für Plugin registrieren
            await oraBrowser.backend.registerCommand('antibot_plugin_get_config', this.getConfig.bind(this));
            await oraBrowser.backend.registerCommand('antibot_plugin_update_config', this.updateConfig.bind(this));
            await oraBrowser.backend.registerCommand('antibot_plugin_get_stats', this.getStats.bind(this));
            await oraBrowser.backend.registerCommand('antibot_plugin_reset_stats', this.resetStats.bind(this));
            
            console.log('🔗 Backend handlers registered');
            
        } catch (error) {
            console.error('❌ Failed to register backend handlers:', error);
        }
    }
    
    /**
     * Navigation-Event Handler
     */
    async onBeforeNavigate(details) {
        if (!this.config.enabled) return;
        
        try {
            const url = details.url;
            const detectedStrategy = this.detectRequiredStrategy(url);
            
            if (detectedStrategy && this.config.strategies.has(detectedStrategy)) {
                console.log(`🎯 Detected strategy needed: ${detectedStrategy} for ${url}`);
                
                // Strategie anwenden
                const strategyConfig = await this.getStrategyConfig(detectedStrategy);
                await this.applyStrategy(details, strategyConfig);
                
                // Statistiken aktualisieren
                this.updateStatistics(detectedStrategy, 'applied');
            }
            
        } catch (error) {
            console.error('❌ Error in onBeforeNavigate:', error);
        }
    }
    
    /**
     * Erforderliche Strategie erkennen
     */
    detectRequiredStrategy(url) {
        const domain = new URL(url).hostname.toLowerCase();
        
        // Google-Strategien
        if (domain.includes('google.') || domain.includes('googleapis.') || domain.includes('googleusercontent.')) {
            return 'google';
        }
        
        // Banking-Strategien
        const bankingDomains = ['sparkasse', 'volksbank', 'commerzbank', 'deutsche-bank', 'paypal', 'stripe'];
        if (bankingDomains.some(bank => domain.includes(bank))) {
            return 'banking';
        }
        
        // Yandex-Strategien
        if (domain.includes('yandex.') || domain.includes('dzen.ru') || domain.endsWith('.ru')) {
            return 'yandex';
        }
        
        // Social Media-Strategien
        const socialDomains = ['facebook.', 'instagram.', 'twitter.', 'x.com', 'linkedin.', 'reddit.'];
        if (socialDomains.some(social => domain.includes(social))) {
            return 'social';
        }
        
        // Cloudflare-Erkennung (generisch)
        return 'cloudflare';
    }
    
    /**
     * Plugin-API für externe Nutzung
     */
    getAPI() {
        return {
            // Öffentliche Methoden
            isEnabled: () => this.config.enabled,
            getStrategies: () => Array.from(this.config.strategies),
            getStats: () => ({ ...this.stats }),
            
            // Konfiguration
            enableStrategy: (strategy) => this.enableStrategy(strategy),
            disableStrategy: (strategy) => this.disableStrategy(strategy),
            toggleStrategy: (strategy) => this.toggleStrategy(strategy),
            
            // Events
            on: (event, callback) => this.on(event, callback),
            off: (event, callback) => this.off(event, callback),
            
            // Utilities
            detectStrategy: (url) => this.detectRequiredStrategy(url),
            applyStrategy: (url, strategy) => this.applyStrategyByName(url, strategy)
        };
    }
}

// Plugin-Instanz erstellen und global verfügbar machen
window.AntiBotPlugin = new AntiBotPlugin();

// Auto-Aktivierung wenn Ora Browser bereit ist
if (typeof oraBrowser !== 'undefined') {
    oraBrowser.plugins.register('antibot-strategies', window.AntiBotPlugin);
} else {
    // Warten auf Ora Browser
    document.addEventListener('oraBrowserReady', () => {
        oraBrowser.plugins.register('antibot-strategies', window.AntiBotPlugin);
    });
}

console.log('🔌 Anti-Bot Plugin loaded and ready for activation'); 