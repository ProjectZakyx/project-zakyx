use tracing::{info, warn, error, debug, trace, Level};
use tracing_subscriber::{FmtSubscriber, EnvFilter};
use tracing_appender::{rolling, non_blocking};
use std::sync::Once;

static INIT: Once = Once::new();

/// 📊 STRUKTURIERTES LOGGING SYSTEM FÜR ZAKYX BROWSER
pub struct ZAKYXLogger {
    _guard: tracing_appender::non_blocking::WorkerGuard,
}

impl ZAKYXLogger {
    /// 🚀 INITIALISIERE LOGGING-SYSTEM
    pub fn init() -> eyre::Result<Self> {
        let mut logger = None;
        
        INIT.call_once(|| {
            // 📝 Log-Datei mit täglicher Rotation
            let file_appender = rolling::daily("logs", "zakyx_browser.log");
            let (non_blocking, guard) = non_blocking(file_appender);
            
            // 🎨 Formatter für strukturierte Logs
            let subscriber = FmtSubscriber::builder()
                .with_max_level(Level::TRACE)
                .with_env_filter(
                    EnvFilter::try_from_default_env()
                        .unwrap_or_else(|_| EnvFilter::new("info"))
                )
                .with_writer(non_blocking)
                .with_target(true)
                .with_thread_ids(true)
                .with_file(true)
                .with_line_number(true)
                .finish();
            
            tracing::subscriber::set_global_default(subscriber)
                .expect("Failed to set tracing subscriber");
            
            logger = Some(Self { _guard: guard });
            
                info!("🚀 ZAKYX Browser Logging System initialized");
    info!("📊 Logs werden in './logs/zakyx_browser.log' gespeichert");
        });
        
        Ok(logger.unwrap())
    }
    
    /// 🌐 LOG BROWSER EVENT
    pub fn log_browser_event(event: &str, details: &str) {
        info!(
            event = %event,
            details = %details,
            component = "browser",
            "🌐 Browser Event"
        );
    }
    
    /// 🎨 LOG UI EVENT
    pub fn log_ui_event(component: &str, action: &str, data: &str) {
        debug!(
            component = %component,
            action = %action,
            data = %data,
            "🎨 UI Event"
        );
    }
    
    /// 🛡️ LOG SECURITY EVENT
    pub fn log_security_event(event_type: &str, url: &str, blocked: bool) {
        warn!(
            event_type = %event_type,
            url = %url,
            blocked = %blocked,
            component = "security",
            "🛡️ Security Event"
        );
    }
    
    /// ⚡ LOG PERFORMANCE METRIC
    pub fn log_performance(metric: &str, value: f64, unit: &str) {
        trace!(
            metric = %metric,
            value = %value,
            unit = %unit,
            component = "performance",
            "⚡ Performance Metric"
        );
    }
    
    /// ❌ LOG ERROR WITH CONTEXT
    pub fn log_error(component: &str, error: &str, context: &str) {
        error!(
            component = %component,
            error = %error,
            context = %context,
            "❌ Error occurred"
        );
    }
    
    /// 🚀 LOG STARTUP EVENT
    pub fn log_startup(phase: &str, duration_ns: u64) {
        info!(
            phase = %phase,
            duration_ns = %duration_ns,
            component = "startup",
            "🚀 Startup Phase"
        );
    }
    
    /// 🌐 LOG WEBVIEW2 EVENT
    pub fn log_webview2(action: &str, url: Option<&str>, success: bool) {
        if success {
            info!(
                action = %action,
                url = %url.unwrap_or("N/A"),
                success = %success,
                component = "webview2",
                "🌐 WebView2 Success"
            );
        } else {
            warn!(
                action = %action,
                url = %url.unwrap_or("N/A"),
                success = %success,
                component = "webview2",
                "🌐 WebView2 Warning"
            );
        }
    }
    
    /// 📊 LOG HTTP REQUEST
    pub fn log_http_request(method: &str, url: &str, status: u16, duration_ms: u64) {
        debug!(
            method = %method,
            url = %url,
            status = %status,
            duration_ms = %duration_ms,
            component = "http",
            "📊 HTTP Request"
        );
    }
    
    /// 🔧 LOG CONFIGURATION CHANGE
    pub fn log_config_change(setting: &str, old_value: &str, new_value: &str) {
        info!(
            setting = %setting,
            old_value = %old_value,
            new_value = %new_value,
            component = "config",
            "🔧 Configuration Changed"
        );
    }
}

/// 🎯 CONVENIENCE MACROS FÜR BESSERE LOGGING
#[macro_export]
macro_rules! log_browser {
    ($event:expr, $details:expr) => {
        $crate::logging::OraLogger::log_browser_event($event, $details);
    };
}

#[macro_export]
macro_rules! log_ui {
    ($component:expr, $action:expr, $data:expr) => {
        $crate::logging::OraLogger::log_ui_event($component, $action, $data);
    };
}

#[macro_export]
macro_rules! log_security {
    ($event_type:expr, $url:expr, $blocked:expr) => {
        $crate::logging::OraLogger::log_security_event($event_type, $url, $blocked);
    };
}

#[macro_export]
macro_rules! log_perf {
    ($metric:expr, $value:expr, $unit:expr) => {
        $crate::logging::OraLogger::log_performance($metric, $value, $unit);
    };
}

#[macro_export]
macro_rules! log_error {
    ($component:expr, $error:expr, $context:expr) => {
        $crate::logging::OraLogger::log_error($component, $error, $context);
    };
}

/// 📈 PERFORMANCE LOGGER MIT AUTOMATISCHEM TIMING
pub struct PerformanceTimer {
    name: String,
    start: std::time::Instant,
}

impl PerformanceTimer {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            start: std::time::Instant::now(),
        }
    }
    
    pub fn finish(self) {
        let duration = self.start.elapsed();
        OraLogger::log_performance(&self.name, duration.as_nanos() as f64, "ns");
        info!(
            timer = %self.name,
            duration_ns = %duration.as_nanos(),
            "📈 Performance Timer finished"
        );
    }
}

/// 🎯 TIMING MACRO
#[macro_export]
macro_rules! time_it {
    ($name:expr, $block:block) => {{
        let _timer = $crate::logging::PerformanceTimer::new($name);
        let result = $block;
        _timer.finish();
        result
    }};
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_logging_system() {
        let _logger = OraLogger::init().expect("Failed to init logger");
        
        log_browser!("startup", "Testing logging system");
        log_ui!("button", "click", "start_browser");
        log_security!("ad_block", "https://example.com/ad", true);
        log_perf!("startup_time", 400.0, "ns");
        
        // Test performance timer
        let result = time_it!("test_operation", {
            std::thread::sleep(std::time::Duration::from_millis(1));
            "test_result"
        });
        
        assert_eq!(result, "test_result");
    }
} 
