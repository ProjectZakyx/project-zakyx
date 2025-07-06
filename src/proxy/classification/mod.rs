// 🔍 URL Classification Module
// Klassifiziert URLs und Domains für optimale Strategy-Auswahl

pub mod domain_detector;
pub mod site_categories;

use domain_detector::DomainDetector;
use site_categories::SiteCategory;

/// Haupt-Klassifizierer für Domains und URLs
pub struct DomainClassifier {
    detector: DomainDetector,
}

impl DomainClassifier {
    pub fn new() -> Self {
        Self {
            detector: DomainDetector::new(),
        }
    }

    /// Klassifiziere eine URL komplett
    pub fn classify_url(&self, url: &str) -> SiteCategory {
        self.detector.classify_site(url)
    }

    /// Prüfe ob es eine Banking-Website ist
    pub fn is_banking_site(&self, url: &str) -> bool {
        matches!(self.classify_url(url), SiteCategory::Banking)
    }

    /// Prüfe ob es eine Government-Website ist
    pub fn is_government_site(&self, url: &str) -> bool {
        matches!(self.classify_url(url), SiteCategory::Government)
    }

    /// Prüfe ob es eine russische Website ist
    pub fn is_russian_site(&self, url: &str) -> bool {
        self.detector.is_russian_domain(url)
    }

    /// Prüfe ob es eine chinesische Website ist
    pub fn is_chinese_site(&self, url: &str) -> bool {
        self.detector.is_chinese_domain(url)
    }

    /// Prüfe ob es eine Developer-Website ist
    pub fn is_developer_site(&self, url: &str) -> bool {
        matches!(self.classify_url(url), SiteCategory::Developer)
    }

    /// Prüfe ob es eine Social Media-Website ist
    pub fn is_social_media_site(&self, url: &str) -> bool {
        matches!(self.classify_url(url), SiteCategory::SocialMedia)
    }

    /// Prüfe ob die Website CSP-Frame-Probleme hat
    pub fn has_csp_frame_issues(&self, url: &str) -> bool {
        self.detector.has_csp_restrictions(url)
    }

    /// Extrahiere Domain aus URL
    pub fn extract_domain(&self, url: &str) -> String {
        self.detector.extract_domain(url)
    }
}

impl Default for DomainClassifier {
    fn default() -> Self {
        Self::new()
    }
} 