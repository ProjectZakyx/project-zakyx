// ⚡ WebView2 Performance Management für ZAKYX Browser
// Performance-Monitoring und Optimierung für WebView2

use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant, SystemTime};
use serde::{Deserialize, Serialize};

/// Performance-Metriken für WebView2
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    /// Navigation-Performance
    pub navigation: NavigationMetrics,
    
    /// JavaScript-Performance
    pub javascript: JavaScriptMetrics,
    
    /// Memory-Nutzung
    pub memory: MemoryMetrics,
    
    /// Rendering-Performance
    pub rendering: RenderingMetrics,
    
    /// Netzwerk-Performance
    pub network: NetworkMetrics,
    
    /// System-Ressourcen
    pub system: SystemMetrics,
}

/// Navigation-Performance-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NavigationMetrics {
    /// Durchschnittliche Ladezeit in ms
    pub average_load_time_ms: f64,
    
    /// Minimale Ladezeit in ms
    pub min_load_time_ms: u64,
    
    /// Maximale Ladezeit in ms
    pub max_load_time_ms: u64,
    
    /// Anzahl erfolgreicher Navigationen
    pub successful_navigations: u64,
    
    /// Anzahl fehlgeschlagener Navigationen
    pub failed_navigations: u64,
    
    /// Erfolgsrate in Prozent
    pub success_rate: f64,
    
    /// Letzte Navigationen (für Trend-Analyse)
    pub recent_load_times: VecDeque<u64>,
}

/// JavaScript-Performance-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JavaScriptMetrics {
    /// Durchschnittliche Ausführungszeit in ms
    pub average_execution_time_ms: f64,
    
    /// Minimale Ausführungszeit in ms
    pub min_execution_time_ms: u64,
    
    /// Maximale Ausführungszeit in ms
    pub max_execution_time_ms: u64,
    
    /// Anzahl erfolgreicher Ausführungen
    pub successful_executions: u64,
    
    /// Anzahl fehlgeschlagener Ausführungen
    pub failed_executions: u64,
    
    /// Erfolgsrate in Prozent
    pub success_rate: f64,
    
    /// Letzte Ausführungszeiten
    pub recent_execution_times: VecDeque<u64>,
}

/// Memory-Nutzung-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryMetrics {
    /// Aktuelle Memory-Nutzung in MB
    pub current_usage_mb: u64,
    
    /// Maximale Memory-Nutzung in MB
    pub peak_usage_mb: u64,
    
    /// Durchschnittliche Memory-Nutzung in MB
    pub average_usage_mb: f64,
    
    /// Memory-Nutzung-Historie
    pub usage_history: VecDeque<MemorySnapshot>,
    
    /// Anzahl Garbage-Collections
    pub gc_count: u64,
    
    /// Zeit für Garbage-Collections in ms
    pub gc_time_ms: u64,
}

/// Memory-Snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    /// Zeitstempel
    pub timestamp: SystemTime,
    
    /// Memory-Nutzung in MB
    pub usage_mb: u64,
    
    /// Heap-Größe in MB
    pub heap_size_mb: u64,
    
    /// Verwendeter Heap in MB
    pub heap_used_mb: u64,
}

/// Rendering-Performance-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenderingMetrics {
    /// Frames per Second
    pub fps: f64,
    
    /// Durchschnittliche Frame-Zeit in ms
    pub average_frame_time_ms: f64,
    
    /// Dropped Frames
    pub dropped_frames: u64,
    
    /// GPU-Nutzung in Prozent
    pub gpu_usage_percent: f64,
    
    /// Rendering-Modus
    pub rendering_mode: RenderingMode,
}

/// Rendering-Modus
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RenderingMode {
    /// Hardware-beschleunigt
    Hardware,
    
    /// Software-Rendering
    Software,
    
    /// Hybrid
    Hybrid,
    
    /// Unbekannt
    Unknown,
}

/// Netzwerk-Performance-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMetrics {
    /// Durchschnittliche Download-Geschwindigkeit in KB/s
    pub average_download_speed_kbps: f64,
    
    /// Durchschnittliche Upload-Geschwindigkeit in KB/s
    pub average_upload_speed_kbps: f64,
    
    /// Durchschnittliche Latenz in ms
    pub average_latency_ms: f64,
    
    /// Anzahl Netzwerk-Requests
    pub total_requests: u64,
    
    /// Anzahl fehlgeschlagener Requests
    pub failed_requests: u64,
    
    /// Cache-Hit-Rate in Prozent
    pub cache_hit_rate: f64,
}

