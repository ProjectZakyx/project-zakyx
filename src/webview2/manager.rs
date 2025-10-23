// 🎯 WebView2 Manager für ZAKYX Browser
// Multi-Instance-Management und Orchestrierung

use anyhow::Result;
use std::collections::HashMap;
use windows::Win32::Foundation::HWND;
use crate::webview2::{
    config::WebView2Config,
    engine::WebView2Engine,
    performance::WebView2PerformanceMonitor,
    environment::{WebView2EnvironmentDetector, WebView2EnvironmentInfo},
};

/// WebView2-Manager für Multi-Instance-Verwaltung
#[derive(Debug)]
pub struct OptimizedWebView2Manager {
    /// WebView2-Instanzen
    instances: HashMap<String, WebView2Instance>,
    
    /// Aktive Instanz-ID
    active_instance: Option<String>,
    
    /// Globale Konfiguration
    global_config: WebView2Config,
    
    /// Environment-Detector
    environment_detector: WebView2EnvironmentDetector,
    
    /// Performance-Monitor
    performance_monitor: WebView2PerformanceMonitor,
    
    /// Manager-Statistiken
    statistics: ManagerStatistics,
    
    /// Initialisierungsstatus
    is_initialized: bool,
}

/// WebView2-Instanz
#[derive(Debug)]
pub struct WebView2Instance {
    /// Eindeutige ID
    pub id: String,
    
    /// WebView2-Engine
    pub engine: WebView2Engine,
    
    /// Instanz-spezifische Konfiguration
    pub config: WebView2Config,
    
    /// Parent-Window
    pub parent_window: HWND,
    
    /// Erstellungszeit
    pub created_at: std::time::SystemTime,
    
    /// Letzte Aktivität
    pub last_activity: std::time::SystemTime,
    
    /// Status
    pub status: InstanceStatus,
    
    /// Instanz-Metriken
    pub metrics: InstanceMetrics,
}

/// Instanz-Status
#[derive(Debug, Clone, PartialEq)]
pub enum InstanceStatus {
    /// Wird erstellt
    Creating,
    
    /// Aktiv und bereit
    Active,
    
    /// Pausiert
    Paused,
    
    /// Fehler aufgetreten
    Error(String),
    
    /// Wird zerstört
    Destroying,
    
    /// Zerstört
    Destroyed,
}

/// Instanz-Metriken
#[derive(Debug, Clone)]
pub struct InstanceMetrics {
    /// Anzahl Navigationen
    pub navigation_count: u64,
    
    /// Anzahl JavaScript-Ausführungen
    pub javascript_executions: u64,
    
    /// Gesamte Laufzeit
    pub total_uptime: std::time::Duration,
    
    /// Letzte Fehler
    pub last_errors: Vec<String>,
    
    /// Performance-Score (0-100)
    pub performance_score: u8,
}

/// Manager-Statistiken
#[derive(Debug, Clone)]
pub struct ManagerStatistics {
    /// Gesamtanzahl erstellter Instanzen
    pub total_instances_created: u64,
    
    /// Aktive Instanzen
    pub active_instances: u64,
    
    /// Fehlgeschlagene Instanzen
    pub failed_instances: u64,
    
    /// Durchschnittliche Instanz-Lebensdauer
    pub average_instance_lifetime: std::time::Duration,
    
    /// Gesamte Navigation-Anzahl
    pub total_navigations: u64,
    
    /// Gesamte JavaScript-Ausführungen
    pub total_javascript_executions: u64,
    
    /// Manager-Startzeit
    pub manager_start_time: std::time::SystemTime,
}

impl OptimizedWebView2Manager {
    /// Erstellt einen neuen WebView2-Manager
    pub fn new() -> Self {
        println!("🎯 Creating Optimized WebView2 Manager...");
        
        Self {
            instances: HashMap::new(),
            active_instance: None,
            global_config: WebView2Config::default(),
            environment_detector: WebView2EnvironmentDetector::new(),
            performance_monitor: WebView2PerformanceMonitor::new(),
            statistics: ManagerStatistics::new(),
            is_initialized: false,
        }
    }

    /// Erstellt einen Manager mit globaler Konfiguration
    pub fn with_global_config(config: WebView2Config) -> Self {
        println!("🎯 Creating WebView2 Manager with global config...");
        
        let mut manager = Self::new();
        manager.global_config = config;
        manager
    }

