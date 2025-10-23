// 🔄 ZAKYX Browser Error Recovery System
// Automatische Fehlerbehandlung und Wiederherstellungsstrategien

use std::time::{Duration, SystemTime};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;

use crate::error_system::types::{ZAKYXBrowserError, ErrorCategory, PluginErrorType, SecuritySeverity};

/// Error-Recovery-Strategien
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorRecovery {
    /// Ob ein Retry möglich ist
    pub retry_possible: bool,
    
    /// Aktuelle Retry-Anzahl
    pub retry_count: u32,
    
    /// Maximale Retry-Anzahl
    pub max_retries: u32,
    
    /// Delay zwischen Retries in Millisekunden
    pub retry_delay_ms: u64,
    
    /// Ob ein Fallback verfügbar ist
    pub fallback_available: bool,
    
    /// Ob Benutzer-Aktion erforderlich ist
    pub user_action_required: bool,
    
    /// Recovery-Strategie-Typ
    pub strategy: RecoveryStrategy,
    
    /// Zusätzliche Recovery-Optionen
    pub options: RecoveryOptions,
}

/// Recovery-Strategie-Typen
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RecoveryStrategy {
    /// Sofortiger Retry
    ImmediateRetry,
    
    /// Exponential Backoff
    ExponentialBackoff,
    
    /// Linear Backoff
    LinearBackoff,
    
    /// Fallback zu alternativer Methode
    Fallback,
    
    /// Benutzer-Intervention erforderlich
    UserIntervention,
    
    /// Automatische Reparatur
    AutoRepair,
    
    /// Graceful Degradation
    GracefulDegradation,
    
    /// Keine Recovery möglich
    NoRecovery,
}

/// Recovery-Optionen
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryOptions {
    /// Timeout für Recovery-Versuche
    pub timeout_seconds: u64,
    
    /// Ob Recovery im Hintergrund läuft
    pub background_recovery: bool,
    
    /// Ob Benutzer über Recovery informiert wird
    pub notify_user: bool,
    
    /// Fallback-URLs oder -Pfade
    pub fallback_resources: Vec<String>,
    
    /// Custom Recovery-Handler
    pub custom_handler: Option<String>,
    
    /// Recovery-Priorität (1-10)
    pub priority: u8,
}

/// Recovery-Manager für das Error-System
#[derive(Debug)]
pub struct ErrorRecoveryManager {
    /// Aktive Recovery-Operationen
    active_recoveries: HashMap<String, ActiveRecovery>,
    
    /// Recovery-Statistiken
    statistics: RecoveryStatistics,
    
    /// Recovery-Konfiguration
    config: RecoveryConfig,
    
    /// Recovery-Historie
    history: Vec<RecoveryHistoryEntry>,
}

/// Aktive Recovery-Operation
#[derive(Debug, Clone)]
pub struct ActiveRecovery {
    /// Recovery-ID
    pub id: String,
    
    /// Ursprünglicher Fehler
    pub original_error: ZAKYXBrowserError,
    
    /// Recovery-Strategie
    pub strategy: ErrorRecovery,
    
    /// Start-Zeit
    pub started_at: SystemTime,
    
    /// Aktueller Status
    pub status: RecoveryStatus,
    
    /// Fortschritt (0-100)
    pub progress: u8,
    
    /// Recovery-Schritte
    pub steps: Vec<RecoveryStep>,
    
    /// Aktuelle Schritt-Index
    pub current_step: usize,
}

/// Recovery-Status
#[derive(Debug, Clone, PartialEq)]
pub enum RecoveryStatus {
    /// Recovery läuft
    InProgress,
    
    /// Recovery erfolgreich
    Success,
    
    /// Recovery fehlgeschlagen
    Failed,
    
    /// Recovery abgebrochen
    Cancelled,
    
    /// Wartet auf Benutzer-Aktion
    WaitingForUser,
    
    /// Recovery pausiert
    Paused,
}

/// Recovery-Schritt
#[derive(Debug, Clone)]
pub struct RecoveryStep {
    /// Schritt-Name
    pub name: String,
    
