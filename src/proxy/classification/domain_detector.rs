// 🔍 Domain Detector
// Erkennt und klassifiziert Domains für Strategy-Auswahl

use super::site_categories::SiteCategory;

pub struct DomainDetector;

impl DomainDetector {
    pub fn new() -> Self {
        Self
    }

    /// Klassifiziere eine Website basierend auf ihrer Domain
    pub fn classify_site(&self, url: &str) -> SiteCategory {
        let domain = self.extract_domain(url);

        // Banking Sites
        if self.is_banking_domain(&domain) {
            return SiteCategory::Banking;
        }

        // Government Sites
        if self.is_government_domain(&domain) {
            return SiteCategory::Government;
        }

        // Developer Sites
        if self.is_developer_domain(&domain) {
            return SiteCategory::Developer;
        }

        // Social Media Sites
        if self.is_social_media_domain(&domain) {
            return SiteCategory::SocialMedia;
        }

        // E-Commerce Sites
        if self.is_ecommerce_domain(&domain) {
            return SiteCategory::ECommerce;
        }

        // News Sites
        if self.is_news_domain(&domain) {
            return SiteCategory::News;
        }

        // Education Sites
        if self.is_education_domain(&domain) {
            return SiteCategory::Education;
        }

        // Healthcare Sites
        if self.is_healthcare_domain(&domain) {
            return SiteCategory::Healthcare;
        }

        // Gaming Sites
        if self.is_gaming_domain(&domain) {
            return SiteCategory::Gaming;
        }

        // Entertainment Sites
        if self.is_entertainment_domain(&domain) {
            return SiteCategory::Entertainment;
        }

        // Search Engines
        if self.is_search_domain(&domain) {
            return SiteCategory::Search;
        }

        // Regional Sites
        if self.is_regional_domain(&domain) {
            return SiteCategory::Regional;
        }

        // Default
        SiteCategory::General
    }

    /// Extrahiere Domain aus URL
    pub fn extract_domain(&self, url: &str) -> String {
        if let Ok(parsed) = url::Url::parse(url) {
            parsed.host_str().unwrap_or("unknown").to_lowercase()
        } else {
            url.to_lowercase()
        }
    }

