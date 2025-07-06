// 🔐 Password Generator - Secure password generation with strength analysis

use anyhow::Result;
use chrono::Utc;
use rand::Rng;
use crate::security::types::{GeneratedPassword, PasswordStrength};

pub struct PasswordGenerator {
    last_generated: Option<GeneratedPassword>,
    generation_history: Vec<GeneratedPassword>,
    default_length: usize,
    include_symbols: bool,
    include_numbers: bool,
    include_uppercase: bool,
    include_lowercase: bool,
    exclude_ambiguous: bool,
}

impl PasswordGenerator {
    pub fn new() -> Self {
        println!("🔐 Initializing Password Generator...");
        
        Self {
            last_generated: None,
            generation_history: Vec::new(),
            default_length: 12,
            include_symbols: true,
            include_numbers: true,
            include_uppercase: true,
            include_lowercase: true,
            exclude_ambiguous: true,
        }
    }

    pub fn generate_password(&mut self, length: usize, include_symbols: bool) -> GeneratedPassword {
        println!("🔐 Generating password with {} characters...", length);
        
        let mut charset = String::new();
        
        // Add character sets based on settings
        if self.include_lowercase {
            if self.exclude_ambiguous {
                charset.push_str("abcdefghijkmnopqrstuvwxyz"); // Exclude l, o
            } else {
                charset.push_str("abcdefghijklmnopqrstuvwxyz");
            }
        }
        
        if self.include_uppercase {
            if self.exclude_ambiguous {
                charset.push_str("ABCDEFGHJKLMNPQRSTUVWXYZ"); // Exclude I, O
            } else {
                charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
            }
        }
        
        if self.include_numbers {
            if self.exclude_ambiguous {
                charset.push_str("23456789"); // Exclude 0, 1
            } else {
                charset.push_str("0123456789");
            }
        }
        
        if include_symbols && self.include_symbols {
            if self.exclude_ambiguous {
                charset.push_str("!@#$%^&*+-=?"); // Exclude similar looking symbols
            } else {
                charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
            }
        }

        if charset.is_empty() {
            charset = "abcdefghijklmnopqrstuvwxyz".to_string(); // Fallback
        }

        let mut rng = rand::thread_rng();
        let password: String = (0..length)
            .map(|_| {
                let idx = rng.gen_range(0..charset.len());
                charset.chars().nth(idx).unwrap()
            })
            .collect();

        let strength = self.calculate_strength(&password);
        
        let generated_password = GeneratedPassword {
            password: password.clone(),
            strength,
            length,
            generated_at: Utc::now(),
        };

        self.last_generated = Some(generated_password.clone());
        self.generation_history.push(generated_password.clone());
        
        // Keep only last 50 passwords in history
        if self.generation_history.len() > 50 {
            self.generation_history.remove(0);
        }

        println!("🔐 Password generated: {} characters, {:?} strength", length, strength);
        generated_password
    }

    pub fn generate_passphrase(&mut self, word_count: usize) -> GeneratedPassword {
        println!("🔐 Generating passphrase with {} words...", word_count);
        
        // Common words for passphrase generation
        let words = vec![
            "apple", "banana", "cherry", "dragon", "elephant", "falcon", "guitar", "harbor",
            "island", "jungle", "kitten", "lemon", "mountain", "notebook", "ocean", "piano",
            "quartz", "rabbit", "sunset", "tiger", "umbrella", "violin", "wizard", "xylophone",
            "yellow", "zebra", "bridge", "castle", "forest", "garden", "hammer", "knight",
            "ladder", "market", "nature", "orange", "palace", "rescue", "silver", "temple",
            "unique", "valley", "winter", "wonder", "bright", "clever", "gentle", "honest",
            "simple", "strong", "brave", "quick", "smart", "happy", "lucky", "magic",
            "cyber", "digital", "secure", "random", "system", "network", "server", "client",
            "database", "protocol", "encrypted", "firewall", "gateway", "router", "switch",
            "terminal", "command", "execute", "process", "thread", "memory", "storage",
        ];

        let mut rng = rand::thread_rng();
        let mut passphrase = Vec::new();

        for _ in 0..word_count {
            let word = words[rng.gen_range(0..words.len())];
            // Capitalize first letter randomly
            let formatted_word = if rng.gen_bool(0.5) {
                word.to_string()
            } else {
                format!("{}{}", word.chars().next().unwrap().to_uppercase(), &word[1..])
            };
            passphrase.push(formatted_word);
        }

        // Add random numbers and symbols
        let separator = if rng.gen_bool(0.5) { "-" } else { "_" };
        let mut final_passphrase = passphrase.join(separator);
        
        // Add random number at the end
        if rng.gen_bool(0.7) {
            final_passphrase.push_str(&format!("{}{}", separator, rng.gen_range(10..999)));
        }

        let strength = self.calculate_strength(&final_passphrase);
        
        let generated_password = GeneratedPassword {
            password: final_passphrase,
            strength,
            length: final_passphrase.len(),
            generated_at: Utc::now(),
        };

        self.last_generated = Some(generated_password.clone());
        self.generation_history.push(generated_password.clone());

        println!("🔐 Passphrase generated: {} words, {:?} strength", word_count, strength);
        generated_password
    }

