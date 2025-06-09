//! # Ora Browser Performance Benchmarks
//! 
//! Benchmark-Tests für kritische Browser-Komponenten

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId};
use projekt_ora::*;
use std::time::Duration;
use tokio::runtime::Runtime;

fn rt() -> Runtime {
    Runtime::new().unwrap()
}

// Browser Startup Benchmarks
fn bench_browser_startup(c: &mut Criterion) {
    let mut group = c.benchmark_group("browser_startup");
    
    group.bench_function("webview_manager_init", |b| {
        b.iter(|| {
            let manager = WebView2Manager::new(black_box(std::ptr::null_mut()));
            black_box(manager)
        })
    });
    
    group.bench_function("performance_optimizer_init", |b| {
        b.to_async(&rt()).iter(|| async {
            let optimizer = PerformanceOptimizer::new().await;
            black_box(optimizer)
        })
    });
    
    group.bench_function("security_manager_init", |b| {
        b.iter(|| {
            let manager = SecurityFeaturesManager::new(black_box(std::ptr::null_mut()));
            black_box(manager)
        })
    });
    
    group.finish();
}

// Security Features Benchmarks
fn bench_security_features(c: &mut Criterion) {
    let mut group = c.benchmark_group("security_features");
    
    // AdBlocker Performance
    group.bench_function("adblocker_url_check", |b| {
        let mut adblocker = AdBlocker::new();
        let test_urls = vec![
            "https://doubleclick.net/ads/123",
            "https://google.com/search",
            "https://facebook.com/tr?id=123",
            "https://github.com/user/repo",
            "https://stackoverflow.com/questions/123",
        ];
        
        b.iter(|| {
            for url in &test_urls {
                black_box(adblocker.should_block(black_box(url)));
            }
        })
    });
    
    // Password Generation Performance
    group.bench_function("password_generation", |b| {
        let generator = PasswordGenerator::new();
        
        b.iter(|| {
            for length in [8, 12, 16, 24, 32] {
                let password = generator.generate(black_box(length), true, true, true);
                black_box(password);
            }
        })
    });
    
    // HTTPS Enforcement Performance
    group.bench_function("https_enforcement", |b| {
        let mut enforcer = HTTPSEnforcer::new();
        let test_urls = vec![
            "http://example.com",
            "http://google.com/search?q=test",
            "https://secure.com",
            "http://insecure.com/path/to/resource",
        ];
        
        b.iter(|| {
            for url in &test_urls {
                let result = enforcer.process_url(black_box(url));
                black_box(result);
            }
        })
    });
    
    group.finish();
}

// Memory Management Benchmarks
fn bench_memory_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_management");
    
    group.bench_function("memory_cleanup", |b| {
        b.to_async(&rt()).iter(|| async {
            let optimizer = PerformanceOptimizer::new().await.unwrap();
            let result = optimizer.cleanup_memory().await;
            black_box(result)
        })
    });
    
    group.bench_function("memory_stats_collection", |b| {
        b.to_async(&rt()).iter(|| async {
            let optimizer = PerformanceOptimizer::new().await.unwrap();
            let stats = optimizer.get_stats().await;
            black_box(stats)
        })
    });
    
    group.finish();
}

// Platform Detection Benchmarks
fn bench_platform_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("platform_detection");
    
    group.bench_function("platform_detection", |b| {
        b.iter(|| {
            let manager = CrossPlatformManager::new().unwrap();
            let platform = manager.detect_platform();
            black_box(platform)
        })
    });
    
    group.bench_function("platform_capabilities", |b| {
        let manager = CrossPlatformManager::new().unwrap();
        b.iter(|| {
            let capabilities = manager.get_platform_capabilities();
            black_box(capabilities)
        })
    });
    
    group.finish();
}

// URL Processing Benchmarks
fn bench_url_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("url_processing");
    
    let test_urls = vec![
        "https://google.com",
        "http://example.com/path/to/resource?param=value",
        "https://github.com/user/repo/issues/123",
        "http://localhost:8080/api/v1/data",
        "https://subdomain.example.com/complex/path?query=test&other=value",
    ];
    
    for url in &test_urls {
        group.bench_with_input(BenchmarkId::new("url_validation", url), url, |b, url| {
            b.iter(|| {
                // Simulate URL validation
                let is_valid = url.starts_with("http://") || url.starts_with("https://");
                black_box(is_valid)
            })
        });
    }
    
    group.finish();
}

// Theme Management Benchmarks
fn bench_theme_management(c: &mut Criterion) {
    let mut group = c.benchmark_group("theme_management");
    
    group.bench_function("theme_switching", |b| {
        let mut theme_manager = ThemeManager::new();
        let themes = vec![
            BrowserTheme::Light,
            BrowserTheme::Dark,
            BrowserTheme::Auto,
        ];
        
        b.iter(|| {
            for theme in &themes {
                theme_manager.set_theme(black_box(*theme));
                let current = theme_manager.get_current_theme();
                black_box(current);
            }
        })
    });
    
    group.finish();
}

// Concurrent Operations Benchmark
fn bench_concurrent_operations(c: &mut Criterion) {
    let mut group = c.benchmark_group("concurrent_operations");
    
    group.bench_function("concurrent_tab_operations", |b| {
        b.to_async(&rt()).iter(|| async {
            let optimizer = PerformanceOptimizer::new().await.unwrap();
            
            // Simulate concurrent operations
            let handles: Vec<_> = (0..10).map(|i| {
                let tab_id = format!("tab_{}", i);
                tokio::spawn(async move {
                    // Simulate tab operation
                    tokio::time::sleep(Duration::from_millis(1)).await;
                    tab_id
                })
            }).collect();
            
            // Wait for all operations
            for handle in handles {
                let result = handle.await;
                black_box(result);
            }
        })
    });
    
    group.finish();
}

// Stress Test Benchmarks
fn bench_stress_tests(c: &mut Criterion) {
    let mut group = c.benchmark_group("stress_tests");
    group.sample_size(20); // Fewer samples for stress tests
    
    group.bench_function("high_load_security_checks", |b| {
        let mut adblocker = AdBlocker::new();
        let test_urls: Vec<String> = (0..1000)
            .map(|i| format!("https://example{}.com/ads/tracker{}", i % 10, i))
            .collect();
        
        b.iter(|| {
            for url in &test_urls {
                black_box(adblocker.should_block(black_box(url)));
            }
        })
    });
    
    group.bench_function("mass_password_generation", |b| {
        let generator = PasswordGenerator::new();
        
        b.iter(|| {
            for _ in 0..100 {
                let password = generator.generate(black_box(16), true, true, true);
                black_box(password);
            }
        })
    });
    
    group.finish();
}

criterion_group!(
    benches,
    bench_browser_startup,
    bench_security_features,
    bench_memory_management,
    bench_platform_detection,
    bench_url_processing,
    bench_theme_management,
    bench_concurrent_operations,
    bench_stress_tests
);

criterion_main!(benches); 