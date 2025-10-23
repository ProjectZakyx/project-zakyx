// 📂 Site Categories
// Verschiedene Kategorien von Websites für Strategy-Auswahl

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SiteCategory {
    /// Banking und Finanz-Websites
    Banking,
    /// Government und Behörden-Websites
    Government,
    /// Developer und Tech-Websites
    Developer,
    /// Social Media Plattformen
    SocialMedia,
    /// E-Commerce Websites
    ECommerce,
    /// News und Media Websites
    News,
    /// Educational Websites
    Education,
    /// Healthcare Websites
    Healthcare,
    /// Legal und Compliance Websites
    Legal,
    /// Gaming Websites
    Gaming,
    /// Streaming und Entertainment
    Entertainment,
    /// Corporate Websites
    Corporate,
    /// Search Engines
    Search,
    /// Regional (Russland, China, etc.)
    Regional,
    /// Unknown oder Allgemein
    General,
}

impl SiteCategory {
    /// Beschreibung der Kategorie
    pub fn description(&self) -> &'static str {
        match self {
            SiteCategory::Banking => "Banking and Financial Services",
            SiteCategory::Government => "Government and Public Services",
            SiteCategory::Developer => "Developer and Technology",
            SiteCategory::SocialMedia => "Social Media Platforms",
            SiteCategory::ECommerce => "E-Commerce and Shopping",
            SiteCategory::News => "News and Media",
            SiteCategory::Education => "Educational Institutions",
            SiteCategory::Healthcare => "Healthcare Services",
            SiteCategory::Legal => "Legal and Compliance",
            SiteCategory::Gaming => "Gaming and Entertainment",
            SiteCategory::Entertainment => "Streaming and Entertainment",
            SiteCategory::Corporate => "Corporate Websites",
            SiteCategory::Search => "Search Engines",
            SiteCategory::Regional => "Regional Websites",
            SiteCategory::General => "General Websites",
        }
    }

    /// Sicherheitsstufe der Kategorie (0-5, 5 = höchste Sicherheit)
    pub fn security_level(&self) -> u8 {
        match self {
            SiteCategory::Banking => 5,
            SiteCategory::Government => 5,
            SiteCategory::Healthcare => 4,
            SiteCategory::Legal => 4,
            SiteCategory::Education => 3,
            SiteCategory::Corporate => 3,
            SiteCategory::Developer => 2,
            SiteCategory::ECommerce => 2,
            SiteCategory::News => 2,
            SiteCategory::Search => 2,
            SiteCategory::SocialMedia => 1,
            SiteCategory::Gaming => 1,
            SiteCategory::Entertainment => 1,
            SiteCategory::Regional => 2,
            SiteCategory::General => 1,
        }
    }

    /// Empfohlene Timeout-Werte für diese Kategorie
    pub fn recommended_timeout(&self) -> u64 {
        match self {
            SiteCategory::Banking => 60,
            SiteCategory::Government => 60,
            SiteCategory::Healthcare => 50,
            SiteCategory::Legal => 50,
            SiteCategory::Education => 40,
            SiteCategory::Corporate => 40,
            SiteCategory::Developer => 30,
            SiteCategory::ECommerce => 35,
            SiteCategory::News => 30,
            SiteCategory::Search => 25,
            SiteCategory::SocialMedia => 25,
            SiteCategory::Gaming => 30,
            SiteCategory::Entertainment => 30,
            SiteCategory::Regional => 35,
            SiteCategory::General => 30,
        }
    }

    /// Maximale Anzahl von Redirects für diese Kategorie
    pub fn max_redirects(&self) -> usize {
        match self {
            SiteCategory::Banking => 2,
            SiteCategory::Government => 2,
            SiteCategory::Healthcare => 3,
            SiteCategory::Legal => 3,
            SiteCategory::Education => 5,
            SiteCategory::Corporate => 5,
            SiteCategory::Developer => 5,
            SiteCategory::ECommerce => 8,
            SiteCategory::News => 6,
            SiteCategory::Search => 8,
            SiteCategory::SocialMedia => 10,
            SiteCategory::Gaming => 6,
            SiteCategory::Entertainment => 8,
            SiteCategory::Regional => 6,
            SiteCategory::General => 5,
        }
    }

    /// Ob diese Kategorie strenge Sicherheitsmaßnahmen erfordert
    pub fn requires_strict_security(&self) -> bool {
        self.security_level() >= 4
    }

    /// Ob diese Kategorie CORS-Probleme haben könnte
    pub fn likely_has_cors_issues(&self) -> bool {
        matches!(
            self,
            SiteCategory::Banking
                | SiteCategory::Government
                | SiteCategory::SocialMedia
                | SiteCategory::Developer
        )
    }
} 