    pub fn generate_pin(&mut self, length: usize) -> GeneratedPassword {
        println!("🔐 Generating PIN with {} digits...", length);
        
        let mut rng = rand::thread_rng();
        let pin: String = (0..length)
            .map(|_| rng.gen_range(0..10).to_string())
            .collect();

        let strength = PasswordStrength::Weak; // PINs are inherently weak
        
        let generated_password = GeneratedPassword {
            password: pin,
            strength,
            length,
            generated_at: Utc::now(),
        };

        self.last_generated = Some(generated_password.clone());
        self.generation_history.push(generated_password.clone());

        println!("🔐 PIN generated: {} digits", length);
        generated_password
    }

    pub fn calculate_strength(&self, password: &str) -> PasswordStrength {
        let mut score = 0;
        let length = password.len();

        // Length scoring
        if length >= 8 { score += 10; }
        if length >= 12 { score += 10; }
        if length >= 16 { score += 10; }
        if length >= 20 { score += 10; }

        // Character diversity scoring
        let has_lowercase = password.chars().any(|c| c.is_lowercase());
        let has_uppercase = password.chars().any(|c| c.is_uppercase());
        let has_numbers = password.chars().any(|c| c.is_numeric());
        let has_symbols = password.chars().any(|c| !c.is_alphanumeric());

        if has_lowercase { score += 10; }
        if has_uppercase { score += 10; }
        if has_numbers { score += 10; }
        if has_symbols { score += 15; }

        // Bonus for having all character types
        if has_lowercase && has_uppercase && has_numbers && has_symbols {
            score += 10;
        }

        // Penalty for common patterns
        if password.contains("123") || password.contains("abc") || password.contains("qwe") {
            score -= 10;
        }

        // Penalty for repeated characters
        let mut char_counts = std::collections::HashMap::new();
        for c in password.chars() {
            *char_counts.entry(c).or_insert(0) += 1;
        }
        
        let max_repeat = char_counts.values().max().unwrap_or(&0);
        if *max_repeat > 2 {
            score -= (*max_repeat as i32 - 2) * 5;
        }

        // Convert score to strength
        match score {
            0..=30 => PasswordStrength::Weak,
            31..=60 => PasswordStrength::Medium,
            61..=80 => PasswordStrength::Strong,
            81.. => PasswordStrength::VeryStrong,
            _ => PasswordStrength::Weak,
        }
    }

