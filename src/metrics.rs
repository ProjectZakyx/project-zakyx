// 📊 PERFORMANCE METRICS SYSTEM

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tracing::{info, debug};
use std::sync::OnceLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricEntry {
    pub timestamp: u64,
    pub value: f64,
    pub tags: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceStats {
    pub avg: f64,
    pub min: f64,
    pub max: f64,
    pub count: u64,
    pub total: f64,
}

impl PerformanceStats {
    pub fn new() -> Self {
        Self {
            avg: 0.0,
            min: f64::MAX,
            max: f64::MIN,
            count: 0,
            total: 0.0,
        }
    }
    
    pub fn add_value(&mut self, value: f64) {
        self.count += 1;
        self.total += value;
        self.avg = self.total / self.count as f64;
        self.min = self.min.min(value);
        self.max = self.max.max(value);
    }
}

#[derive(Debug)]
pub struct MetricsCollector {
    metrics: Arc<Mutex<HashMap<String, Vec<MetricEntry>>>>,
    stats: Arc<Mutex<HashMap<String, PerformanceStats>>>,
    start_time: Instant,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(HashMap::new())),
            start_time: Instant::now(),
        }
    }
    
    /// Startet eine Timer-Messung
    pub fn start_timer(&self, name: &str) -> MetricTimer {
        MetricTimer::new(name.to_string(), self.clone())
    }
    
    /// Zeichnet einen Wert mit Tags auf
    pub fn record_value(&self, name: &str, value: f64, tags: Option<HashMap<String, String>>) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(0))
            .as_secs();
        
        let entry = MetricEntry {
            timestamp,
            value,
            tags: tags.unwrap_or_default(),
        };
        
        // Speichere Rohdaten
        {
            let mut metrics = self.metrics.lock().unwrap();
            metrics.entry(name.to_string()).or_insert_with(Vec::new).push(entry);
        }
        
        // Aktualisiere Statistiken
        {
            let mut stats = self.stats.lock().unwrap();
            stats.entry(name.to_string()).or_insert_with(PerformanceStats::new).add_value(value);
        }
        
        debug!("📊 Metric recorded: {} = {}", name, value);
    }
    
    /// Zeichnet eine Dauer auf
    pub fn record_duration(&self, name: &str, duration: Duration, tags: Option<HashMap<String, String>>) {
        self.record_value(name, duration.as_secs_f64(), tags);
    }
    
    /// Zählt ein Event
    pub fn count(&self, name: &str, tags: Option<HashMap<String, String>>) {
        self.record_value(name, 1.0, tags);
    }
    
    /// Holt Statistiken für eine Metrik
    pub fn get_stats(&self, name: &str) -> Option<PerformanceStats> {
        let stats = self.stats.lock().unwrap();
        stats.get(name).cloned()
    }
    
    /// Holt alle verfügbaren Metriken
    pub fn get_all_stats(&self) -> HashMap<String, PerformanceStats> {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }
    
    /// Exportiert Metriken als JSON
    pub fn export_json(&self) -> Result<String, serde_json::Error> {
        let stats = self.get_all_stats();
        let export_data = serde_json::json!({
            "uptime_seconds": self.start_time.elapsed().as_secs(),
            "timestamp": SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
            "stats": stats
        });
        serde_json::to_string_pretty(&export_data)
    }
    
    /// Bereinigt alte Metriken (behält nur die letzten N Einträge)
    pub fn cleanup_old_metrics(&self, keep_last: usize) {
        let mut metrics = self.metrics.lock().unwrap();
        for (_, entries) in metrics.iter_mut() {
            if entries.len() > keep_last {
                entries.drain(0..entries.len() - keep_last);
            }
        }
        debug!("🧹 Cleaned up old metrics, keeping last {} entries per metric", keep_last);
    }
    
    /// Loggt aktuelle Performance-Statistiken
    pub fn log_summary(&self) {
        let stats = self.get_all_stats();
        info!("📊 === PERFORMANCE METRICS SUMMARY ===");
        info!("🕐 Uptime: {:.2} seconds", self.start_time.elapsed().as_secs_f64());
        
        for (name, stat) in stats.iter() {
            info!(
                "📈 {}: avg={:.3}ms, min={:.3}ms, max={:.3}ms, count={}",
                name,
                stat.avg * 1000.0,
                stat.min * 1000.0,
                stat.max * 1000.0,
                stat.count
            );
        }
        info!("📊 === END METRICS SUMMARY ===");
    }
}