    /// Initialisiert den Manager
    pub async fn initialize(&mut self) -> Result<()> {
        println!("🚀 Initializing WebView2 Manager...");
        
        // Environment prüfen
        let env_info = self.environment_detector.check_availability()?;
        if !env_info.is_available {
            return Err(anyhow::anyhow!("WebView2 Runtime not available"));
        }
        
        println!("✅ WebView2 Environment: {} ({})", env_info.version, env_info.runtime_type);
        
        // Performance-Monitoring starten
        self.performance_monitor.start_monitoring();
        
        self.is_initialized = true;
        println!("✅ WebView2 Manager initialized successfully!");
        
        Ok(())
    }

    /// Erstellt eine neue WebView2-Instanz
    pub async fn create_instance(&mut self, id: String, parent_window: HWND) -> Result<()> {
        self.create_instance_with_config(id, parent_window, None).await
    }

    /// Erstellt eine neue WebView2-Instanz mit spezifischer Konfiguration
    pub async fn create_instance_with_config(
        &mut self, 
        id: String, 
        parent_window: HWND, 
        config: Option<WebView2Config>
    ) -> Result<()> {
        if !self.is_initialized {
            return Err(anyhow::anyhow!("Manager not initialized"));
        }
        
        if self.instances.contains_key(&id) {
            return Err(anyhow::anyhow!("Instance with ID '{}' already exists", id));
        }
        
        println!("🚀 Creating WebView2 instance: {}", id);
        
        // Konfiguration bestimmen
        let instance_config = config.unwrap_or_else(|| self.global_config.clone());
        
        // Engine erstellen und konfigurieren
        let mut engine = WebView2Engine::with_config(instance_config.clone());
        engine.create_container(parent_window)?;
        engine.initialize().await?;
        
        // Instanz erstellen
        let instance = WebView2Instance {
            id: id.clone(),
            engine,
            config: instance_config,
            parent_window,
            created_at: std::time::SystemTime::now(),
            last_activity: std::time::SystemTime::now(),
            status: InstanceStatus::Active,
            metrics: InstanceMetrics::new(),
        };
        
        // Instanz registrieren
        self.instances.insert(id.clone(), instance);
        self.active_instance = Some(id.clone());
        
        // Statistiken aktualisieren
        self.statistics.total_instances_created += 1;
        self.statistics.active_instances += 1;
        
        println!("✅ WebView2 instance '{}' created successfully!", id);
        Ok(())
    }

    /// Zerstört eine WebView2-Instanz
    pub async fn destroy_instance(&mut self, id: &str) -> Result<()> {
        if let Some(mut instance) = self.instances.remove(id) {
            println!("🗑️ Destroying WebView2 instance: {}", id);
            
            instance.status = InstanceStatus::Destroying;
            instance.engine.cleanup()?;
            instance.status = InstanceStatus::Destroyed;
            
            // Aktive Instanz zurücksetzen falls nötig
            if self.active_instance.as_ref() == Some(&id.to_string()) {
                self.active_instance = None;
                
                // Neue aktive Instanz wählen
                if let Some(next_id) = self.instances.keys().next() {
                    self.active_instance = Some(next_id.clone());
                }
            }
            
            // Statistiken aktualisieren
            self.statistics.active_instances = self.statistics.active_instances.saturating_sub(1);
            
            // Lebensdauer berechnen
            if let Ok(lifetime) = instance.created_at.elapsed() {
                self.update_average_lifetime(lifetime);
            }
            
            println!("✅ WebView2 instance '{}' destroyed", id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", id))
        }
    }

    /// Setzt die aktive Instanz
    pub fn set_active_instance(&mut self, id: &str) -> Result<()> {
        if self.instances.contains_key(id) {
            self.active_instance = Some(id.to_string());
            
            // Letzte Aktivität aktualisieren
            if let Some(instance) = self.instances.get_mut(id) {
                instance.last_activity = std::time::SystemTime::now();
            }
            
            println!("✅ Active instance set to: {}", id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", id))
        }
    }

    /// Navigiert die aktive Instanz zu einer URL
    pub async fn navigate_active_to_url(&mut self, url: &str) -> Result<()> {
        if let Some(active_id) = &self.active_instance.clone() {
            self.navigate_instance_to_url(active_id, url).await
        } else {
            Err(anyhow::anyhow!("No active instance"))
        }
    }

    /// Navigiert eine spezifische Instanz zu einer URL
    pub async fn navigate_instance_to_url(&mut self, instance_id: &str, url: &str) -> Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            instance.engine.navigate_to_url(url).await?;
            instance.last_activity = std::time::SystemTime::now();
            instance.metrics.navigation_count += 1;
            
            // Globale Statistiken aktualisieren
            self.statistics.total_navigations += 1;
            
            Ok(())
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", instance_id))
        }
    }

