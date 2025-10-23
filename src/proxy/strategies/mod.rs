// 🧠 Smart Proxy Strategies Module
// Verschiedene Verbindungsstrategien für unterschiedliche Websites

pub mod browser_simulation;
pub mod regional_strategies;
pub mod security_strategies;
pub mod mobile_strategies;

use crate::proxy::classification::DomainClassifier;
use browser_simulation::ConnectionStrategy;

/// Strategy-Selector für adaptive Verbindungsstrategien
pub struct StrategySelector;

impl StrategySelector {
    pub fn new() -> Self {
        Self
    }

    /// Generiere adaptive Strategien basierend auf URL und Domain-Klassifizierung
    pub fn generate_adaptive_strategies(
        &self,
        url: &str,
        classifier: &DomainClassifier,
    ) -> Vec<ConnectionStrategy> {
        let mut strategies = Vec::new();

        // Standard Browser-Simulation (universell)
        strategies.extend(browser_simulation::get_standard_strategies());

        // Regional-spezifische Strategien
        if classifier.is_russian_site(url) || classifier.is_chinese_site(url) {
            strategies.extend(regional_strategies::get_regional_strategies(url));
        }

        // Sicherheitsorientierte Strategien für Banking/Government
        if classifier.is_banking_site(url) || classifier.is_government_site(url) {
            strategies.extend(security_strategies::get_security_strategies());
        }

        // Mobile Fallback-Strategien
        strategies.extend(mobile_strategies::get_mobile_strategies());

        strategies
    }

    /// Wähle die beste Strategie für eine spezifische URL
    pub fn select_best_strategy(
        &self,
        url: &str,
        classifier: &DomainClassifier,
    ) -> Option<ConnectionStrategy> {
        self.generate_adaptive_strategies(url, classifier)
            .into_iter()
            .next()
    }
}

impl Default for StrategySelector {
    fn default() -> Self {
        Self::new()
    }
} 