impl Clone for MetricsCollector {
    fn clone(&self) -> Self {
        Self {
            metrics: Arc::clone(&self.metrics),
            stats: Arc::clone(&self.stats),
            start_time: self.start_time,
        }
    }
}

/// Timer für automatische Zeitmessung
pub struct MetricTimer {
    name: String,
    start: Instant,
    collector: MetricsCollector,
    tags: HashMap<String, String>,
}

impl MetricTimer {
    fn new(name: String, collector: MetricsCollector) -> Self {
        Self {
            name,
            start: Instant::now(),
            collector,
            tags: HashMap::new(),
        }
    }
    
    /// Fügt Tags zum Timer hinzu
    pub fn with_tag(mut self, key: &str, value: &str) -> Self {
        self.tags.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Beendet den Timer und zeichnet die Dauer auf
    pub fn finish(self) {
        let duration = self.start.elapsed();
        self.collector.record_duration(&self.name, duration, Some(self.tags.clone()));
    }
}

impl Drop for MetricTimer {
    fn drop(&mut self) {
        let duration = self.start.elapsed();
        self.collector.record_duration(&self.name, duration, Some(self.tags.clone()));
    }
}

/// Globaler Metrics Collector
static GLOBAL_METRICS: OnceLock<MetricsCollector> = OnceLock::new();

/// Initialisiert den globalen Metrics Collector
pub fn init_metrics() -> MetricsCollector {
    let collector = MetricsCollector::new();
    let _ = GLOBAL_METRICS.set(collector.clone());
    info!("📊 Global metrics collector initialized");
    collector
}

/// Holt den globalen Metrics Collector
pub fn get_metrics() -> Option<MetricsCollector> {
    GLOBAL_METRICS.get().cloned()
}

/// Convenience-Makros für häufige Metriken
#[macro_export]
macro_rules! time_function {
    ($name:expr, $func:expr) => {{
        let _timer = crate::metrics::get_metrics()
            .map(|m| m.start_timer($name));
        $func
    }};
}

#[macro_export]
macro_rules! count_event {
    ($name:expr) => {
        if let Some(metrics) = crate::metrics::get_metrics() {
            metrics.count($name, None);
        }
    };
    ($name:expr, $($key:expr => $value:expr),*) => {
        if let Some(metrics) = crate::metrics::get_metrics() {
            let mut tags = std::collections::HashMap::new();
            $(
                tags.insert($key.to_string(), $value.to_string());
            )*
            metrics.count($name, Some(tags));
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_metrics_collector() {
        let collector = MetricsCollector::new();
        
        // Test value recording
        collector.record_value("test_metric", 1.5, None);
        collector.record_value("test_metric", 2.5, None);
        
        let stats = collector.get_stats("test_metric").unwrap();
        assert_eq!(stats.count, 2);
        assert_eq!(stats.avg, 2.0);
        assert_eq!(stats.min, 1.5);
        assert_eq!(stats.max, 2.5);
    }
    
    #[test]
    fn test_timer() {
        let collector = MetricsCollector::new();
        
        {
            let _timer = collector.start_timer("test_timer");
            thread::sleep(Duration::from_millis(10));
        }
        
        let stats = collector.get_stats("test_timer").unwrap();
        assert_eq!(stats.count, 1);
        assert!(stats.avg > 0.008); // Should be at least 8ms
    }
    
    #[test]
    fn test_export_json() {
        let collector = MetricsCollector::new();
        collector.record_value("test", 42.0, None);
        
        let json = collector.export_json().unwrap();
        assert!(json.contains("test"));
        assert!(json.contains("42"));
    }
} 
