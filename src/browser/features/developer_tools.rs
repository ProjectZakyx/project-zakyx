// 🔧 Developer Tools für ZAKYX Browser
// Console-Logs, Network-Monitoring und Debug-Tools

use anyhow::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsoleLogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub message: String,
    pub source: String,
    pub line: Option<u32>,
    pub column: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogLevel {
    Log,
    Info,
    Warn,
    Error,
    Debug,
    Trace,
}

impl LogLevel {
    pub fn to_string(&self) -> String {
        match self {
            LogLevel::Log => "📝 LOG".to_string(),
            LogLevel::Info => "ℹ️ INFO".to_string(),
            LogLevel::Warn => "⚠️ WARN".to_string(),
            LogLevel::Error => "❌ ERROR".to_string(),
            LogLevel::Debug => "🐛 DEBUG".to_string(),
            LogLevel::Trace => "🔍 TRACE".to_string(),
        }
    }

    pub fn color(&self) -> &str {
        match self {
            LogLevel::Log => "white",
            LogLevel::Info => "cyan",
            LogLevel::Warn => "yellow",
            LogLevel::Error => "red",
            LogLevel::Debug => "green",
            LogLevel::Trace => "gray",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkLogEntry {
    pub timestamp: DateTime<Utc>,
    pub method: String,
    pub url: String,
    pub status_code: Option<u16>,
    pub response_time: Option<u64>, // milliseconds
    pub request_size: Option<u64>,
    pub response_size: Option<u64>,
    pub headers: HashMap<String, String>,
    pub error: Option<String>,
}

impl NetworkLogEntry {
    pub fn new(method: String, url: String) -> Self {
        Self {
            timestamp: Utc::now(),
            method,
            url,
            status_code: None,
            response_time: None,
            request_size: None,
            response_size: None,
            headers: HashMap::new(),
            error: None,
        }
    }

    pub fn with_response(mut self, status_code: u16, response_time: u64) -> Self {
        self.status_code = Some(status_code);
        self.response_time = Some(response_time);
        self
    }

    pub fn with_error(mut self, error: String) -> Self {
        self.error = Some(error);
        self
    }

    pub fn with_sizes(mut self, request_size: u64, response_size: u64) -> Self {
        self.request_size = Some(request_size);
        self.response_size = Some(response_size);
        self
    }

    pub fn status_emoji(&self) -> &str {
        match self.status_code {
            Some(code) => match code {
                200..=299 => "✅",
                300..=399 => "🔄",
                400..=499 => "⚠️",
                500..=599 => "❌",
                _ => "❓",
            },
            None => if self.error.is_some() { "❌" } else { "⏳" },
        }
    }

    pub fn format_response_time(&self) -> String {
        match self.response_time {
            Some(time) => format!("{}ms", time),
            None => "--".to_string(),
        }
    }

    pub fn format_size(&self, size_opt: Option<u64>) -> String {
        match size_opt {
            Some(size) => format_bytes(size),
            None => "--".to_string(),
        }
    }
}

pub struct DeveloperTools {
    console_logs: VecDeque<ConsoleLogEntry>,
    network_logs: VecDeque<NetworkLogEntry>,
    enabled: bool,
    max_console_entries: usize,
    max_network_entries: usize,
    filters: LogFilters,
    performance_marks: HashMap<String, DateTime<Utc>>,
    element_inspector_enabled: bool,
}

#[derive(Debug, Clone)]
pub struct LogFilters {
    pub console_levels: Vec<LogLevel>,
    pub network_methods: Vec<String>,
    pub min_response_time: Option<u64>,
    pub status_codes: Vec<u16>,
}

impl Default for LogFilters {
    fn default() -> Self {
        Self {
            console_levels: vec![
                LogLevel::Log,
                LogLevel::Info,
                LogLevel::Warn,
                LogLevel::Error,
                LogLevel::Debug,
            ],
            network_methods: vec![
                "GET".to_string(),
                "POST".to_string(),
                "PUT".to_string(),
                "DELETE".to_string(),
                "PATCH".to_string(),
            ],
            min_response_time: None,
            status_codes: Vec::new(), // Empty = show all
        }
    }
}

impl DeveloperTools {
    pub fn new() -> Self {
        Self {
            console_logs: VecDeque::new(),
            network_logs: VecDeque::new(),
            enabled: false,
            max_console_entries: 1000,
            max_network_entries: 500,
            filters: LogFilters::default(),
            performance_marks: HashMap::new(),
            element_inspector_enabled: false,
        }
    }

    /// Toggle Developer Tools
    pub fn toggle(&mut self) -> bool {
        self.enabled = !self.enabled;
        
        if self.enabled {
            println!("🔧 Developer Tools: Enabled");
            self.log_console_internal(LogLevel::Info, "Developer Tools activated".to_string(), "DevTools".to_string());
        } else {
            println!("🔧 Developer Tools: Disabled");
        }
        
        self.enabled
    }

    /// Console-Logging
    pub fn log_console(&mut self, level: LogLevel, message: &str, source: &str) {
        if self.enabled {
            self.log_console_internal(level, message.to_string(), source.to_string());
        }
    }

    fn log_console_internal(&mut self, level: LogLevel, message: String, source: String) {
        let entry = ConsoleLogEntry {
            timestamp: Utc::now(),
            level,
            message,
            source,
            line: None,
            column: None,
        };

        self.console_logs.push_back(entry);

        // Begrenze Anzahl der Einträge
        while self.console_logs.len() > self.max_console_entries {
            self.console_logs.pop_front();
        }
    }

    /// Network-Logging
    pub fn log_network_request(&mut self, method: &str, url: &str) -> String {
        if !self.enabled {
            return String::new();
        }

        let request_id = format!("req_{}", Utc::now().timestamp_millis());
        let entry = NetworkLogEntry::new(method.to_string(), url.to_string());
        
        self.network_logs.push_back(entry);
        
        // Begrenze Anzahl der Einträge
        while self.network_logs.len() > self.max_network_entries {
            self.network_logs.pop_front();
        }

        println!("🌐 Network: {} {}", method, url);
        request_id
    }

    /// Network-Response protokollieren
    pub fn log_network_response(&mut self, url: &str, status_code: u16, response_time: u64) {
        if !self.enabled {
            return;
        }

        // Finde den entsprechenden Request
        if let Some(entry) = self.network_logs.iter_mut().rev().find(|e| e.url == url && e.status_code.is_none()) {
            entry.status_code = Some(status_code);
            entry.response_time = Some(response_time);
        }

        println!("📡 Response: {} {} ({}ms)", status_code, url, response_time);
    }

    /// Network-Error protokollieren
    pub fn log_network_error(&mut self, url: &str, error: &str) {
        if !self.enabled {
            return;
        }

        if let Some(entry) = self.network_logs.iter_mut().rev().find(|e| e.url == url && e.status_code.is_none()) {
            entry.error = Some(error.to_string());
        }

        println!("❌ Network Error: {} - {}", url, error);
    }

    /// Performance-Mark setzen
    pub fn mark_performance(&mut self, mark_name: &str) {
        if self.enabled {
            self.performance_marks.insert(mark_name.to_string(), Utc::now());
            println!("⏱️ Performance Mark: {}", mark_name);
        }
    }

    /// Performance-Messung zwischen zwei Marks
    pub fn measure_performance(&self, start_mark: &str, end_mark: &str) -> Option<i64> {
        if let (Some(start), Some(end)) = (
            self.performance_marks.get(start_mark),
            self.performance_marks.get(end_mark)
        ) {
            let duration = end.signed_duration_since(*start);
            Some(duration.num_milliseconds())
        } else {
            None
        }
    }

    /// Bereinige Logs
    pub fn clear_console_logs(&mut self) {
        self.console_logs.clear();
        println!("🧹 Console logs cleared");
    }

    pub fn clear_network_logs(&mut self) {
        self.network_logs.clear();
        println!("🧹 Network logs cleared");
    }

    pub fn clear_all_logs(&mut self) {
        self.clear_console_logs();
        self.clear_network_logs();
        self.performance_marks.clear();
        println!("🧹 All logs cleared");
    }

    /// Filtere Console-Logs
    pub fn get_filtered_console_logs(&self) -> Vec<&ConsoleLogEntry> {
        self.console_logs.iter()
            .filter(|entry| self.filters.console_levels.iter().any(|level| 
                std::mem::discriminant(level) == std::mem::discriminant(&entry.level)
            ))
            .collect()
    }

    /// Filtere Network-Logs
    pub fn get_filtered_network_logs(&self) -> Vec<&NetworkLogEntry> {
        self.network_logs.iter()
            .filter(|entry| {
                // Method filter
                if !self.filters.network_methods.contains(&entry.method) {
                    return false;
                }
                
                // Response time filter
                if let Some(min_time) = self.filters.min_response_time {
                    if let Some(response_time) = entry.response_time {
                        if response_time < min_time {
                            return false;
                        }
                    }
                }
                
                // Status code filter
                if !self.filters.status_codes.is_empty() {
                    if let Some(status) = entry.status_code {
                        if !self.filters.status_codes.contains(&status) {
                            return false;
                        }
                    }
                }
                
                true
            })
            .collect()
    }

    /// Exportiere Logs
    pub fn export_logs(&self) -> Result<String> {
        let export_data = serde_json::json!({
            "console_logs": self.console_logs,
            "network_logs": self.network_logs,
            "performance_marks": self.performance_marks,
            "export_timestamp": Utc::now().to_rfc3339(),
            "version": "1.0"
        });

        serde_json::to_string_pretty(&export_data)
            .map_err(|e| anyhow::anyhow!("Export failed: {}", e))
    }

    /// Hole DevTools-Display
    pub fn get_devtools_display(&self) -> Vec<String> {
        let mut result = vec![
            "🔧 DEVELOPER TOOLS".to_string(),
            "==================".to_string(),
            "".to_string(),
        ];

        if !self.enabled {
            result.extend(vec![
                "❌ Developer Tools sind deaktiviert".to_string(),
                "💡 Verwende 'devtools toggle' zum Aktivieren".to_string(),
            ]);
            return result;
        }

        result.extend(vec![
            format!("📊 Statistiken:"),
            format!("   • Console-Logs: {}", self.console_logs.len()),
            format!("   • Network-Logs: {}", self.network_logs.len()),
            format!("   • Performance-Marks: {}", self.performance_marks.len()),
            "".to_string(),
        ]);

        // Console Logs
        result.push("📝 CONSOLE LOGS (letzte 10):".to_string());
        let console_logs = self.get_filtered_console_logs();
        for entry in console_logs.iter().rev().take(10) {
            result.push(format!(
                "[{}] {} {} - {}",
                entry.timestamp.format("%H:%M:%S"),
                entry.level.to_string(),
                entry.source,
                entry.message
            ));
        }
        result.push("".to_string());

        // Network Logs
        result.push("🌐 NETWORK LOGS (letzte 10):".to_string());
        let network_logs = self.get_filtered_network_logs();
        for entry in network_logs.iter().rev().take(10) {
            let status_info = match entry.status_code {
                Some(code) => format!("{} {}", entry.status_emoji(), code),
                None => if entry.error.is_some() { "❌ Error".to_string() } else { "⏳ Pending".to_string() },
            };

            result.push(format!(
                "[{}] {} {} {} - {} ({})",
                entry.timestamp.format("%H:%M:%S"),
                entry.method,
                entry.url,
                status_info,
                entry.format_response_time(),
                entry.format_size(entry.response_size)
            ));

            if let Some(error) = &entry.error {
                result.push(format!("   Error: {}", error));
            }
        }
        result.push("".to_string());

        // Performance Marks
        if !self.performance_marks.is_empty() {
            result.push("⏱️ PERFORMANCE MARKS:".to_string());
            for (mark, timestamp) in &self.performance_marks {
                result.push(format!("   • {} - {}", mark, timestamp.format("%H:%M:%S%.3f")));
            }
            result.push("".to_string());
        }

        result.extend(vec![
            "💡 KOMMANDOS:".to_string(),
            "• 'console clear' - Console-Logs löschen".to_string(),
            "• 'network clear' - Network-Logs löschen".to_string(),
            "• 'export logs' - Logs exportieren".to_string(),
        ]);

        result
    }

    /// Element-Inspector
    pub fn toggle_element_inspector(&mut self) -> bool {
        self.element_inspector_enabled = !self.element_inspector_enabled;
        println!("🔍 Element Inspector: {}", 
            if self.element_inspector_enabled { "Enabled" } else { "Disabled" }
        );
        self.element_inspector_enabled
    }

    pub fn inspect_element(&mut self, selector: &str) -> Vec<String> {
        if !self.enabled || !self.element_inspector_enabled {
            return vec!["Element Inspector ist deaktiviert".to_string()];
        }

        // Simuliere Element-Inspektion
        vec![
            format!("🔍 ELEMENT INSPECTOR: {}", selector),
            "".to_string(),
            "📋 Element Properties:".to_string(),
            format!("   • Tag: div"),
            format!("   • ID: example-id"),
            format!("   • Classes: container, main"),
            format!("   • Attributes: data-value=\"test\""),
            "".to_string(),
            "🎨 Computed Styles:".to_string(),
            format!("   • width: 100%"),
            format!("   • height: auto"),
            format!("   • display: block"),
            format!("   • color: #333333"),
        ]
    }

    // Getter & Setter
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_max_console_entries(&mut self, max: usize) {
        self.max_console_entries = max;
        
        // Trimme falls nötig
        while self.console_logs.len() > max {
            self.console_logs.pop_front();
        }
    }

    pub fn set_max_network_entries(&mut self, max: usize) {
        self.max_network_entries = max;
        
        // Trimme falls nötig
        while self.network_logs.len() > max {
            self.network_logs.pop_front();
        }
    }

    pub fn get_filters(&self) -> &LogFilters {
        &self.filters
    }

    pub fn get_filters_mut(&mut self) -> &mut LogFilters {
        &mut self.filters
    }

    pub fn get_console_log_count(&self) -> usize {
        self.console_logs.len()
    }

    pub fn get_network_log_count(&self) -> usize {
        self.network_logs.len()
    }
}

/// Formatiere Bytes in menschenlesbares Format
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

impl Default for DeveloperTools {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_developer_tools_creation() {
        let dev_tools = DeveloperTools::new();
        assert!(!dev_tools.is_enabled());
        assert_eq!(dev_tools.get_console_log_count(), 0);
        assert_eq!(dev_tools.get_network_log_count(), 0);
    }

    #[test]
    fn test_console_logging() {
        let mut dev_tools = DeveloperTools::new();
        dev_tools.toggle(); // Enable
        
        dev_tools.log_console(LogLevel::Info, "Test message", "TestSource");
        assert_eq!(dev_tools.get_console_log_count(), 2); // 1 toggle message + 1 test message
        
        let logs = dev_tools.get_filtered_console_logs();
        assert!(logs.iter().any(|log| log.message.contains("Test message")));
    }

    #[test]
    fn test_network_logging() {
        let mut dev_tools = DeveloperTools::new();
        dev_tools.toggle(); // Enable
        
        let _request_id = dev_tools.log_network_request("GET", "https://example.com");
        assert_eq!(dev_tools.get_network_log_count(), 1);
        
        dev_tools.log_network_response("https://example.com", 200, 150);
        
        let logs = dev_tools.get_filtered_network_logs();
        assert_eq!(logs[0].status_code, Some(200));
        assert_eq!(logs[0].response_time, Some(150));
    }

    #[test]
    fn test_performance_marks() {
        let mut dev_tools = DeveloperTools::new();
        dev_tools.toggle(); // Enable
        
        dev_tools.mark_performance("start");
        std::thread::sleep(std::time::Duration::from_millis(10));
        dev_tools.mark_performance("end");
        
        let duration = dev_tools.measure_performance("start", "end");
        assert!(duration.is_some());
        assert!(duration.unwrap() >= 10);
    }

    #[test]
    fn test_log_filtering() {
        let mut dev_tools = DeveloperTools::new();
        dev_tools.toggle(); // Enable
        
        dev_tools.log_console(LogLevel::Error, "Error message", "Test");
        dev_tools.log_console(LogLevel::Debug, "Debug message", "Test");
        
        // Filter out debug messages
        dev_tools.get_filters_mut().console_levels.retain(|level| 
            !matches!(level, LogLevel::Debug)
        );
        
        let filtered_logs = dev_tools.get_filtered_console_logs();
        assert!(!filtered_logs.iter().any(|log| log.message.contains("Debug message")));
        assert!(filtered_logs.iter().any(|log| log.message.contains("Error message")));
    }

    #[test]
    fn test_log_export() {
        let mut dev_tools = DeveloperTools::new();
        dev_tools.toggle(); // Enable
        
        dev_tools.log_console(LogLevel::Info, "Test log", "Test");
        dev_tools.log_network_request("GET", "https://test.com");
        
        let export = dev_tools.export_logs().unwrap();
        assert!(export.contains("Test log"));
        assert!(export.contains("https://test.com"));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
    }
}