    /// Prüfe ob es eine Banking-Domain ist
    pub fn is_banking_domain(&self, domain: &str) -> bool {
        let banking_patterns = [
            "bank", "banking", "sparkasse", "volksbank", "commerzbank", "deutsche-bank",
            "postbank", "dkb", "ing", "comdirect", "consorsbank", "dab", "onvista",
            "paypal", "stripe", "klarna", "sofort", "giropay", "paydirekt",
            "chase", "wells", "bofa", "citi", "jpmorgan", "hsbc", "barclays",
            "santander", "bnp", "credit", "american-express", "mastercard", "visa",
            "revolut", "n26", "monzo", "starling", "wise", "transferwise"
        ];
        
        banking_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Government-Domain ist
    pub fn is_government_domain(&self, domain: &str) -> bool {
        let government_patterns = [
            ".gov", ".mil", ".edu", "government", "bundesamt", "bundestag",
            "bundesregierung", "bund.de", "bundesrat", "bundeskanzlerin",
            "destatis", "arbeitsagentur", "rentenversicherung", "krankenkasse",
            "finanzamt", "zoll", "polizei", "feuerwehr", "stadt", "gemeinde",
            "landkreis", "regierung", "ministerium", "behörde", "amt.",
            "whitehouse", "congress", "senate", "fbi", "cia", "nsa", "dhs",
            "irs", "uscis", "usps", "medicare", "medicaid", "va.gov"
        ];
        
        government_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Developer-Domain ist
    pub fn is_developer_domain(&self, domain: &str) -> bool {
        let developer_patterns = [
            "github", "gitlab", "stackoverflow", "stackexchange", "codepen",
            "jsfiddle", "replit", "codesandbox", "glitch", "heroku", "netlify",
            "vercel", "firebase", "aws", "azure", "gcp", "docker", "kubernetes",
            "npm", "yarn", "pypi", "crates.io", "packagist", "maven", "gradle",
            "jenkins", "travis", "circleci", "appveyor", "bitbucket", "sourcetree",
            "jetbrains", "vscode", "atom", "sublime", "vim", "emacs", "dev.to",
            "hackernews", "reddit.com/r/programming", "medium.com/@", "hashnode"
        ];
        
        developer_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Social Media-Domain ist
    pub fn is_social_media_domain(&self, domain: &str) -> bool {
        let social_patterns = [
            "facebook", "instagram", "twitter", "x.com", "linkedin", "xing",
            "tiktok", "snapchat", "pinterest", "reddit", "discord", "telegram",
            "whatsapp", "signal", "clubhouse", "twitch", "youtube", "vimeo",
            "dailymotion", "vk.com", "ok.ru", "weibo", "wechat", "line",
            "kakao", "qq.com", "douyin", "sina", "baidu", "tumblr", "mastodon"
        ];
        
        social_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine E-Commerce-Domain ist
    pub fn is_ecommerce_domain(&self, domain: &str) -> bool {
        let ecommerce_patterns = [
            "amazon", "ebay", "etsy", "shopify", "woocommerce", "magento",
            "otto", "zalando", "lidl", "aldi", "rewe", "kaufland", "dm",
            "rossmann", "mediamarkt", "saturn", "conrad", "alternate",
            "notebooksbilliger", "cyberport", "mindfactory", "digitec",
            "alibaba", "aliexpress", "wish", "banggood", "gearbest",
            "walmart", "target", "bestbuy", "newegg", "bhphotovideo",
            "shop", "store", "market", "buy", "cart", "checkout", "payment"
        ];
        
        ecommerce_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine News-Domain ist
    pub fn is_news_domain(&self, domain: &str) -> bool {
        let news_patterns = [
            "bbc", "cnn", "reuters", "ap", "bloomberg", "wsj", "nytimes",
            "guardian", "telegraph", "independent", "economist", "forbes",
            "spiegel", "bild", "zeit", "faz", "sueddeutsche", "welt",
            "focus", "stern", "ntv", "tagesschau", "zdf", "ard", "rtl",
            "sat1", "pro7", "vox", "kabel1", "news", "nachrichten",
            "breaking", "heute", "aktuell", "live", "radio", "tv", "media"
        ];
        
        news_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Education-Domain ist
    pub fn is_education_domain(&self, domain: &str) -> bool {
        let education_patterns = [
            ".edu", "university", "college", "school", "akademie", "hochschule",
            "universität", "tu-", "uni-", "fh-", "rwth", "lmu", "kit",
            "coursera", "udemy", "edx", "khanacademy", "codecademy", "pluralsight",
            "linkedin-learning", "skillshare", "udacity", "treehouse",
            "freecodecamp", "w3schools", "mozilla", "developer", "learn",
            "tutorial", "course", "education", "training", "academy"
        ];
        
        education_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Healthcare-Domain ist
    pub fn is_healthcare_domain(&self, domain: &str) -> bool {
        let healthcare_patterns = [
            "hospital", "klinik", "krankenhaus", "arzt", "doctor", "medical",
            "gesundheit", "health", "medizin", "pharmacy", "apotheke",
            "medicare", "medicaid", "who", "cdc", "nih", "fda", "rki",
            "charité", "uniklinik", "helios", "asklepios", "sana", "vivantes"
        ];
        
        healthcare_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Gaming-Domain ist
    pub fn is_gaming_domain(&self, domain: &str) -> bool {
        let gaming_patterns = [
            "steam", "origin", "uplay", "epic", "gog", "battlenet", "blizzard",
            "riot", "valve", "ea", "ubisoft", "activision", "bethesda",
            "nintendo", "playstation", "xbox", "microsoft", "sony", "gaming",
            "game", "esports", "twitch", "ign", "gamespot", "polygon",
            "kotaku", "destructoid", "pcgamer", "eurogamer", "4players"
        ];
        
        gaming_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Entertainment-Domain ist
    pub fn is_entertainment_domain(&self, domain: &str) -> bool {
        let entertainment_patterns = [
            "netflix", "disney", "hulu", "amazon-prime", "hbo", "paramount",
            "peacock", "apple-tv", "youtube", "vimeo", "dailymotion", "twitch",
            "spotify", "apple-music", "deezer", "tidal", "soundcloud",
            "imdb", "rotten-tomatoes", "metacritic", "entertainment", "movie",
            "film", "music", "streaming", "video", "audio", "podcast"
        ];
        
        entertainment_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Search Engine-Domain ist
    pub fn is_search_domain(&self, domain: &str) -> bool {
        let search_patterns = [
            "google", "bing", "yahoo", "duckduckgo", "startpage", "searx",
            "yandex", "baidu", "ask", "aol", "ecosia", "qwant", "brave-search"
        ];
        
        search_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine Regional-Domain ist
    pub fn is_regional_domain(&self, domain: &str) -> bool {
        self.is_russian_domain(domain) || self.is_chinese_domain(domain) || self.is_european_domain(domain)
    }

    /// Prüfe ob es eine russische Domain ist
    pub fn is_russian_domain(&self, domain: &str) -> bool {
        let russian_patterns = [
            ".ru", ".рф", "yandex", "mail.ru", "vk.com", "ok.ru", "rambler",
            "livejournal", "lj", "gazeta", "rbc", "rt.com", "sputnik",
            "tass", "interfax", "ria", "kommersant", "vedomosti"
        ];
        
        russian_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine chinesische Domain ist
    pub fn is_chinese_domain(&self, domain: &str) -> bool {
        let chinese_patterns = [
            ".cn", ".中国", "baidu", "qq.com", "weibo", "wechat", "taobao",
            "tmall", "jd.com", "163.com", "sina", "sohu", "youku", "tudou",
            "bilibili", "zhihu", "douban", "xiaomi", "huawei", "oppo", "vivo"
        ];
        
        chinese_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob es eine europäische Domain ist
    pub fn is_european_domain(&self, domain: &str) -> bool {
        let european_patterns = [
            ".de", ".fr", ".it", ".es", ".nl", ".be", ".at", ".ch", ".pl",
            ".cz", ".sk", ".hu", ".ro", ".bg", ".hr", ".si", ".lt", ".lv",
            ".ee", ".fi", ".se", ".no", ".dk", ".ie", ".pt", ".gr", ".cy",
            ".mt", ".lu", ".eu"
        ];
        
        european_patterns.iter().any(|pattern| domain.contains(pattern))
    }

    /// Prüfe ob die Domain CSP-Restriktionen hat
    pub fn has_csp_restrictions(&self, domain: &str) -> bool {
        let csp_patterns = [
            "google", "youtube", "facebook", "instagram", "twitter", "linkedin",
            "github", "stackoverflow", "amazon", "microsoft", "apple", "adobe",
            "salesforce", "atlassian", "slack", "zoom", "teams", "dropbox",
            "banking", "government", "financial", "secure", "login", "auth"
        ];
        
        csp_patterns.iter().any(|pattern| domain.contains(pattern))
    }
}

impl Default for DomainDetector {
    fn default() -> Self {
        Self::new()
    }
} 
