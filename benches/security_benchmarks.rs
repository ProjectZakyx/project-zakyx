//! # Ora Browser Security Benchmarks
//! 
//! Spezialisierte Benchmarks für Sicherheitsfeatures

use criterion::{black_box, criterion_group, criterion_main, Criterion, Throughput};
use projekt_ora::*;
use std::time::Duration;

// AdBlocker Performance Tests
fn bench_adblocker_performance(c: &mut Criterion) {
    let mut group = c.benchmark_group("adblocker_performance");
    
    // Single URL check
    group.bench_function("single_url_check", |b| {
        let mut adblocker = AdBlocker::new();
        let test_url = "https://doubleclick.net/ads/tracking/pixel.gif";
        
        b.iter(|| {
            black_box(adblocker.should_block(black_box(test_url)))
        })
    });
    
    // Batch URL processing
    let test_urls = vec![
        "https://doubleclick.net/ads/123",
        "https://googleadservices.com/pagead/conversion/",
        "https://facebook.com/tr?id=123456789",
        "https://google-analytics.com/collect",
        "https://googlesyndication.com/safeframe/",
        "https://amazon-adsystem.com/widgets/",
        "https://google.com/search?q=test", // Should not be blocked
        "https://github.com/user/repo",     // Should not be blocked
        "https://stackoverflow.com/questions/123", // Should not be blocked
        "https://youtube.com/watch?v=abc",  // Should not be blocked
    ];
    
    group.throughput(Throughput::Elements(test_urls.len() as u64));
    group.bench_function("batch_url_processing", |b| {
        let mut adblocker = AdBlocker::new();
        
        b.iter(|| {
            for url in &test_urls {
                black_box(adblocker.should_block(black_box(url)));
            }
        })
    });
    
    // Large scale URL processing
    let large_url_set: Vec<String> = (0..10000)
        .map(|i| {
            if i % 3 == 0 {
                format!("https://ads-tracker{}.com/pixel/{}", i % 100, i)
            } else {
                format!("https://legitimate-site{}.com/page/{}", i % 50, i)
            }
        })
        .collect();
    
    group.throughput(Throughput::Elements(large_url_set.len() as u64));
    group.bench_function("large_scale_processing", |b| {
        let mut adblocker = AdBlocker::new();
        
        b.iter(|| {
            for url in &large_url_set {
                black_box(adblocker.should_block(black_box(url)));
            }
        })
    });
    
    group.finish();
}

// Password Generation Security Benchmarks
fn bench_password_security(c: &mut Criterion) {
    let mut group = c.benchmark_group("password_security");
    
    // Different password lengths
    for length in [8, 12, 16, 24, 32, 64, 128] {
        group.bench_with_input(
            format!("generate_password_{}", length),
            &length,
            |b, &length| {
                let generator = PasswordGenerator::new();
                b.iter(|| {
                    let password = generator.generate(
                        black_box(length),
                        black_box(true),  // Include symbols
                        black_box(true),  // Include numbers
                        black_box(true),  // Include uppercase
                    );
                    black_box(password)
                })
            },
        );
    }
    
    // Password strength analysis
    let test_passwords = vec![
        "123456",                           // Weak
        "password123",                      // Weak
        "MyP@ssw0rd!",                     // Medium
        "Tr0ub4dor&3",                     // Strong
        "X7$mK9@pL2#vN8%qR5!wT1*zY6^uI3", // Very Strong
    ];
    
    group.bench_function("password_strength_analysis", |b| {
        let generator = PasswordGenerator::new();
        
        b.iter(|| {
            for password in &test_passwords {
                let strength = generator.analyze_strength(black_box(password));
                black_box(strength);
            }
        })
    });
    
    // Bulk password generation
    group.throughput(Throughput::Elements(1000));
    group.bench_function("bulk_password_generation", |b| {
        let generator = PasswordGenerator::new();
        
        b.iter(|| {
            for _ in 0..1000 {
                let password = generator.generate(16, true, true, true);
                black_box(password);
            }
        })
    });
    
    group.finish();
}