/// System-Ressourcen-Metriken
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMetrics {
    /// CPU-Nutzung in Prozent
    pub cpu_usage_percent: f64,
    
    /// Verfügbarer RAM in MB
    pub available_ram_mb: u64,
    
    /// Festplatten-I/O in MB/s
    pub disk_io_mbps: f64,
    
    /// Anzahl Threads
    pub thread_count: u32,
    
    /// Handle-Count
    pub handle_count: u32,
}

/// Performance-Monitor für WebView2
#[derive(Debug)]
pub struct WebView2PerformanceMonitor {
    /// Aktuelle Metriken
    metrics: PerformanceMetrics,
    
    /// Performance-Historie
    history: VecDeque<PerformanceSnapshot>,
    
    /// Monitoring-Start-Zeit
    start_time: Instant,
    
    /// Letzte Aktualisierung
    last_update: Instant,
    
    /// Monitoring-Intervall
    update_interval: Duration,
    
    /// Performance-Warnungen
    warnings: Vec<PerformanceWarning>,
    
    /// Optimierungsvorschläge
    optimization_suggestions: Vec<OptimizationSuggestion>,
}

/// Performance-Snapshot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceSnapshot {
    /// Zeitstempel
    pub timestamp: SystemTime,
    
    /// Metriken zum Zeitpunkt
    pub metrics: PerformanceMetrics,
    
    /// Zusätzliche Informationen
    pub additional_info: HashMap<String, String>,
}

/// Performance-Warnung
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceWarning {
    /// Warnung-Typ
    pub warning_type: WarningType,
    
    /// Beschreibung
    pub description: String,
    
    /// Schweregrad
    pub severity: WarningSeverity,
    
    /// Zeitstempel
    pub timestamp: SystemTime,
    
    /// Betroffene Metriken
    pub affected_metrics: Vec<String>,
}

/// Warnung-Typ
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum WarningType {
    /// Hohe Memory-Nutzung
    HighMemoryUsage,
    
    /// Langsame Navigation
    SlowNavigation,
    
    /// Langsame JavaScript-Ausführung
    SlowJavaScript,
    
    /// Niedrige FPS
    LowFPS,
    
    /// Hohe CPU-Nutzung
    HighCPUUsage,
    
    /// Netzwerk-Probleme
    NetworkIssues,
    
    /// Häufige Garbage-Collections
    FrequentGC,
}

/// Warnung-Schweregrad
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum WarningSeverity {
    /// Information
    Info,
    
    /// Warnung
    Warning,
    
    /// Fehler
    Error,
    
    /// Kritisch
    Critical,
}

/// Optimierungsvorschlag
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    /// Titel
    pub title: String,
    
    /// Beschreibung
    pub description: String,
    
    /// Kategorie
    pub category: OptimizationCategory,
    
    /// Erwartete Verbesserung
    pub expected_improvement: String,
    
    /// Implementierungsaufwand
    pub implementation_effort: ImplementationEffort,
    
    /// Priorität
    pub priority: u8, // 1-10
}

/// Optimierungskategorie
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OptimizationCategory {
    /// Memory-Optimierung
    Memory,
    
    /// Performance-Optimierung
    Performance,
    
    /// Netzwerk-Optimierung
    Network,
    
    /// Rendering-Optimierung
    Rendering,
    
    /// JavaScript-Optimierung
    JavaScript,
    
    /// System-Optimierung
    System,
}

/// Implementierungsaufwand
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImplementationEffort {
    /// Niedrig
    Low,
    
    /// Mittel
    Medium,
    
    /// Hoch
    High,
}

impl WebView2PerformanceMonitor {
    /// Erstellt einen neuen Performance-Monitor
    pub fn new() -> Self {
        Self {
            metrics: PerformanceMetrics::default(),
            history: VecDeque::new(),
            start_time: Instant::now(),
            last_update: Instant::now(),
            update_interval: Duration::from_secs(1),
            warnings: Vec::new(),
            optimization_suggestions: Vec::new(),
        }
    }