    pub fn check_password_strength(&self, password: &str) -> (PasswordStrength, Vec<String>) {
        let strength = self.calculate_strength(password);
        let mut recommendations = Vec::new();

        if password.len() < 8 {
            recommendations.push("• Verwende mindestens 8 Zeichen".to_string());
        }

        if !password.chars().any(|c| c.is_lowercase()) {
            recommendations.push("• Füge Kleinbuchstaben hinzu".to_string());
        }

        if !password.chars().any(|c| c.is_uppercase()) {
            recommendations.push("• Füge Großbuchstaben hinzu".to_string());
        }

        if !password.chars().any(|c| c.is_numeric()) {
            recommendations.push("• Füge Zahlen hinzu".to_string());
        }

        if !password.chars().any(|c| !c.is_alphanumeric()) {
            recommendations.push("• Füge Symbole hinzu (!@#$%^&*)".to_string());
        }

        if password.contains("123") || password.contains("abc") || password.contains("qwe") {
            recommendations.push("• Vermeide häufige Muster".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("🎉 Passwort ist stark!".to_string());
        }

        (strength, recommendations)
    }

    pub fn set_generation_options(&mut self, options: PasswordGenerationOptions) {
        self.default_length = options.default_length;
        self.include_symbols = options.include_symbols;
        self.include_numbers = options.include_numbers;
        self.include_uppercase = options.include_uppercase;
        self.include_lowercase = options.include_lowercase;
        self.exclude_ambiguous = options.exclude_ambiguous;
        
        println!("🔐 Password generation options updated");
    }

    pub fn get_last_generated(&self) -> Option<&GeneratedPassword> {
        self.last_generated.as_ref()
    }

    pub fn get_generation_history(&self) -> &Vec<GeneratedPassword> {
        &self.generation_history
    }

    pub fn clear_history(&mut self) {
        self.generation_history.clear();
        println!("🧹 Password generation history cleared");
    }

    pub fn get_generator_display(&self) -> Vec<String> {
        let mut display = vec![
            "🔐 PASSWORD GENERATOR".to_string(),
            "====================".to_string(),
            "".to_string(),
            "⚙️ AKTUELLE EINSTELLUNGEN:".to_string(),
            format!("• Standard-Länge: {}", self.default_length),
            format!("• Symbole: {}", if self.include_symbols { "✅" } else { "❌" }),
            format!("• Zahlen: {}", if self.include_numbers { "✅" } else { "❌" }),
            format!("• Großbuchstaben: {}", if self.include_uppercase { "✅" } else { "❌" }),
            format!("• Kleinbuchstaben: {}", if self.include_lowercase { "✅" } else { "❌" }),
            format!("• Mehrdeutige ausschließen: {}", if self.exclude_ambiguous { "✅" } else { "❌" }),
            "".to_string(),
        ];

        if let Some(last) = &self.last_generated {
            display.extend(vec![
                "🔐 LETZTES GENERIERTES PASSWORT:".to_string(),
                format!("• Länge: {} Zeichen", last.length),
                format!("• Stärke: {} {}", last.strength.emoji(), last.strength.description()),
                format!("• Generiert: {}", last.generated_at.format("%Y-%m-%d %H:%M:%S")),
                "".to_string(),
            ]);
        }

        display.extend(vec![
            format!("📊 GENERIERTE PASSWÖRTER: {}", self.generation_history.len()),
            "".to_string(),
            "💡 KOMMANDOS:".to_string(),
            "• 'password generate' → Neues Passwort (12 Zeichen)".to_string(),
            "• 'password strong' → Starkes Passwort (16 Zeichen)".to_string(),
            "• 'password simple' → Einfaches Passwort (8 Zeichen)".to_string(),
            "• 'password pin' → PIN generieren".to_string(),
            "• 'password phrase' → Passphrase generieren".to_string(),
        ]);

        display
    }

    pub fn get_strength_statistics(&self) -> Vec<String> {
        let mut stats = vec![
            "📊 PASSWORT-STÄRKE STATISTIKEN".to_string(),
            "===============================".to_string(),
            "".to_string(),
        ];

        if self.generation_history.is_empty() {
            stats.push("📝 Noch keine Passwörter generiert".to_string());
            return stats;
        }

        let weak_count = self.generation_history.iter().filter(|p| matches!(p.strength, PasswordStrength::Weak)).count();
        let medium_count = self.generation_history.iter().filter(|p| matches!(p.strength, PasswordStrength::Medium)).count();
        let strong_count = self.generation_history.iter().filter(|p| matches!(p.strength, PasswordStrength::Strong)).count();
        let very_strong_count = self.generation_history.iter().filter(|p| matches!(p.strength, PasswordStrength::VeryStrong)).count();

        let total = self.generation_history.len();

        stats.extend(vec![
            format!("📈 Gesamt generierte Passwörter: {}", total),
            "".to_string(),
            "🎯 STÄRKE-VERTEILUNG:".to_string(),
            format!("• 🔴 Schwach: {} ({:.1}%)", weak_count, (weak_count as f32 / total as f32) * 100.0),
            format!("• 🟡 Mittel: {} ({:.1}%)", medium_count, (medium_count as f32 / total as f32) * 100.0),
            format!("• 🟢 Stark: {} ({:.1}%)", strong_count, (strong_count as f32 / total as f32) * 100.0),
            format!("• 💪 Sehr stark: {} ({:.1}%)", very_strong_count, (very_strong_count as f32 / total as f32) * 100.0),
            "".to_string(),
        ]);

        // Average length
        let avg_length = self.generation_history.iter().map(|p| p.length).sum::<usize>() as f32 / total as f32;
        stats.push(format!("📏 Durchschnittliche Länge: {:.1} Zeichen", avg_length));

        stats
    }

    pub fn export_history(&self) -> Result<String> {
        let json_data = serde_json::to_string_pretty(&self.generation_history)?;
        println!("📤 Exported {} password generation records", self.generation_history.len());
        Ok(json_data)
    }
}

#[derive(Debug, Clone)]
pub struct PasswordGenerationOptions {
    pub default_length: usize,
    pub include_symbols: bool,
    pub include_numbers: bool,
    pub include_uppercase: bool,
    pub include_lowercase: bool,
    pub exclude_ambiguous: bool,
}

impl Default for PasswordGenerationOptions {
    fn default() -> Self {
        Self {
            default_length: 12,
            include_symbols: true,
            include_numbers: true,
            include_uppercase: true,
            include_lowercase: true,
            exclude_ambiguous: true,
        }
    }
}

impl Default for PasswordGenerator {
    fn default() -> Self {
        Self::new()
    }
}

// 🧪 TESTS
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_generator_creation() {
        let generator = PasswordGenerator::new();
        assert_eq!(generator.default_length, 12);
        assert!(generator.include_symbols);
        assert!(generator.include_numbers);
    }