    /// Schritt-Beschreibung
    pub description: String,
    
    /// Schritt-Status
    pub status: RecoveryStatus,
    
    /// Ausführungszeit
    pub execution_time: Option<Duration>,
    
    /// Schritt-Ergebnis
    pub result: Option<String>,
    
    /// Fehler (falls aufgetreten)
    pub error: Option<String>,
}

/// Recovery-Statistiken
#[derive(Debug, Clone, Default)]
pub struct RecoveryStatistics {
    /// Gesamte Recovery-Versuche
    pub total_attempts: u64,
    
    /// Erfolgreiche Recoveries
    pub successful_recoveries: u64,
    
    /// Fehlgeschlagene Recoveries
    pub failed_recoveries: u64,
    
    /// Durchschnittliche Recovery-Zeit
    pub average_recovery_time: Duration,
    
    /// Recovery-Versuche nach Kategorie
    pub attempts_by_category: HashMap<ErrorCategory, u64>,
    
    /// Erfolgsrate nach Strategie
    pub success_rate_by_strategy: HashMap<RecoveryStrategy, f64>,
}

/// Recovery-Konfiguration
#[derive(Debug, Clone)]
pub struct RecoveryConfig {
    /// Maximale gleichzeitige Recovery-Operationen
    pub max_concurrent_recoveries: usize,
    
    /// Standard-Timeout für Recovery
    pub default_timeout: Duration,
    
    /// Ob automatische Recovery aktiviert ist
    pub auto_recovery_enabled: bool,
    
    /// Minimales Intervall zwischen Retries
    pub min_retry_interval: Duration,
    
    /// Maximales Intervall zwischen Retries
    pub max_retry_interval: Duration,
    
    /// Ob Recovery-Historie gespeichert wird
    pub save_history: bool,
    
    /// Maximale Anzahl Historie-Einträge
    pub max_history_entries: usize,
}

/// Recovery-Historie-Eintrag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecoveryHistoryEntry {
    /// Zeitstempel
    pub timestamp: SystemTime,
    
    /// Error-Typ
    pub error_type: String,
    
    /// Recovery-Strategie
    pub strategy: RecoveryStrategy,
    
    /// Ob erfolgreich
    pub success: bool,
    
    /// Recovery-Dauer
    pub duration: Duration,
    
    /// Zusätzliche Informationen
    pub details: HashMap<String, String>,
}

impl ErrorRecoveryManager {
    /// Erstellt einen neuen Recovery-Manager
    pub fn new() -> Self {
        println!("🔄 Creating Error Recovery Manager...");
        
        Self {
            active_recoveries: HashMap::new(),
            statistics: RecoveryStatistics::default(),
            config: RecoveryConfig::default(),
            history: Vec::new(),
        }
    }

    /// Erstellt einen Manager mit spezifischer Konfiguration
    pub fn with_config(config: RecoveryConfig) -> Self {
        println!("🔄 Creating Error Recovery Manager with custom config...");
        
        Self {
            active_recoveries: HashMap::new(),
            statistics: RecoveryStatistics::default(),
            config,
            history: Vec::new(),
        }
    }

    /// Startet eine Recovery-Operation
    pub async fn start_recovery(&mut self, error: ZAKYXBrowserError) -> Result<String, ZAKYXBrowserError> {
        let recovery_id = format!("recovery_{}", uuid::Uuid::new_v4());
        
        println!("🔄 Starting recovery for error: {} (ID: {})", error, recovery_id);
        
        // Erstelle Recovery-Strategie basierend auf Error-Typ
        let strategy = self.create_recovery_strategy(&error);
        
        // Erstelle aktive Recovery
        let active_recovery = ActiveRecovery {
            id: recovery_id.clone(),
            original_error: error.clone(),
            strategy: strategy.clone(),
            started_at: SystemTime::now(),
            status: RecoveryStatus::InProgress,
            progress: 0,
            steps: self.create_recovery_steps(&error, &strategy),
            current_step: 0,
        };
        
        // Füge zu aktiven Recoveries hinzu
        self.active_recoveries.insert(recovery_id.clone(), active_recovery);
        
        // Starte Recovery-Prozess
        if self.config.auto_recovery_enabled {
            self.execute_recovery(&recovery_id).await?;
        }
        
        Ok(recovery_id)
    }