    /// Erstellt einen Monitor mit angepasstem Update-Intervall
    pub fn with_update_interval(interval: Duration) -> Self {
        let mut monitor = Self::new();
        monitor.update_interval = interval;
        monitor
    }

    /// Startet das Performance-Monitoring
    pub fn start_monitoring(&mut self) {
        println!("⚡ Starting WebView2 performance monitoring...");
        self.start_time = Instant::now();
        self.last_update = Instant::now();
        self.collect_initial_metrics();
    }

    /// Aktualisiert die Performance-Metriken
    pub fn update_metrics(&mut self) {
        let now = Instant::now();
        if now.duration_since(self.last_update) >= self.update_interval {
            self.collect_metrics();
            self.analyze_performance();
            self.generate_suggestions();
            self.last_update = now;
        }
    }

    /// Sammelt initiale Metriken
    fn collect_initial_metrics(&mut self) {
        println!("📊 Collecting initial performance metrics...");
        
        // Simuliere initiale Metriken-Sammlung
        self.metrics = PerformanceMetrics::default();
    }

    /// Sammelt aktuelle Metriken
    fn collect_metrics(&mut self) {
        // Navigation-Metriken aktualisieren
        self.update_navigation_metrics();
        
        // JavaScript-Metriken aktualisieren
        self.update_javascript_metrics();
        
        // Memory-Metriken aktualisieren
        self.update_memory_metrics();
        
        // Rendering-Metriken aktualisieren
        self.update_rendering_metrics();
        
        // Netzwerk-Metriken aktualisieren
        self.update_network_metrics();
        
        // System-Metriken aktualisieren
        self.update_system_metrics();
        
        // Snapshot erstellen
        self.create_snapshot();
    }

    /// Aktualisiert Navigation-Metriken
    fn update_navigation_metrics(&mut self) {
        // Simuliere Navigation-Metriken-Update
        let nav = &mut self.metrics.navigation;
        
        // Beispiel-Daten
        nav.recent_load_times.push_back(150);
        if nav.recent_load_times.len() > 10 {
            nav.recent_load_times.pop_front();
        }
        
        // Durchschnitt berechnen
        if !nav.recent_load_times.is_empty() {
            nav.average_load_time_ms = nav.recent_load_times.iter().sum::<u64>() as f64 / nav.recent_load_times.len() as f64;
            nav.min_load_time_ms = *nav.recent_load_times.iter().min().unwrap();
            nav.max_load_time_ms = *nav.recent_load_times.iter().max().unwrap();
        }
    }

    /// Aktualisiert JavaScript-Metriken
    fn update_javascript_metrics(&mut self) {
        // Simuliere JavaScript-Metriken-Update
        let js = &mut self.metrics.javascript;
        
        // Beispiel-Daten
        js.recent_execution_times.push_back(25);
        if js.recent_execution_times.len() > 20 {
            js.recent_execution_times.pop_front();
        }
        
        // Durchschnitt berechnen
        if !js.recent_execution_times.is_empty() {
            js.average_execution_time_ms = js.recent_execution_times.iter().sum::<u64>() as f64 / js.recent_execution_times.len() as f64;
            js.min_execution_time_ms = *js.recent_execution_times.iter().min().unwrap();
            js.max_execution_time_ms = *js.recent_execution_times.iter().max().unwrap();
        }
    }

    /// Aktualisiert Memory-Metriken
    fn update_memory_metrics(&mut self) {
        // Simuliere Memory-Metriken-Update
        let mem = &mut self.metrics.memory;
        
        // Beispiel-Memory-Snapshot
        let snapshot = MemorySnapshot {
            timestamp: SystemTime::now(),
            usage_mb: 128,
            heap_size_mb: 64,
            heap_used_mb: 48,
        };
        
        mem.usage_history.push_back(snapshot.clone());
        if mem.usage_history.len() > 60 { // Letzte 60 Snapshots
            mem.usage_history.pop_front();
        }
        
        // Aktuelle Werte aktualisieren
        mem.current_usage_mb = snapshot.usage_mb;
        mem.peak_usage_mb = mem.peak_usage_mb.max(snapshot.usage_mb);
        
        // Durchschnitt berechnen
        if !mem.usage_history.is_empty() {
            mem.average_usage_mb = mem.usage_history.iter().map(|s| s.usage_mb).sum::<u64>() as f64 / mem.usage_history.len() as f64;
        }
    }

