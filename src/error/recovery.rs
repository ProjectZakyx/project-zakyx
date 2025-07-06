// 🔄 ERROR RECOVERY
// Retry-Logik und Fallback-Strategien
// Copyright © 2024 Ora Browser Team

use crate::error::{OraBrowserError, OraBrowserResult};

/// Error-Recovery-Utilities
pub struct ErrorRecovery;

impl ErrorRecovery {
    /// Führe Operation mit Retry-Logik aus
    pub async fn with_retry<T, F, Fut>(
        mut operation: F,
        max_retries: u32,
        delay_ms: u64,
    ) -> OraBrowserResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = OraBrowserResult<T>>,
    {
        let mut last_error = None;
        
        for attempt in 0..=max_retries {
            match operation().await {
                Ok(result) => return Ok(result),
                Err(error) => {
                    last_error = Some(error.clone());
                    
                    if attempt < max_retries && error.is_retryable() {
                        println!("🔄 Retry attempt {} of {}", attempt + 1, max_retries);
                        tokio::time::sleep(std::time::Duration::from_millis(delay_ms)).await;
                    }
                }
            }
        }
        
        Err(last_error.unwrap())
    }
    
    /// Führe Operation mit Fallback aus
    pub async fn with_fallback<T, F, Fut, G, Gut>(
        mut primary: F,
        mut fallback: G,
    ) -> OraBrowserResult<T>
    where
        F: FnMut() -> Fut,
        Fut: std::future::Future<Output = OraBrowserResult<T>>,
        G: FnMut() -> Gut,
        Gut: std::future::Future<Output = OraBrowserResult<T>>,
    {
        match primary().await {
            Ok(result) => Ok(result),
            Err(primary_error) => {
                println!("🔄 Primary operation failed, trying fallback: {}", primary_error);
                
                match fallback().await {
                    Ok(result) => {
                        println!("✅ Fallback succeeded");
                        Ok(result)
                    },
                    Err(fallback_error) => {
                        println!("❌ Fallback also failed: {}", fallback_error);
                        // Gebe den ursprünglichen Fehler zurück
                        Err(primary_error)
                    }
                }
            }
        }
    }
    
    /// Batch-Operation mit Error-Collection
    pub async fn batch_operation<T, F, Fut>(
        items: Vec<T>,
        mut operation: F,
    ) -> (Vec<T>, ErrorCollection)
    where
        F: FnMut(T) -> Fut,
        Fut: std::future::Future<Output = OraBrowserResult<T>>,
    {
        let mut results = Vec::new();
        let mut errors = ErrorCollection::new();
        
        for item in items {
            match operation(item).await {
                Ok(result) => {
                    results.push(result);
                    errors.add_success();
                },
                Err(error) => {
                    errors.add_error(error);
                }
            }
        }
        
        (results, errors)
    }
}

/// Error-Aggregation für Batch-Operationen
#[derive(Debug, Clone)]
pub struct ErrorCollection {
    pub errors: Vec<OraBrowserError>,
    pub successful_operations: usize,
    pub failed_operations: usize,
}

impl ErrorCollection {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            successful_operations: 0,
            failed_operations: 0,
        }
    }
    
    pub fn add_error(&mut self, error: OraBrowserError) {
        self.errors.push(error);
        self.failed_operations += 1;
    }
    
    pub fn add_success(&mut self) {
        self.successful_operations += 1;
    }
    
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    pub fn total_operations(&self) -> usize {
        self.successful_operations + self.failed_operations
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_operations() == 0 {
            0.0
        } else {
            self.successful_operations as f64 / self.total_operations() as f64
        }
    }
    
    pub fn summary(&self) -> String {
        format!(
            "📊 Operations Summary:\n\
            ✅ Successful: {}\n\
            ❌ Failed: {}\n\
            📈 Success Rate: {:.1}%\n\
            🚨 Errors: {}",
            self.successful_operations,
            self.failed_operations,
            self.success_rate() * 100.0,
            self.errors.len()
        )
    }
    
    /// Konvertiere zu einer zusammenfassenden Error, falls Fehler aufgetreten sind
    pub fn to_result(self) -> OraBrowserResult<()> {
        if self.has_errors() {
            let message = format!(
                "{} of {} operations failed. Success rate: {:.1}%",
                self.failed_operations,
                self.total_operations(),
                self.success_rate() * 100.0
            );
            
            Err(OraBrowserError::Internal {
                message,
                module: "batch_operations".to_string(),
            })
        } else {
            Ok(())
        }
    }
}

impl Default for ErrorCollection {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::Cell;
    
    #[test]
    fn test_error_collection() {
        let mut collection = ErrorCollection::new();
        
        collection.add_success();
        collection.add_success();
        collection.add_error(OraBrowserError::Unknown { 
            message: "test error".to_string() 
        });
        
        assert_eq!(collection.successful_operations, 2);
        assert_eq!(collection.failed_operations, 1);
        assert_eq!(collection.total_operations(), 3);
        assert!((collection.success_rate() - 0.6666).abs() < 0.01);
        assert!(collection.has_errors());
        
        let summary = collection.summary();
        assert!(summary.contains("Successful: 2"));
        assert!(summary.contains("Failed: 1"));
    }
    
    #[tokio::test]
    async fn test_error_recovery_with_retry() {
        let attempt_count = Cell::new(0);
        
        let result = ErrorRecovery::with_retry(
            || {
                let count = attempt_count.get() + 1;
                attempt_count.set(count);
                async move {
                    if count < 3 {
                        Err(OraBrowserError::Network {
                            message: "temporary failure".to_string(),
                            url: None,
                            retry_possible: true,
                        })
                    } else {
                        Ok("success")
                    }
                }
            },
            3,
            10, // 10ms delay for fast test
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
        assert_eq!(attempt_count.get(), 3);
    }
} 