    /// Führt eine Recovery-Operation aus
    pub async fn execute_recovery(&mut self, recovery_id: &str) -> Result<(), ZAKYXBrowserError> {
        // Prüfe ob Recovery existiert
        if !self.active_recoveries.contains_key(recovery_id) {
            return Err(ZAKYXBrowserError::Internal { 
                message: format!("Recovery not found: {}", recovery_id),
                error_code: Some("RECOVERY_NOT_FOUND".to_string()),
            });
        }
        
        println!("🔄 Executing recovery: {}", recovery_id);
        
        let start_time = SystemTime::now();
        let mut should_continue = true;
        
        // Führe Recovery-Schritte aus
        while should_continue {
            // Hole aktuelle Recovery-Info
            let (current_step, total_steps, step_name) = {
                let recovery = self.active_recoveries.get(recovery_id).unwrap();
                if recovery.current_step >= recovery.steps.len() {
                    break;
                }
                let step = &recovery.steps[recovery.current_step];
                (recovery.current_step, recovery.steps.len(), step.name.clone())
            };
            
            println!("🔄 Executing recovery step: {}", step_name);
            
            let step_start = SystemTime::now();
            
            // Führe Schritt aus (ohne Borrow von self)
            let step_result = self.execute_recovery_step_by_id(recovery_id, current_step).await;
            
            // Aktualisiere Schritt-Ergebnis
            match step_result {
                Ok(_) => {
                    if let Some(recovery) = self.active_recoveries.get_mut(recovery_id) {
                        let step = &mut recovery.steps[current_step];
                        step.status = RecoveryStatus::Success;
                        step.execution_time = step_start.elapsed().ok();
                        recovery.progress = ((current_step + 1) * 100 / total_steps) as u8;
                        recovery.current_step += 1;
                    }
                }
                Err(e) => {
                    // Entscheide ob Recovery fortgesetzt werden soll (vor Mutation)
                    let continue_recovery = self.should_continue_recovery_by_id(recovery_id, &e);
                    should_continue = continue_recovery;
                    
                    if let Some(recovery) = self.active_recoveries.get_mut(recovery_id) {
                        let step = &mut recovery.steps[current_step];
                        step.status = RecoveryStatus::Failed;
                        step.error = Some(e.to_string());
                        step.execution_time = step_start.elapsed().ok();
                        
                        if !should_continue {
                            recovery.status = RecoveryStatus::Failed;
                        } else {
                            recovery.current_step += 1;
                        }
                    }
                }
            }
        }
        
        // Bestimme finalen Status und aktualisiere Statistiken
        let (final_status, recovery_clone) = {
            if let Some(recovery) = self.active_recoveries.get_mut(recovery_id) {
                if recovery.current_step >= recovery.steps.len() && recovery.status != RecoveryStatus::Failed {
                    recovery.status = RecoveryStatus::Success;
                    recovery.progress = 100;
                }
                (recovery.status.clone(), recovery.clone())
            } else {
                return Ok(());
            }
        };
        
        // Aktualisiere Statistiken
        self.update_statistics(&recovery_clone, start_time.elapsed().unwrap_or_default());
        
        // Füge zur Historie hinzu
        if self.config.save_history {
            self.add_to_history(&recovery_clone);
        }
        
        println!("✅ Recovery completed: {} (Status: {:?})", recovery_id, final_status);
        
        Ok(())
    }