    /// Führt JavaScript in der aktiven Instanz aus
    pub async fn execute_script_in_active(&mut self, script: &str) -> Result<String> {
        if let Some(active_id) = &self.active_instance.clone() {
            self.execute_script_in_instance(active_id, script).await
        } else {
            Err(anyhow::anyhow!("No active instance"))
        }
    }

    /// Führt JavaScript in einer spezifischen Instanz aus
    pub async fn execute_script_in_instance(&mut self, instance_id: &str, script: &str) -> Result<String> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            let result = instance.engine.execute_script(script).await?;
            instance.last_activity = std::time::SystemTime::now();
            instance.metrics.javascript_executions += 1;
            
            // Globale Statistiken aktualisieren
            self.statistics.total_javascript_executions += 1;
            
            Ok(result)
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", instance_id))
        }
    }

    /// Pausiert eine Instanz
    pub fn pause_instance(&mut self, instance_id: &str) -> Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            instance.status = InstanceStatus::Paused;
            println!("⏸️ Instance '{}' paused", instance_id);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", instance_id))
        }
    }

    /// Reaktiviert eine pausierte Instanz
    pub fn resume_instance(&mut self, instance_id: &str) -> Result<()> {
        if let Some(instance) = self.instances.get_mut(instance_id) {
            if instance.status == InstanceStatus::Paused {
                instance.status = InstanceStatus::Active;
                instance.last_activity = std::time::SystemTime::now();
                println!("▶️ Instance '{}' resumed", instance_id);
                Ok(())
            } else {
                Err(anyhow::anyhow!("Instance '{}' is not paused", instance_id))
            }
        } else {
            Err(anyhow::anyhow!("Instance '{}' not found", instance_id))
        }
    }

    /// Gibt alle Instanz-IDs zurück
    pub fn get_instance_ids(&self) -> Vec<String> {
        self.instances.keys().cloned().collect()
    }

    /// Gibt die aktive Instanz-ID zurück
    pub fn get_active_instance_id(&self) -> Option<&String> {
        self.active_instance.as_ref()
    }

    /// Gibt eine Instanz zurück (immutable)
    pub fn get_instance(&self, id: &str) -> Option<&WebView2Instance> {
        self.instances.get(id)
    }

    /// Gibt eine Instanz zurück (mutable)
    pub fn get_instance_mut(&mut self, id: &str) -> Option<&mut WebView2Instance> {
        self.instances.get_mut(id)
    }

    /// Gibt die aktive Instanz zurück (immutable)
    pub fn get_active_instance(&self) -> Option<&WebView2Instance> {
        self.active_instance.as_ref()
            .and_then(|id| self.instances.get(id))
    }

    /// Gibt die aktive Instanz zurück (mutable)
    pub fn get_active_instance_mut(&mut self) -> Option<&mut WebView2Instance> {
        let active_id = self.active_instance.clone();
        active_id.as_ref()
            .and_then(|id| self.instances.get_mut(id))
    }

    /// Aktualisiert Performance-Metriken
    pub fn update_performance(&mut self) {
        self.performance_monitor.update_metrics();
        
        // Instanz-Metriken aktualisieren
        for (_, instance) in &mut self.instances {
            instance.update_metrics();
        }
    }

    /// Bereinigt inaktive Instanzen
    pub async fn cleanup_inactive_instances(&mut self, max_idle_time: std::time::Duration) -> Result<()> {
        let now = std::time::SystemTime::now();
        let mut to_remove = Vec::new();
        
        for (id, instance) in &self.instances {
            if let Ok(idle_time) = now.duration_since(instance.last_activity) {
                if idle_time > max_idle_time && instance.status != InstanceStatus::Active {
                    to_remove.push(id.clone());
                }
            }
        }
        
        for id in to_remove {
            println!("🧹 Cleaning up inactive instance: {}", id);
            self.destroy_instance(&id).await?;
        }
        
        Ok(())
    }

    /// Gibt Manager-Statistiken zurück
    pub fn get_statistics(&self) -> &ManagerStatistics {
        &self.statistics
    }

    /// Gibt Performance-Metriken zurück
    pub fn get_performance_metrics(&self) -> &WebView2PerformanceMonitor {
        &self.performance_monitor
    }

    /// Erstellt einen detaillierten Manager-Report
    pub fn generate_manager_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("🎯 WEBVIEW2 MANAGER REPORT\n");
        report.push_str("==========================\n\n");
        
        // Manager-Status
        report.push_str(&format!("📊 MANAGER STATUS:\n"));
        report.push_str(&format!("• Initialized: {}\n", self.is_initialized));
        report.push_str(&format!("• Total Instances: {}\n", self.instances.len()));
        report.push_str(&format!("• Active Instance: {}\n", 
            self.active_instance.as_ref().unwrap_or(&"None".to_string())));
        report.push_str("\n");
        
        // Statistiken
        report.push_str("📈 STATISTICS:\n");
        report.push_str(&format!("• Total Instances Created: {}\n", self.statistics.total_instances_created));
        report.push_str(&format!("• Active Instances: {}\n", self.statistics.active_instances));
        report.push_str(&format!("• Failed Instances: {}\n", self.statistics.failed_instances));
        report.push_str(&format!("• Total Navigations: {}\n", self.statistics.total_navigations));
        report.push_str(&format!("• Total JS Executions: {}\n", self.statistics.total_javascript_executions));
        
        if let Ok(uptime) = self.statistics.manager_start_time.elapsed() {
            report.push_str(&format!("• Manager Uptime: {:.1} minutes\n", uptime.as_secs_f64() / 60.0));
        }
        report.push_str("\n");
        
        // Instanz-Details
        if !self.instances.is_empty() {
            report.push_str("🔧 INSTANCE DETAILS:\n");
            for (id, instance) in &self.instances {
                report.push_str(&format!("• {}: {:?} ({} navigations, {} JS executions)\n", 
                    id, instance.status, instance.metrics.navigation_count, instance.metrics.javascript_executions));
            }
            report.push_str("\n");
        }
        
        // Performance-Report anhängen
        report.push_str(&self.performance_monitor.generate_performance_report());
        
        report
    }

    /// Bereinigt alle Instanzen und den Manager
    pub async fn cleanup(&mut self) -> Result<()> {
        println!("🧹 Cleaning up WebView2 Manager...");
        
        // Alle Instanzen zerstören
        let instance_ids: Vec<String> = self.instances.keys().cloned().collect();
        for id in instance_ids {
            self.destroy_instance(&id).await?;
        }
        
        self.instances.clear();
        self.active_instance = None;
        self.is_initialized = false;
        
        println!("✅ WebView2 Manager cleaned up");
        Ok(())
    }

    /// Aktualisiert die durchschnittliche Instanz-Lebensdauer
    fn update_average_lifetime(&mut self, lifetime: std::time::Duration) {
        let total_instances = self.statistics.total_instances_created;
        if total_instances > 0 {
            let current_avg = self.statistics.average_instance_lifetime.as_secs_f64();
            let new_lifetime = lifetime.as_secs_f64();
            let new_avg = (current_avg * (total_instances - 1) as f64 + new_lifetime) / total_instances as f64;
            self.statistics.average_instance_lifetime = std::time::Duration::from_secs_f64(new_avg);
        }
    }

    // Getter
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }

    pub fn get_global_config(&self) -> &WebView2Config {
        &self.global_config
    }

    pub fn instance_count(&self) -> usize {
        self.instances.len()
    }

    pub fn has_active_instance(&self) -> bool {
        self.active_instance.is_some()
    }
}

