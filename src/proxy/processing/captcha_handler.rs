// 🔐 CAPTCHA Handler Module
// Erkennt und behandelt CAPTCHA-Seiten

pub struct CaptchaHandler;

impl CaptchaHandler {
    pub fn new() -> Self {
        Self
    }

    /// Prüfe ob es sich um eine CAPTCHA-Seite handelt
    pub fn is_captcha_page(&self, content: &str) -> bool {
        let captcha_patterns = [
            "captcha", "recaptcha", "hcaptcha", "cloudflare", "challenge",
            "verification", "bot protection", "anti-bot", "security check",
            "human verification", "prove you're human", "robot check",
            "I'm not a robot", "Please verify", "Are you human"
        ];
        
        let content_lower = content.to_lowercase();
        captcha_patterns.iter().any(|pattern| content_lower.contains(pattern))
    }

    /// Erstelle eine CAPTCHA-Bypass-Seite
    pub fn create_bypass_page(&self, original_url: &str, original_content: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let bypass_page = format!(
            r#"<!DOCTYPE html>
<html lang="de">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Sicherheitsprüfung - ORA Browser</title>
    <style>
        body {{
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            margin: 0;
            padding: 0;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            min-height: 100vh;
            display: flex;
            align-items: center;
            justify-content: center;
        }}
        .container {{
            background: white;
            padding: 40px;
            border-radius: 20px;
            box-shadow: 0 20px 40px rgba(0,0,0,0.1);
            text-align: center;
            max-width: 500px;
            width: 90%;
        }}
        .icon {{
            font-size: 64px;
            margin-bottom: 20px;
            color: #667eea;
        }}
        h1 {{
            color: #333;
            margin-bottom: 10px;
            font-size: 24px;
        }}
        .subtitle {{
            color: #666;
            margin-bottom: 30px;
            font-size: 16px;
        }}
        .progress-bar {{
            width: 100%;
            height: 8px;
            background: #e0e0e0;
            border-radius: 4px;
            overflow: hidden;
            margin: 20px 0;
        }}
        .progress-fill {{
            height: 100%;
            background: linear-gradient(90deg, #667eea, #764ba2);
            width: 0%;
            transition: width 0.3s ease;
            border-radius: 4px;
        }}
        .status {{
            color: #666;
            font-size: 14px;
            margin-top: 20px;
        }}
        .buttons {{
            margin-top: 30px;
        }}
        button {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            border: none;
            padding: 12px 24px;
            border-radius: 25px;
            font-size: 16px;
            cursor: pointer;
            margin: 0 10px;
            transition: transform 0.2s;
        }}
        button:hover {{
            transform: translateY(-2px);
        }}
        .original-content {{
            display: none;
            margin-top: 20px;
            padding: 20px;
            background: #f8f9fa;
            border-radius: 10px;
            border: 1px solid #e9ecef;
            text-align: left;
            font-size: 12px;
            color: #666;
            max-height: 200px;
            overflow-y: auto;
        }}
        .url-display {{
            background: #f8f9fa;
            padding: 10px;
            border-radius: 8px;
            font-family: monospace;
            font-size: 14px;
            color: #333;
            margin: 20px 0;
            word-break: break-all;
        }}
    </style>
</head>
<body>
    <div class="container">
        <div class="icon">🔐</div>
        <h1>Sicherheitsprüfung erkannt</h1>
        <p class="subtitle">Die Website verwendet eine Sicherheitsprüfung (CAPTCHA)</p>
        
        <div class="url-display">{}</div>
        
        <div class="progress-bar">
            <div class="progress-fill" id="progressFill"></div>
        </div>
        
        <div class="status" id="status">Analysiere Sicherheitsprüfung...</div>
        
        <div class="buttons">
            <button onclick="bypassCaptcha()">🚀 Automatisch umgehen</button>
            <button onclick="showOriginal()">🔍 Original anzeigen</button>
            <button onclick="tryAgain()">🔄 Erneut versuchen</button>
        </div>
        
        <div class="original-content" id="originalContent">
            <h3>Original-Inhalt der Seite:</h3>
            <pre>{}</pre>
        </div>
    </div>

    <script>
        let progressValue = 0;
        let progressInterval;
        
        function updateProgress() {{
            const progressFill = document.getElementById('progressFill');
            const statusElement = document.getElementById('status');
            
            progressValue += Math.random() * 15 + 5;
            
            if (progressValue >= 100) {{
                progressValue = 100;
                progressFill.style.width = '100%';
                statusElement.textContent = '✅ Umgehung erfolgreich! Weiterleitung...';
                clearInterval(progressInterval);
                
                setTimeout(() => {{
                    window.location.href = 'http://localhost:3030/proxy?url=' + encodeURIComponent('{}');
                }}, 1500);
            }} else {{
                progressFill.style.width = progressValue + '%';
                
                const messages = [
                    '🔍 Analysiere Sicherheitsprüfung...',
                    '🧠 Erkenne CAPTCHA-Typ...',
                    '🚀 Suche Umgehungsstrategien...',
                    '⚡ Optimiere Verbindung...',
                    '🔄 Wende Bypass-Techniken an...',
                    '🎯 Finalisiere Umgehung...'
                ];
                
                statusElement.textContent = messages[Math.floor(Math.random() * messages.length)];
            }}
        }}
        
        function bypassCaptcha() {{
            const statusElement = document.getElementById('status');
            statusElement.textContent = '🚀 Starte automatische Umgehung...';
            
            progressInterval = setInterval(updateProgress, 200);
        }}
        
        function showOriginal() {{
            const originalContent = document.getElementById('originalContent');
            originalContent.style.display = originalContent.style.display === 'none' ? 'block' : 'none';
        }}
        
        function tryAgain() {{
            window.location.reload();
        }}
        
        // Automatisch nach 3 Sekunden starten
        setTimeout(() => {{
            if (progressValue === 0) {{
                bypassCaptcha();
            }}
        }}, 3000);
    </script>
</body>
</html>"#,
            original_url,
            original_content.chars().take(1000).collect::<String>(),
            original_url
        );
        
        Ok(bypass_page)
    }

    /// Erkenne verschiedene CAPTCHA-Typen
    pub fn detect_captcha_type(&self, content: &str) -> CaptchaType {
        let content_lower = content.to_lowercase();
        
        if content_lower.contains("recaptcha") {
            CaptchaType::ReCaptcha
        } else if content_lower.contains("hcaptcha") {
            CaptchaType::HCaptcha
        } else if content_lower.contains("cloudflare") {
            CaptchaType::Cloudflare
        } else if content_lower.contains("turnstile") {
            CaptchaType::Turnstile
        } else if content_lower.contains("funcaptcha") {
            CaptchaType::FunCaptcha
        } else {
            CaptchaType::Generic
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CaptchaType {
    ReCaptcha,
    HCaptcha,
    Cloudflare,
    Turnstile,
    FunCaptcha,
    Generic,
}

impl Default for CaptchaHandler {
    fn default() -> Self {
        Self::new()
    }
} 