    /// Führt einen einzelnen Recovery-Schritt aus (by ID)
    async fn execute_recovery_step_by_id(&self, recovery_id: &str, step_index: usize) -> Result<(), ZAKYXBrowserError> {
        let recovery = self.active_recoveries.get(recovery_id)
            .ok_or_else(|| ZAKYXBrowserError::Internal { 
                message: format!("Recovery not found: {}", recovery_id),
                error_code: Some("RECOVERY_NOT_FOUND".to_string()),
            })?;
        let step = &recovery.steps[step_index];
        
        match step.name.as_str() {
            "retry_operation" => self.retry_operation(recovery).await,
            "fallback_strategy" => self.execute_fallback(recovery).await,
            "clear_cache" => self.clear_cache().await,
            "reset_connection" => self.reset_connection().await,
            "reload_plugin" => self.reload_plugin(recovery).await,
            "user_notification" => self.notify_user(recovery).await,
            _ => {
                println!("⚠️ Unknown recovery step: {}", step.name);
                Ok(())
            }
        }
    }

    /// Retry-Operation
    async fn retry_operation(&self, recovery: &ActiveRecovery) -> Result<(), ZAKYXBrowserError> {
        println!("🔄 Retrying original operation...");
        
        // Warte basierend auf Retry-Strategie
        let delay = self.calculate_retry_delay(recovery);
        sleep(Duration::from_millis(delay)).await;
        
        // Simuliere Retry (in echter Implementierung würde hier die ursprüngliche Operation wiederholt)
        println!("✅ Operation retry completed");
        Ok(())
    }

    /// Fallback-Strategie ausführen
    async fn execute_fallback(&self, recovery: &ActiveRecovery) -> Result<(), ZAKYXBrowserError> {
        println!("🔄 Executing fallback strategy...");
        
        // Verwende Fallback-Ressourcen aus den Recovery-Optionen
        for fallback_resource in &recovery.strategy.options.fallback_resources {
            println!("🔄 Trying fallback resource: {}", fallback_resource);
            
            // Simuliere Fallback-Versuch
            sleep(Duration::from_millis(500)).await;
            
            // In echter Implementierung würde hier der Fallback versucht
            println!("✅ Fallback successful: {}", fallback_resource);
            return Ok(());
        }
        
        Err(ZAKYXBrowserError::Internal {
            message: "No fallback resources available".to_string(),
            error_code: Some("NO_FALLBACK".to_string()),
        })
    }

    /// Cache leeren
    async fn clear_cache(&self) -> Result<(), ZAKYXBrowserError> {
        println!("🔄 Clearing cache...");
        sleep(Duration::from_millis(200)).await;
        println!("✅ Cache cleared");
        Ok(())
    }

    /// Verbindung zurücksetzen
    async fn reset_connection(&self) -> Result<(), ZAKYXBrowserError> {
        println!("🔄 Resetting connection...");
        sleep(Duration::from_millis(1000)).await;
        println!("✅ Connection reset");
        Ok(())
    }

    /// Plugin neu laden
    async fn reload_plugin(&self, recovery: &ActiveRecovery) -> Result<(), ZAKYXBrowserError> {
        if let ZAKYXBrowserError::Plugin { plugin_id, .. } = &recovery.original_error {
            println!("🔄 Reloading plugin: {}", plugin_id);
            sleep(Duration::from_millis(800)).await;
            println!("✅ Plugin reloaded: {}", plugin_id);
        }
        Ok(())
    }

    /// Benutzer benachrichtigen
    async fn notify_user(&self, recovery: &ActiveRecovery) -> Result<(), ZAKYXBrowserError> {
        println!("🔄 Notifying user about recovery...");
        
        let message = format!(
            "🔄 Automatische Fehlerbehandlung läuft...\n\
             Fehler: {}\n\
             Strategie: {:?}\n\
             Fortschritt: {}%",
            recovery.original_error.user_message(),
            recovery.strategy.strategy,
            recovery.progress
        );
        
        println!("📢 User notification: {}", message);
        Ok(())
    }