    /// Aktualisiert Rendering-Metriken
    fn update_rendering_metrics(&mut self) {
        // Simuliere Rendering-Metriken-Update
        let render = &mut self.metrics.rendering;
        
        render.fps = 60.0;
        render.average_frame_time_ms = 16.67;
        render.dropped_frames = 0;
        render.gpu_usage_percent = 25.0;
        render.rendering_mode = RenderingMode::Hardware;
    }

    /// Aktualisiert Netzwerk-Metriken
    fn update_network_metrics(&mut self) {
        // Simuliere Netzwerk-Metriken-Update
        let net = &mut self.metrics.network;
        
        net.average_download_speed_kbps = 1024.0;
        net.average_upload_speed_kbps = 256.0;
        net.average_latency_ms = 50.0;
        net.total_requests += 1;
        net.cache_hit_rate = 85.0;
    }

    /// Aktualisiert System-Metriken
    fn update_system_metrics(&mut self) {
        // Simuliere System-Metriken-Update
        let sys = &mut self.metrics.system;
        
        sys.cpu_usage_percent = 15.0;
        sys.available_ram_mb = 8192;
        sys.disk_io_mbps = 50.0;
        sys.thread_count = 12;
        sys.handle_count = 256;
    }

    /// Erstellt einen Performance-Snapshot
    fn create_snapshot(&mut self) {
        let snapshot = PerformanceSnapshot {
            timestamp: SystemTime::now(),
            metrics: self.metrics.clone(),
            additional_info: HashMap::new(),
        };
        
        self.history.push_back(snapshot);
        
        // Historie begrenzen (letzte 100 Snapshots)
        if self.history.len() > 100 {
            self.history.pop_front();
        }
    }

    /// Analysiert Performance und erstellt Warnungen
    fn analyze_performance(&mut self) {
        self.warnings.clear();
        
        // Memory-Analyse
        if self.metrics.memory.current_usage_mb > 512 {
            self.warnings.push(PerformanceWarning {
                warning_type: WarningType::HighMemoryUsage,
                description: format!("High memory usage: {}MB", self.metrics.memory.current_usage_mb),
                severity: WarningSeverity::Warning,
                timestamp: SystemTime::now(),
                affected_metrics: vec!["memory.current_usage_mb".to_string()],
            });
        }
        
        // Navigation-Analyse
        if self.metrics.navigation.average_load_time_ms > 3000.0 {
            self.warnings.push(PerformanceWarning {
                warning_type: WarningType::SlowNavigation,
                description: format!("Slow navigation: {:.1}ms average", self.metrics.navigation.average_load_time_ms),
                severity: WarningSeverity::Warning,
                timestamp: SystemTime::now(),
                affected_metrics: vec!["navigation.average_load_time_ms".to_string()],
            });
        }
        
        // JavaScript-Analyse
        if self.metrics.javascript.average_execution_time_ms > 100.0 {
            self.warnings.push(PerformanceWarning {
                warning_type: WarningType::SlowJavaScript,
                description: format!("Slow JavaScript execution: {:.1}ms average", self.metrics.javascript.average_execution_time_ms),
                severity: WarningSeverity::Warning,
                timestamp: SystemTime::now(),
                affected_metrics: vec!["javascript.average_execution_time_ms".to_string()],
            });
        }
        
        // FPS-Analyse
        if self.metrics.rendering.fps < 30.0 {
            self.warnings.push(PerformanceWarning {
                warning_type: WarningType::LowFPS,
                description: format!("Low FPS: {:.1}", self.metrics.rendering.fps),
                severity: WarningSeverity::Error,
                timestamp: SystemTime::now(),
                affected_metrics: vec!["rendering.fps".to_string()],
            });
        }
        
        // CPU-Analyse
        if self.metrics.system.cpu_usage_percent > 80.0 {
            self.warnings.push(PerformanceWarning {
                warning_type: WarningType::HighCPUUsage,
                description: format!("High CPU usage: {:.1}%", self.metrics.system.cpu_usage_percent),
                severity: WarningSeverity::Warning,
                timestamp: SystemTime::now(),
                affected_metrics: vec!["system.cpu_usage_percent".to_string()],
            });
        }
    }