    #[test]
    fn test_password_generation() {
        let mut generator = PasswordGenerator::new();
        let password = generator.generate_password(10, true);
        assert_eq!(password.length, 10);
        assert_eq!(password.password.len(), 10);
    }

    #[test]
    fn test_passphrase_generation() {
        let mut generator = PasswordGenerator::new();
        let passphrase = generator.generate_passphrase(4);
        assert!(!passphrase.password.is_empty());
        assert!(passphrase.password.contains("-") || passphrase.password.contains("_"));
    }

    #[test]
    fn test_pin_generation() {
        let mut generator = PasswordGenerator::new();
        let pin = generator.generate_pin(4);
        assert_eq!(pin.length, 4);
        assert!(pin.password.chars().all(|c| c.is_numeric()));
        assert!(matches!(pin.strength, PasswordStrength::Weak));
    }

    #[test]
    fn test_strength_calculation() {
        let generator = PasswordGenerator::new();
        
        let weak = generator.calculate_strength("123");
        assert!(matches!(weak, PasswordStrength::Weak));
        
        let strong = generator.calculate_strength("MyStr0ng!P@ssw0rd");
        assert!(matches!(strong, PasswordStrength::Strong | PasswordStrength::VeryStrong));
    }

    #[test]
    fn test_password_strength_check() {
        let generator = PasswordGenerator::new();
        let (strength, recommendations) = generator.check_password_strength("password");
        
        assert!(matches!(strength, PasswordStrength::Weak));
        assert!(!recommendations.is_empty());
    }

    #[test]
    fn test_generation_history() {
        let mut generator = PasswordGenerator::new();
        
        assert_eq!(generator.generation_history.len(), 0);
        
        generator.generate_password(10, true);
        assert_eq!(generator.generation_history.len(), 1);
        
        generator.generate_pin(4);
        assert_eq!(generator.generation_history.len(), 2);
        
        generator.clear_history();
        assert_eq!(generator.generation_history.len(), 0);
    }
}