impl WebView2Instance {
    /// Aktualisiert die Instanz-Metriken
    pub fn update_metrics(&mut self) {
        // Laufzeit berechnen
        if let Ok(uptime) = self.created_at.elapsed() {
            self.metrics.total_uptime = uptime;
        }
        
        // Performance-Score berechnen (vereinfacht)
        self.metrics.performance_score = self.calculate_performance_score();
    }

    /// Berechnet einen Performance-Score (0-100)
    fn calculate_performance_score(&self) -> u8 {
        let mut score = 100u8;
        
        // Abzug für Fehler
        if !self.metrics.last_errors.is_empty() {
            score = score.saturating_sub(self.metrics.last_errors.len() as u8 * 10);
        }
        
        // Abzug basierend auf Status
        match self.status {
            InstanceStatus::Active => {},
            InstanceStatus::Paused => score = score.saturating_sub(20),
            InstanceStatus::Error(_) => score = score.saturating_sub(50),
            _ => score = score.saturating_sub(30),
        }
        
        score
    }

    /// Fügt einen Fehler hinzu
    pub fn add_error(&mut self, error: String) {
        self.metrics.last_errors.push(error.clone());
        
        // Begrenzte Anzahl von Fehlern speichern
        if self.metrics.last_errors.len() > 10 {
            self.metrics.last_errors.remove(0);
        }
        
        self.status = InstanceStatus::Error(error);
    }