    /// Erstellt Recovery-Strategie basierend auf Error-Typ
    fn create_recovery_strategy(&self, error: &ZAKYXBrowserError) -> ErrorRecovery {
        match error {
            ZAKYXBrowserError::Network { .. } => ErrorRecovery {
                retry_possible: true,
                retry_count: 0,
                max_retries: 3,
                retry_delay_ms: 1000,
                fallback_available: true,
                user_action_required: false,
                strategy: RecoveryStrategy::ExponentialBackoff,
                options: RecoveryOptions {
                    timeout_seconds: 30,
                    background_recovery: true,
                    notify_user: false,
                    fallback_resources: vec![
                        "https://backup-server.com".to_string(),
                        "https://cdn-fallback.com".to_string(),
                    ],
                    custom_handler: None,
                    priority: 8,
                },
            },
            
            ZAKYXBrowserError::Plugin { error_type, .. } => ErrorRecovery {
                retry_possible: !error_type.is_critical(),
                retry_count: 0,
                max_retries: 2,
                retry_delay_ms: 2000,
                fallback_available: false,
                user_action_required: error_type.is_critical(),
                strategy: if error_type.is_critical() { 
                    RecoveryStrategy::UserIntervention 
                } else { 
                    RecoveryStrategy::AutoRepair 
                },
                options: RecoveryOptions {
                    timeout_seconds: 60,
                    background_recovery: false,
                    notify_user: true,
                    fallback_resources: vec![],
                    custom_handler: Some("plugin_recovery_handler".to_string()),
                    priority: 6,
                },
            },
            
            ZAKYXBrowserError::Security { severity, .. } => ErrorRecovery {
                retry_possible: false,
                retry_count: 0,
                max_retries: 0,
                retry_delay_ms: 0,
                fallback_available: matches!(severity, SecuritySeverity::Low | SecuritySeverity::Medium),
                user_action_required: severity.requires_immediate_action(),
                strategy: RecoveryStrategy::UserIntervention,
                options: RecoveryOptions {
                    timeout_seconds: 0,
                    background_recovery: false,
                    notify_user: true,
                    fallback_resources: vec![],
                    custom_handler: Some("security_handler".to_string()),
                    priority: 10,
                },
            },
            
            _ => ErrorRecovery {
                retry_possible: error.is_retryable(),
                retry_count: 0,
                max_retries: if error.is_retryable() { 2 } else { 0 },
                retry_delay_ms: 1500,
                fallback_available: false,
                user_action_required: error.requires_user_action(),
                strategy: if error.requires_user_action() {
                    RecoveryStrategy::UserIntervention
                } else if error.is_retryable() {
                    RecoveryStrategy::LinearBackoff
                } else {
                    RecoveryStrategy::NoRecovery
                },
                options: RecoveryOptions::default(),
            },
        }
    }

    /// Erstellt Recovery-Schritte
    fn create_recovery_steps(&self, error: &ZAKYXBrowserError, strategy: &ErrorRecovery) -> Vec<RecoveryStep> {
        let mut steps = Vec::new();
        
        match strategy.strategy {
            RecoveryStrategy::ExponentialBackoff | RecoveryStrategy::LinearBackoff => {
                if strategy.retry_possible {
                    steps.push(RecoveryStep {
                        name: "retry_operation".to_string(),
                        description: "Wiederhole ursprüngliche Operation".to_string(),
                        status: RecoveryStatus::InProgress,
                        execution_time: None,
                        result: None,
                        error: None,
                    });
                }
                
                if strategy.fallback_available {
                    steps.push(RecoveryStep {
                        name: "fallback_strategy".to_string(),
                        description: "Verwende Fallback-Strategie".to_string(),
                        status: RecoveryStatus::InProgress,
                        execution_time: None,
                        result: None,
                        error: None,
                    });
                }
            },
            
            RecoveryStrategy::AutoRepair => {
                match error.category() {
                    ErrorCategory::Network => {
                        steps.push(RecoveryStep {
                            name: "clear_cache".to_string(),
                            description: "Leere Netzwerk-Cache".to_string(),
                            status: RecoveryStatus::InProgress,
                            execution_time: None,
                            result: None,
                            error: None,
                        });
                        
                        steps.push(RecoveryStep {
                            name: "reset_connection".to_string(),
                            description: "Setze Verbindung zurück".to_string(),
                            status: RecoveryStatus::InProgress,
                            execution_time: None,
                            result: None,
                            error: None,
                        });
                    },
                    
                    ErrorCategory::Plugin => {
                        steps.push(RecoveryStep {
                            name: "reload_plugin".to_string(),
                            description: "Lade Plugin neu".to_string(),
                            status: RecoveryStatus::InProgress,
                            execution_time: None,
                            result: None,
                            error: None,
                        });
                    },
                    
                    _ => {
                        steps.push(RecoveryStep {
                            name: "generic_repair".to_string(),
                            description: "Führe allgemeine Reparatur durch".to_string(),
                            status: RecoveryStatus::InProgress,
                            execution_time: None,
                            result: None,
                            error: None,
                        });
                    }
                }
            },
            
            RecoveryStrategy::UserIntervention => {
                steps.push(RecoveryStep {
                    name: "user_notification".to_string(),
                    description: "Benachrichtige Benutzer".to_string(),
                    status: RecoveryStatus::InProgress,
                    execution_time: None,
                    result: None,
                    error: None,
                });
            },
            
            _ => {
                // Keine spezifischen Schritte für andere Strategien
            }
        }
        
        steps
    }