    /// Generiert Optimierungsvorschläge
    fn generate_suggestions(&mut self) {
        self.optimization_suggestions.clear();
        
        // Memory-Optimierung
        if self.metrics.memory.current_usage_mb > 256 {
            self.optimization_suggestions.push(OptimizationSuggestion {
                title: "Memory-Optimierung".to_string(),
                description: "Reduziere Memory-Nutzung durch Cache-Bereinigung und Garbage Collection".to_string(),
                category: OptimizationCategory::Memory,
                expected_improvement: "20-30% weniger Memory-Verbrauch".to_string(),
                implementation_effort: ImplementationEffort::Low,
                priority: 7,
            });
        }
        
        // JavaScript-Optimierung
        if self.metrics.javascript.average_execution_time_ms > 50.0 {
            self.optimization_suggestions.push(OptimizationSuggestion {
                title: "JavaScript-Optimierung".to_string(),
                description: "Optimiere JavaScript-Code und verwende asynchrone Ausführung".to_string(),
                category: OptimizationCategory::JavaScript,
                expected_improvement: "30-50% schnellere Script-Ausführung".to_string(),
                implementation_effort: ImplementationEffort::Medium,
                priority: 8,
            });
        }
        
        // Rendering-Optimierung
        if self.metrics.rendering.rendering_mode == RenderingMode::Software {
            self.optimization_suggestions.push(OptimizationSuggestion {
                title: "Hardware-Beschleunigung aktivieren".to_string(),
                description: "Aktiviere GPU-Rendering für bessere Performance".to_string(),
                category: OptimizationCategory::Rendering,
                expected_improvement: "2-3x bessere Rendering-Performance".to_string(),
                implementation_effort: ImplementationEffort::Low,
                priority: 9,
            });
        }
        
        // Netzwerk-Optimierung
        if self.metrics.network.cache_hit_rate < 70.0 {
            self.optimization_suggestions.push(OptimizationSuggestion {
                title: "Cache-Optimierung".to_string(),
                description: "Verbessere Cache-Strategien für häufig verwendete Ressourcen".to_string(),
                category: OptimizationCategory::Network,
                expected_improvement: "20-40% weniger Netzwerk-Traffic".to_string(),
                implementation_effort: ImplementationEffort::Medium,
                priority: 6,
            });
        }
    }

    /// Gibt aktuelle Performance-Metriken zurück
    pub fn get_metrics(&self) -> &PerformanceMetrics {
        &self.metrics
    }

    /// Gibt Performance-Historie zurück
    pub fn get_history(&self) -> &VecDeque<PerformanceSnapshot> {
        &self.history
    }

    /// Gibt aktuelle Warnungen zurück
    pub fn get_warnings(&self) -> &[PerformanceWarning] {
        &self.warnings
    }

    /// Gibt Optimierungsvorschläge zurück
    pub fn get_optimization_suggestions(&self) -> &[OptimizationSuggestion] {
        &self.optimization_suggestions
    }

    /// Gibt einen detaillierten Performance-Report zurück
    pub fn generate_performance_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("⚡ WEBVIEW2 PERFORMANCE REPORT\n");
        report.push_str("==============================\n\n");
        
        // Laufzeit
        let uptime = self.start_time.elapsed();
        report.push_str(&format!("📊 MONITORING DURATION: {:.1} minutes\n\n", uptime.as_secs_f64() / 60.0));
        
        // Navigation-Metriken
        report.push_str("🌐 NAVIGATION PERFORMANCE:\n");
        report.push_str(&format!("• Average Load Time: {:.1}ms\n", self.metrics.navigation.average_load_time_ms));
        report.push_str(&format!("• Min/Max Load Time: {}ms / {}ms\n", self.metrics.navigation.min_load_time_ms, self.metrics.navigation.max_load_time_ms));
        report.push_str(&format!("• Success Rate: {:.1}%\n", self.metrics.navigation.success_rate));
        report.push_str(&format!("• Total Navigations: {}\n\n", self.metrics.navigation.successful_navigations + self.metrics.navigation.failed_navigations));
        
        // Memory-Metriken
        report.push_str("💾 MEMORY USAGE:\n");
        report.push_str(&format!("• Current Usage: {}MB\n", self.metrics.memory.current_usage_mb));
        report.push_str(&format!("• Peak Usage: {}MB\n", self.metrics.memory.peak_usage_mb));
        report.push_str(&format!("• Average Usage: {:.1}MB\n", self.metrics.memory.average_usage_mb));
        report.push_str(&format!("• GC Count: {}\n\n", self.metrics.memory.gc_count));
        