    /// Prüft ob die Instanz aktiv ist
    pub fn is_active(&self) -> bool {
        matches!(self.status, InstanceStatus::Active)
    }

    /// Prüft ob die Instanz einen Fehler hat
    pub fn has_error(&self) -> bool {
        matches!(self.status, InstanceStatus::Error(_))
    }
}

impl InstanceMetrics {
    /// Erstellt neue Instanz-Metriken
    pub fn new() -> Self {
        Self {
            navigation_count: 0,
            javascript_executions: 0,
            total_uptime: std::time::Duration::new(0, 0),
            last_errors: Vec::new(),
            performance_score: 100,
        }
    }
}

impl ManagerStatistics {
    /// Erstellt neue Manager-Statistiken
    pub fn new() -> Self {
        Self {
            total_instances_created: 0,
            active_instances: 0,
            failed_instances: 0,
            average_instance_lifetime: std::time::Duration::new(0, 0),
            total_navigations: 0,
            total_javascript_executions: 0,
            manager_start_time: std::time::SystemTime::now(),
        }
    }
}

impl Default for OptimizedWebView2Manager {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for InstanceMetrics {
    fn default() -> Self {
        Self::new()
    }
}

impl Default for ManagerStatistics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_manager_creation() {
        let manager = OptimizedWebView2Manager::new();
        assert!(!manager.is_initialized);
        assert_eq!(manager.instances.len(), 0);
        assert!(manager.active_instance.is_none());
    }

    #[test]
    fn test_manager_with_config() {
        let config = WebView2Config::development();
        let manager = OptimizedWebView2Manager::with_global_config(config.clone());
        assert_eq!(manager.global_config.debug.enable_dev_tools, config.debug.enable_dev_tools);
    }

    #[test]
    fn test_instance_status() {
        assert_eq!(InstanceStatus::Active, InstanceStatus::Active);
        assert_ne!(InstanceStatus::Active, InstanceStatus::Paused);
        
        let error_status = InstanceStatus::Error("Test error".to_string());
        if let InstanceStatus::Error(msg) = error_status {
            assert_eq!(msg, "Test error");
        } else {
            panic!("Expected Error status");
        }
    }

    #[test]
    fn test_instance_metrics() {
        let mut metrics = InstanceMetrics::new();
        assert_eq!(metrics.navigation_count, 0);
        assert_eq!(metrics.javascript_executions, 0);
        assert_eq!(metrics.performance_score, 100);
        
        metrics.navigation_count = 5;
        metrics.javascript_executions = 10;
        assert_eq!(metrics.navigation_count, 5);
        assert_eq!(metrics.javascript_executions, 10);
    }

    #[test]
    fn test_manager_statistics() {
        let stats = ManagerStatistics::new();
        assert_eq!(stats.total_instances_created, 0);
        assert_eq!(stats.active_instances, 0);
        assert_eq!(stats.failed_instances, 0);
        assert_eq!(stats.total_navigations, 0);
        assert_eq!(stats.total_javascript_executions, 0);
    }

    #[test]
    fn test_performance_score_calculation() {
        let hwnd = HWND(std::ptr::null_mut());
        let mut instance = WebView2Instance {
            id: "test".to_string(),
            engine: WebView2Engine::new(),
            config: WebView2Config::default(),
            parent_window: hwnd,
            created_at: std::time::SystemTime::now(),
            last_activity: std::time::SystemTime::now(),
            status: InstanceStatus::Active,
            metrics: InstanceMetrics::new(),
        };
        
        // Anfangs sollte der Score 100 sein
        assert_eq!(instance.calculate_performance_score(), 100);
        
        // Nach Fehler sollte der Score niedriger sein
        instance.add_error("Test error".to_string());
        assert!(instance.calculate_performance_score() < 100);
        assert!(instance.has_error());
        assert!(!instance.is_active());
    }

    #[test]
    fn test_manager_report_generation() {
        let manager = OptimizedWebView2Manager::new();
        let report = manager.generate_manager_report();
        
        assert!(report.contains("WEBVIEW2 MANAGER REPORT"));
        assert!(report.contains("MANAGER STATUS"));
        assert!(report.contains("STATISTICS"));
    }
}