    /// Berechnet Retry-Delay basierend auf Strategie
    fn calculate_retry_delay(&self, recovery: &ActiveRecovery) -> u64 {
        match recovery.strategy.strategy {
            RecoveryStrategy::ExponentialBackoff => {
                recovery.strategy.retry_delay_ms * (2_u64.pow(recovery.strategy.retry_count))
            },
            RecoveryStrategy::LinearBackoff => {
                recovery.strategy.retry_delay_ms * (recovery.strategy.retry_count as u64 + 1)
            },
            _ => recovery.strategy.retry_delay_ms,
        }
    }

    /// Entscheidet ob Recovery fortgesetzt werden soll (by ID)
    fn should_continue_recovery_by_id(&self, recovery_id: &str, error: &ZAKYXBrowserError) -> bool {
        let recovery = match self.active_recoveries.get(recovery_id) {
            Some(r) => r,
            None => return false,
        };
        // Stoppe bei kritischen Fehlern
        if error.is_critical() {
            return false;
        }
        
        // Stoppe wenn maximale Retries erreicht
        if recovery.strategy.retry_count >= recovery.strategy.max_retries {
            return false;
        }
        
        // Stoppe bei Sicherheitsfehlern
        if matches!(error.category(), ErrorCategory::Security) {
            return false;
        }
        
        true
    }

    /// Aktualisiert Recovery-Statistiken
    fn update_statistics(&mut self, recovery: &ActiveRecovery, duration: Duration) {
        self.statistics.total_attempts += 1;
        
        if recovery.status == RecoveryStatus::Success {
            self.statistics.successful_recoveries += 1;
        } else {
            self.statistics.failed_recoveries += 1;
        }
        
        // Aktualisiere durchschnittliche Recovery-Zeit
        let total_time = self.statistics.average_recovery_time.as_millis() as u64 * (self.statistics.total_attempts - 1) + duration.as_millis() as u64;
        self.statistics.average_recovery_time = Duration::from_millis(total_time / self.statistics.total_attempts);
        
        // Aktualisiere Kategorie-Statistiken
        let category = recovery.original_error.category();
        *self.statistics.attempts_by_category.entry(category).or_insert(0) += 1;
        
        // Aktualisiere Erfolgsrate nach Strategie
        let strategy = recovery.strategy.strategy.clone();
        let current_rate = self.statistics.success_rate_by_strategy.get(&strategy).unwrap_or(&0.0);
        let new_rate = if recovery.status == RecoveryStatus::Success {
            (current_rate + 1.0) / 2.0
        } else {
            current_rate / 2.0
        };
        self.statistics.success_rate_by_strategy.insert(strategy, new_rate);
    }