// HTTPS Enforcement Benchmarks
fn bench_https_enforcement(c: &mut Criterion) {
    let mut group = c.benchmark_group("https_enforcement");
    
    let test_urls = vec![
        ("http://example.com", "https://example.com"),
        ("http://google.com/search?q=test", "https://google.com/search?q=test"),
        ("http://subdomain.example.com/path?param=value", "https://subdomain.example.com/path?param=value"),
        ("https://already-secure.com", "https://already-secure.com"),
        ("http://localhost:8080/api", "https://localhost:8080/api"),
    ];
    
    group.throughput(Throughput::Elements(test_urls.len() as u64));
    group.bench_function("url_conversion", |b| {
        let mut enforcer = HTTPSEnforcer::new();
        
        b.iter(|| {
            for (input, _expected) in &test_urls {
                let result = enforcer.process_url(black_box(input));
                black_box(result);
            }
        })
    });
    
    // Large batch processing
    let large_url_batch: Vec<String> = (0..5000)
        .map(|i| {
            if i % 2 == 0 {
                format!("http://site{}.com/page/{}", i % 100, i)
            } else {
                format!("https://secure{}.com/page/{}", i % 100, i)
            }
        })
        .collect();
    
    group.throughput(Throughput::Elements(large_url_batch.len() as u64));
    group.bench_function("large_batch_conversion", |b| {
        let mut enforcer = HTTPSEnforcer::new();
        
        b.iter(|| {
            for url in &large_url_batch {
                let result = enforcer.process_url(black_box(url));
                black_box(result);
            }
        })
    });
    
    group.finish();
}

// Privacy Manager Benchmarks
fn bench_privacy_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("privacy_features");
    
    // Privacy mode switching
    group.bench_function("privacy_mode_switching", |b| {
        let mut manager = PrivacyManager::new();
        let modes = vec![
            PrivacyMode::Normal,
            PrivacyMode::Incognito,
            PrivacyMode::Strict,
        ];
        
        b.iter(|| {
            for mode in &modes {
                manager.set_mode(black_box(*mode));
                let current = manager.get_mode();
                black_box(current);
            }
        })
    });
    
    // Tracking detection
    let tracking_urls = vec![
        "https://facebook.com/tr?id=123",
        "https://google-analytics.com/collect",
        "https://doubleclick.net/pixel",
        "https://amazon-adsystem.com/widgets/",
        "https://googlesyndication.com/safeframe/",
        "https://legitimate-site.com/page", // Should not be blocked
        "https://github.com/user/repo",     // Should not be blocked
    ];
    
    group.throughput(Throughput::Elements(tracking_urls.len() as u64));
    group.bench_function("tracking_detection", |b| {
        let manager = PrivacyManager::new();
        
        b.iter(|| {
            for url in &tracking_urls {
                let is_tracking = manager.should_block_tracking(black_box(url));
                black_box(is_tracking);
            }
        })
    });
    
    group.finish();
}

// Security Alert Processing
fn bench_security_alerts(c: &mut Criterion) {
    let mut group = c.benchmark_group("security_alerts");
    
    // Alert creation and processing
    group.bench_function("alert_processing", |b| {
        let mut manager = SecurityFeaturesManager::new(std::ptr::null_mut()).unwrap();
        
        b.iter(|| {
            // Simulate various security events
            let events = vec![
                "Blocked malicious script",
                "HTTPS upgrade applied",
                "Tracking request blocked",
                "Insecure content detected",
                "Cookie policy violation",
            ];
            
            for event in &events {
                // This would normally create actual alerts
                black_box(event.len());
            }
        })
    });
    
    group.finish();
}

// Concurrent Security Operations
fn bench_concurrent_security(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_security");
    group.sample_size(20); // Fewer samples for concurrent tests
    
    group.bench_function("concurrent_url_checking", |b| {
        use std::sync::Arc;
        use tokio::runtime::Runtime;
        
        let rt = Runtime::new().unwrap();
        
        b.to_async(&rt).iter(|| async {
            let adblocker = Arc::new(parking_lot::Mutex::new(AdBlocker::new()));
            let test_urls = vec![
                "https://doubleclick.net/ads/1",
                "https://doubleclick.net/ads/2",
                "https://doubleclick.net/ads/3",
                "https://google.com/search/1",
                "https://google.com/search/2",
                "https://google.com/search/3",
            ];
            
            let handles: Vec<_> = test_urls.into_iter().map(|url| {
                let adblocker = Arc::clone(&adblocker);
                tokio::spawn(async move {
                    let mut blocker = adblocker.lock();
                    blocker.should_block(&url)
                })
            }).collect();
            
            for handle in handles {
                let result = handle.await.unwrap();
                black_box(result);
            }
        })
    });
    
    group.finish();
}

// Memory Security Benchmarks
fn bench_memory_security(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_security");
    
    // Secure memory clearing (simulation)
    group.bench_function("secure_memory_clear", |b| {
        b.iter(|| {
            // Simulate secure memory clearing for sensitive data
            let mut sensitive_data = vec![0u8; 1024];
            for i in 0..sensitive_data.len() {
                sensitive_data[i] = (i % 256) as u8;
            }
            
            // Clear memory securely
            for byte in &mut sensitive_data {
                *byte = 0;
            }
            
            black_box(sensitive_data);
        })
    });
    
    group.finish();
}

criterion_group!(
    security_benches,
    bench_adblocker_performance,
    bench_password_security,
    bench_https_enforcement,
    bench_privacy_features,
    bench_security_alerts,
    bench_concurrent_security,
    bench_memory_security
);

criterion_main!(security_benches); 