        // JavaScript-Metriken
        report.push_str("💉 JAVASCRIPT PERFORMANCE:\n");
        report.push_str(&format!("• Average Execution Time: {:.1}ms\n", self.metrics.javascript.average_execution_time_ms));
        report.push_str(&format!("• Min/Max Execution Time: {}ms / {}ms\n", self.metrics.javascript.min_execution_time_ms, self.metrics.javascript.max_execution_time_ms));
        report.push_str(&format!("• Success Rate: {:.1}%\n", self.metrics.javascript.success_rate));
        report.push_str(&format!("• Total Executions: {}\n\n", self.metrics.javascript.successful_executions + self.metrics.javascript.failed_executions));
        
        // Rendering-Metriken
        report.push_str("🎨 RENDERING PERFORMANCE:\n");
        report.push_str(&format!("• FPS: {:.1}\n", self.metrics.rendering.fps));
        report.push_str(&format!("• Average Frame Time: {:.1}ms\n", self.metrics.rendering.average_frame_time_ms));
        report.push_str(&format!("• Dropped Frames: {}\n", self.metrics.rendering.dropped_frames));
        report.push_str(&format!("• GPU Usage: {:.1}%\n", self.metrics.rendering.gpu_usage_percent));
        report.push_str(&format!("• Rendering Mode: {:?}\n\n", self.metrics.rendering.rendering_mode));
        
        // System-Metriken
        report.push_str("🖥️ SYSTEM RESOURCES:\n");
        report.push_str(&format!("• CPU Usage: {:.1}%\n", self.metrics.system.cpu_usage_percent));
        report.push_str(&format!("• Available RAM: {}MB\n", self.metrics.system.available_ram_mb));
        report.push_str(&format!("• Disk I/O: {:.1}MB/s\n", self.metrics.system.disk_io_mbps));
        report.push_str(&format!("• Thread Count: {}\n", self.metrics.system.thread_count));
        report.push_str(&format!("• Handle Count: {}\n\n", self.metrics.system.handle_count));
        
        // Warnungen
        if !self.warnings.is_empty() {
            report.push_str("⚠️ PERFORMANCE WARNINGS:\n");
            for warning in &self.warnings {
                report.push_str(&format!("• [{:?}] {}\n", warning.severity, warning.description));
            }
            report.push('\n');
        }
        
        // Optimierungsvorschläge
        if !self.optimization_suggestions.is_empty() {
            report.push_str("💡 OPTIMIZATION SUGGESTIONS:\n");
            for suggestion in &self.optimization_suggestions {
                report.push_str(&format!("• [Priority {}] {}: {}\n", suggestion.priority, suggestion.title, suggestion.expected_improvement));
            }
        }
        
        report
    }

    /// Exportiert Metriken als JSON
    pub fn export_metrics_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(&self.metrics)
    }

    /// Bereinigt alte Historie-Einträge
    pub fn cleanup_history(&mut self, max_age: Duration) {
        let cutoff = SystemTime::now() - max_age;
        
        self.history.retain(|snapshot| {
            snapshot.timestamp > cutoff
        });
        
        println!("🧹 Cleaned up performance history, {} snapshots remaining", self.history.len());
    }
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            navigation: NavigationMetrics::default(),
            javascript: JavaScriptMetrics::default(),
            memory: MemoryMetrics::default(),
            rendering: RenderingMetrics::default(),
            network: NetworkMetrics::default(),
            system: SystemMetrics::default(),
        }
    }
}

impl Default for NavigationMetrics {
    fn default() -> Self {
        Self {
            average_load_time_ms: 0.0,
            min_load_time_ms: 0,
            max_load_time_ms: 0,
            successful_navigations: 0,
            failed_navigations: 0,
            success_rate: 100.0,
            recent_load_times: VecDeque::new(),
        }
    }
}

impl Default for JavaScriptMetrics {
    fn default() -> Self {
        Self {
            average_execution_time_ms: 0.0,
            min_execution_time_ms: 0,
            max_execution_time_ms: 0,
            successful_executions: 0,
            failed_executions: 0,
            success_rate: 100.0,
            recent_execution_times: VecDeque::new(),
        }
    }
}