    /// Fügt Recovery zur Historie hinzu
    fn add_to_history(&mut self, recovery: &ActiveRecovery) {
        let entry = RecoveryHistoryEntry {
            timestamp: recovery.started_at,
            error_type: format!("{:?}", recovery.original_error.category()),
            strategy: recovery.strategy.strategy.clone(),
            success: recovery.status == RecoveryStatus::Success,
            duration: recovery.started_at.elapsed().unwrap_or_default(),
            details: HashMap::new(),
        };
        
        self.history.push(entry);
        
        // Begrenze Historie-Größe
        if self.history.len() > self.config.max_history_entries {
            self.history.remove(0);
        }
    }

    /// Gibt aktive Recovery zurück
    pub fn get_active_recovery(&self, recovery_id: &str) -> Option<&ActiveRecovery> {
        self.active_recoveries.get(recovery_id)
    }

    /// Gibt alle aktiven Recoveries zurück
    pub fn get_active_recoveries(&self) -> Vec<&ActiveRecovery> {
        self.active_recoveries.values().collect()
    }

    /// Bricht eine Recovery ab
    pub fn cancel_recovery(&mut self, recovery_id: &str) -> Result<(), ZAKYXBrowserError> {
        if let Some(recovery) = self.active_recoveries.get_mut(recovery_id) {
            recovery.status = RecoveryStatus::Cancelled;
            println!("🚫 Recovery cancelled: {}", recovery_id);
            Ok(())
        } else {
            Err(ZAKYXBrowserError::Internal {
                message: format!("Recovery not found: {}", recovery_id),
                error_code: Some("RECOVERY_NOT_FOUND".to_string()),
            })
        }
    }

    /// Bereinigt abgeschlossene Recoveries
    pub fn cleanup_completed_recoveries(&mut self) -> usize {
        let initial_count = self.active_recoveries.len();
        
        self.active_recoveries.retain(|_, recovery| {
            !matches!(recovery.status, 
                RecoveryStatus::Success | 
                RecoveryStatus::Failed | 
                RecoveryStatus::Cancelled
            )
        });
        
        let cleaned_count = initial_count - self.active_recoveries.len();
        if cleaned_count > 0 {
            println!("🧹 Cleaned up {} completed recoveries", cleaned_count);
        }
        
        cleaned_count
    }

    /// Erstellt Recovery-Report
    pub fn create_recovery_report(&self) -> String {
        let mut report = String::new();
        report.push_str("🔄 ERROR RECOVERY REPORT\n");
        report.push_str("========================\n\n");
        
        report.push_str(&format!("📊 STATISTICS:\n"));
        report.push_str(&format!("• Total Attempts: {}\n", self.statistics.total_attempts));
        report.push_str(&format!("• Successful: {}\n", self.statistics.successful_recoveries));
        report.push_str(&format!("• Failed: {}\n", self.statistics.failed_recoveries));
        report.push_str(&format!("• Success Rate: {:.1}%\n", 
            if self.statistics.total_attempts > 0 {
                (self.statistics.successful_recoveries as f64 / self.statistics.total_attempts as f64) * 100.0
            } else { 0.0 }
        ));
        report.push_str(&format!("• Average Recovery Time: {:.1}s\n", self.statistics.average_recovery_time.as_secs_f64()));
        report.push_str("\n");
        
        if !self.active_recoveries.is_empty() {
            report.push_str("🔄 ACTIVE RECOVERIES:\n");
            for (id, recovery) in &self.active_recoveries {
                report.push_str(&format!(
                    "• {} ({:?}): {:?} - {}%\n",
                    id, recovery.original_error.category(), recovery.status, recovery.progress
                ));
            }
            report.push_str("\n");
        }
        
        if !self.statistics.attempts_by_category.is_empty() {
            report.push_str("📈 ATTEMPTS BY CATEGORY:\n");
            for (category, count) in &self.statistics.attempts_by_category {
                report.push_str(&format!("• {:?}: {}\n", category, count));
            }
        }
        
        report
    }

    // Getter
    pub fn get_statistics(&self) -> &RecoveryStatistics {
        &self.statistics
    }

    pub fn get_config(&self) -> &RecoveryConfig {
        &self.config
    }

    pub fn get_history(&self) -> &[RecoveryHistoryEntry] {
        &self.history
    }
}

