// 🔍 PLUGIN VALIDATOR
// Validierung von Plugin-Manifesten und -Berechtigungen
// Copyright © 2024 Ora Browser Team

use crate::plugin::types::{PluginManifest, ALLOWED_PERMISSIONS, SUPPORTED_API_VERSIONS};
use crate::error::{OraBrowserError, OraBrowserResult};

/// Plugin-Validierungsresultate
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    
    pub fn add_error(&mut self, message: String) {
        self.errors.push(message);
        self.valid = false;
    }
    
    pub fn add_warning(&mut self, message: String) {
        self.warnings.push(message);
    }
    
    pub fn is_valid(&self) -> bool {
        self.valid
    }
}

/// Plugin-Validierungsregeln
#[derive(Debug)]
pub struct PluginValidator {
    strict_validation: bool,
}

impl PluginValidator {
    pub fn new(strict: bool) -> Self {
        Self {
            strict_validation: strict,
        }
    }
    
    /// Validiert ein Plugin-Manifest vollständig
    pub fn validate_manifest(&self, manifest: &PluginManifest) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        // Validiere API-Version
        self.validate_api_version(manifest, &mut result);
        
        // Validiere Berechtigungen
        self.validate_permissions(manifest, &mut result);
        
        // Validiere Pflichtfelder
        self.validate_required_fields(manifest, &mut result);
        
        // Validiere Skript-Dateien
        self.validate_scripts(manifest, &mut result);
        
        // Validiere Versionsnummer
        self.validate_version(manifest, &mut result);
        
        result
    }
    
    /// Validiert nur die Plugin-Berechtigungen (Legacy-Methode)
    pub fn validate_plugin_permissions(&self, manifest: &PluginManifest) -> OraBrowserResult<()> {
        let result = self.validate_manifest(manifest);
        
        if result.is_valid() {
            Ok(())
        } else {
            Err(OraBrowserError::plugin_error("validation", &result.errors.join("; ")))
        }
    }
    
    fn validate_api_version(&self, manifest: &PluginManifest, result: &mut ValidationResult) {
        if !SUPPORTED_API_VERSIONS.contains(&manifest.api_version.as_str()) {
            result.add_error(format!(
                "Unsupported API version: {}. Supported versions: {:?}",
                manifest.api_version, SUPPORTED_API_VERSIONS
            ));
        }
    }
    
    fn validate_permissions(&self, manifest: &PluginManifest, result: &mut ValidationResult) {
        for permission in &manifest.permissions {
            if !ALLOWED_PERMISSIONS.contains(&permission.as_str()) {
                if self.strict_validation {
                    result.add_error(format!("Unknown permission: {}", permission));
                } else {
                    result.add_warning(format!("Unknown permission: {}", permission));
                }
            }
        }
        
        // Prüfe auf gefährliche Berechtigungen
        let dangerous_permissions = ["network", "webRequest", "proxy"];
        for permission in &manifest.permissions {
            if dangerous_permissions.contains(&permission.as_str()) {
                result.add_warning(format!("Dangerous permission requested: {}", permission));
            }
        }
    }
    
    fn validate_required_fields(&self, manifest: &PluginManifest, result: &mut ValidationResult) {
        if manifest.name.trim().is_empty() {
            result.add_error("Plugin name cannot be empty".to_string());
        }
        
        if manifest.version.trim().is_empty() {
            result.add_error("Plugin version cannot be empty".to_string());
        }
        
        if manifest.description.trim().is_empty() {
            result.add_warning("Plugin description is empty".to_string());
        }
        
        if manifest.author.trim().is_empty() {
            result.add_warning("Plugin author is empty".to_string());
        }
        
        if manifest.main_script.trim().is_empty() {
            result.add_error("Main script path cannot be empty".to_string());
        }
    }
    
    fn validate_scripts(&self, manifest: &PluginManifest, result: &mut ValidationResult) {
        // Validiere Hauptskript-Erweiterung
        if !manifest.main_script.ends_with(".js") {
            result.add_warning("Main script should have .js extension".to_string());
        }
        
        // Validiere Background-Skripte
        if let Some(background) = &manifest.background {
            for script in &background.scripts {
                if !script.ends_with(".js") {
                    result.add_warning(format!("Background script should have .js extension: {}", script));
                }
            }
        }
        
        // Validiere Content-Skripte
        if let Some(content_scripts) = &manifest.content_scripts {
            for content_script in content_scripts {
                for script in &content_script.js {
                    if !script.ends_with(".js") {
                        result.add_warning(format!("Content script should have .js extension: {}", script));
                    }
                }
                
                if content_script.matches.is_empty() {
                    result.add_error("Content script must have at least one match pattern".to_string());
                }
            }
        }
    }
    
    fn validate_version(&self, manifest: &PluginManifest, result: &mut ValidationResult) {
        // Einfache Versionsnummer-Validierung (major.minor.patch)
        let version_parts: Vec<&str> = manifest.version.split('.').collect();
        
        if version_parts.len() != 3 {
            result.add_warning("Version should follow semantic versioning (major.minor.patch)".to_string());
        } else {
            for part in version_parts {
                if part.parse::<u32>().is_err() {
                    result.add_warning("Version parts should be numeric".to_string());
                    break;
                }
            }
        }
    }
    
    /// Validiert Plugin-Abhängigkeiten
    pub fn validate_dependencies(&self, manifest: &PluginManifest) -> ValidationResult {
        let mut result = ValidationResult::new();
        
        if let Some(dependencies) = &manifest.dependencies {
            for (dep_name, dep_version) in dependencies {
                if dep_name.trim().is_empty() {
                    result.add_error("Dependency name cannot be empty".to_string());
                }
                
                if dep_version.trim().is_empty() {
                    result.add_error(format!("Dependency version cannot be empty for: {}", dep_name));
                }
            }
        }
        
        result
    }
}