impl Default for MemoryMetrics {
    fn default() -> Self {
        Self {
            current_usage_mb: 0,
            peak_usage_mb: 0,
            average_usage_mb: 0.0,
            usage_history: VecDeque::new(),
            gc_count: 0,
            gc_time_ms: 0,
        }
    }
}

impl Default for RenderingMetrics {
    fn default() -> Self {
        Self {
            fps: 60.0,
            average_frame_time_ms: 16.67,
            dropped_frames: 0,
            gpu_usage_percent: 0.0,
            rendering_mode: RenderingMode::Unknown,
        }
    }
}

impl Default for NetworkMetrics {
    fn default() -> Self {
        Self {
            average_download_speed_kbps: 0.0,
            average_upload_speed_kbps: 0.0,
            average_latency_ms: 0.0,
            total_requests: 0,
            failed_requests: 0,
            cache_hit_rate: 0.0,
        }
    }
}

impl Default for SystemMetrics {
    fn default() -> Self {
        Self {
            cpu_usage_percent: 0.0,
            available_ram_mb: 0,
            disk_io_mbps: 0.0,
            thread_count: 0,
            handle_count: 0,
        }
    }
}

impl Default for WebView2PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_performance_monitor_creation() {
        let monitor = WebView2PerformanceMonitor::new();
        assert!(monitor.history.is_empty());
        assert!(monitor.warnings.is_empty());
        assert!(monitor.optimization_suggestions.is_empty());
    }

    #[test]
    fn test_performance_metrics_default() {
        let metrics = PerformanceMetrics::default();
        assert_eq!(metrics.navigation.successful_navigations, 0);
        assert_eq!(metrics.javascript.successful_executions, 0);
        assert_eq!(metrics.memory.current_usage_mb, 0);
        assert_eq!(metrics.rendering.fps, 60.0);
    }

    #[test]
    fn test_warning_severity_ordering() {
        assert!(WarningSeverity::Critical > WarningSeverity::Error);
        assert!(WarningSeverity::Error > WarningSeverity::Warning);
        assert!(WarningSeverity::Warning > WarningSeverity::Info);
    }

    #[test]
    fn test_implementation_effort_ordering() {
        assert!(ImplementationEffort::High > ImplementationEffort::Medium);
        assert!(ImplementationEffort::Medium > ImplementationEffort::Low);
    }

    #[test]
    fn test_rendering_mode_equality() {
        assert_eq!(RenderingMode::Hardware, RenderingMode::Hardware);
        assert_ne!(RenderingMode::Hardware, RenderingMode::Software);
    }

    #[test]
    fn test_memory_snapshot() {
        let snapshot = MemorySnapshot {
            timestamp: SystemTime::now(),
            usage_mb: 128,
            heap_size_mb: 64,
            heap_used_mb: 48,
        };
        
        assert_eq!(snapshot.usage_mb, 128);
        assert_eq!(snapshot.heap_size_mb, 64);
        assert_eq!(snapshot.heap_used_mb, 48);
    }

    #[test]
    fn test_optimization_suggestion() {
        let suggestion = OptimizationSuggestion {
            title: "Test Optimization".to_string(),
            description: "Test Description".to_string(),
            category: OptimizationCategory::Performance,
            expected_improvement: "50% better".to_string(),
            implementation_effort: ImplementationEffort::Low,
            priority: 8,
        };
        
        assert_eq!(suggestion.priority, 8);
        assert_eq!(suggestion.category, OptimizationCategory::Performance);
        assert_eq!(suggestion.implementation_effort, ImplementationEffort::Low);
    }

    #[test]
    fn test_performance_report_generation() {
        let monitor = WebView2PerformanceMonitor::new();
        let report = monitor.generate_performance_report();
        
        assert!(report.contains("WEBVIEW2 PERFORMANCE REPORT"));
        assert!(report.contains("NAVIGATION PERFORMANCE"));
        assert!(report.contains("MEMORY USAGE"));
        assert!(report.contains("JAVASCRIPT PERFORMANCE"));
    }

    #[test]
    fn test_metrics_json_export() {
        let metrics = PerformanceMetrics::default();
        let json_result = serde_json::to_string_pretty(&metrics);
        
        assert!(json_result.is_ok());
        let json = json_result.unwrap();
        assert!(json.contains("navigation"));
        assert!(json.contains("javascript"));
        assert!(json.contains("memory"));
    }
}