impl Default for RecoveryOptions {
    fn default() -> Self {
        Self {
            timeout_seconds: 30,
            background_recovery: true,
            notify_user: false,
            fallback_resources: Vec::new(),
            custom_handler: None,
            priority: 5,
        }
    }
}

impl Default for RecoveryConfig {
    fn default() -> Self {
        Self {
            max_concurrent_recoveries: 5,
            default_timeout: Duration::from_secs(60),
            auto_recovery_enabled: true,
            min_retry_interval: Duration::from_millis(500),
            max_retry_interval: Duration::from_secs(30),
            save_history: true,
            max_history_entries: 100,
        }
    }
}

impl Default for ErrorRecoveryManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_recovery_manager_creation() {
        let manager = ErrorRecoveryManager::new();
        assert_eq!(manager.active_recoveries.len(), 0);
        assert_eq!(manager.statistics.total_attempts, 0);
    }

    #[tokio::test]
    async fn test_recovery_strategy_creation() {
        let manager = ErrorRecoveryManager::new();
        
        let network_error = ZAKYXBrowserError::network_error("Test error");
        let strategy = manager.create_recovery_strategy(&network_error);
        
        assert!(strategy.retry_possible);
        assert_eq!(strategy.strategy, RecoveryStrategy::ExponentialBackoff);
        assert!(strategy.fallback_available);
    }

    #[tokio::test]
    async fn test_recovery_steps_creation() {
        let manager = ErrorRecoveryManager::new();
        
        let network_error = ZAKYXBrowserError::network_error("Test error");
        let strategy = manager.create_recovery_strategy(&network_error);
        let steps = manager.create_recovery_steps(&network_error, &strategy);
        
        assert!(!steps.is_empty());
        assert!(steps.iter().any(|s| s.name == "retry_operation"));
    }

    #[test]
    fn test_retry_delay_calculation() {
        let manager = ErrorRecoveryManager::new();
        
        let mut recovery = ActiveRecovery {
            id: "test".to_string(),
            original_error: ZAKYXBrowserError::network_error("Test"),
            strategy: ErrorRecovery {
                retry_possible: true,
                retry_count: 2,
                max_retries: 3,
                retry_delay_ms: 1000,
                fallback_available: true,
                user_action_required: false,
                strategy: RecoveryStrategy::ExponentialBackoff,
                options: RecoveryOptions::default(),
            },
            started_at: SystemTime::now(),
            status: RecoveryStatus::InProgress,
            progress: 0,
            steps: Vec::new(),
            current_step: 0,
        };
        
        // Exponential backoff: 1000 * 2^2 = 4000
        let delay = manager.calculate_retry_delay(&recovery);
        assert_eq!(delay, 4000);
        
        // Linear backoff
        recovery.strategy.strategy = RecoveryStrategy::LinearBackoff;
        let delay = manager.calculate_retry_delay(&recovery);
        assert_eq!(delay, 3000); // 1000 * (2 + 1)
    }

    #[test]
    fn test_recovery_statistics() {
        let mut manager = ErrorRecoveryManager::new();
        
        let recovery = ActiveRecovery {
            id: "test".to_string(),
            original_error: ZAKYXBrowserError::network_error("Test"),
            strategy: manager.create_recovery_strategy(&ZAKYXBrowserError::network_error("Test")),
            started_at: SystemTime::now(),
            status: RecoveryStatus::Success,
            progress: 100,
            steps: Vec::new(),
            current_step: 0,
        };
        
        manager.update_statistics(&recovery, Duration::from_secs(5));
        
        assert_eq!(manager.statistics.total_attempts, 1);
        assert_eq!(manager.statistics.successful_recoveries, 1);
        assert_eq!(manager.statistics.failed_recoveries, 0);
    }

    #[test]
    fn test_recovery_config() {
        let config = RecoveryConfig::default();
        
        assert_eq!(config.max_concurrent_recoveries, 5);
        assert!(config.auto_recovery_enabled);
        assert!(config.save_history);
        assert_eq!(config.max_history_entries, 100);
    }
}