impl Default for PluginValidator {
    fn default() -> Self {
        Self::new(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::types::PluginManifest;
    
    fn create_valid_manifest() -> PluginManifest {
        PluginManifest {
            id: Some("test-plugin".to_string()),
            name: "Test Plugin".to_string(),
            version: "1.0.0".to_string(),
            description: "Test description".to_string(),
            author: "Test Author".to_string(),
            homepage: None,
            main_script: "main.js".to_string(),
            permissions: vec!["network".to_string(), "storage".to_string()],
            api_version: "1.0".to_string(),
            enabled: true,
            background: None,
            content_scripts: None,
            web_accessible_resources: None,
            browser_action: None,
            options_page: None,
            manifest_version: None,
            minimum_ora_version: None,
            dependencies: None,
            settings: None,
            update_url: None,
        }
    }
    
    #[test]
    fn test_valid_manifest() {
        let validator = PluginValidator::new(true);
        let manifest = create_valid_manifest();
        let result = validator.validate_manifest(&manifest);
        
        assert!(result.is_valid());
        assert!(result.errors.is_empty());
    }
    
    #[test]
    fn test_invalid_api_version() {
        let validator = PluginValidator::new(true);
        let mut manifest = create_valid_manifest();
        manifest.api_version = "2.0".to_string();
        
        let result = validator.validate_manifest(&manifest);
        
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("Unsupported API version")));
    }
    
    #[test]
    fn test_invalid_permissions() {
        let validator = PluginValidator::new(true);
        let mut manifest = create_valid_manifest();
        manifest.permissions = vec!["invalid_permission".to_string()];
        
        let result = validator.validate_manifest(&manifest);
        
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("Unknown permission")));
    }
    
    #[test]
    fn test_empty_required_fields() {
        let validator = PluginValidator::new(true);
        let mut manifest = create_valid_manifest();
        manifest.name = "".to_string();
        manifest.main_script = "".to_string();
        
        let result = validator.validate_manifest(&manifest);
        
        assert!(!result.is_valid());
        assert!(result.errors.iter().any(|e| e.contains("name cannot be empty")));
        assert!(result.errors.iter().any(|e| e.contains("script path cannot be empty")));
    }
    
    #[test]
    fn test_version_validation() {
        let validator = PluginValidator::new(true);
        let mut manifest = create_valid_manifest();
        manifest.version = "1.0".to_string(); // Nicht semantic versioning
        
        let result = validator.validate_manifest(&manifest);
        
        assert!(result.is_valid()); // Sollte gültig sein, aber mit Warnung
        assert!(result.warnings.iter().any(|w| w.contains("semantic versioning")));
    }
    
    #[test]
    fn test_legacy_permission_validation() {
        let validator = PluginValidator::new(true);
        let manifest = create_valid_manifest();
        
        assert!(validator.validate_plugin_permissions(&manifest).is_ok());
        
        let mut invalid_manifest = manifest;
        invalid_manifest.permissions = vec!["invalid_permission".to_string()];
        
        assert!(validator.validate_plugin_permissions(&invalid_manifest).is_err());
    }
} 