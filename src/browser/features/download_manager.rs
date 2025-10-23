// 📥 Download Manager für ZAKYX Browser
// Verwaltet Downloads, Progress-Tracking und Datei-Operationen

use anyhow::Result;
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use std::path::PathBuf;
use tokio::fs;

#[derive(Debug, Clone)]
pub struct Download {
    pub id: String,
    pub url: String,
    pub filename: String,
    pub file_path: PathBuf,
    pub progress: f64,
    pub status: DownloadStatus,
    pub created_at: DateTime<Utc>,
    pub file_size: u64,
    pub download_speed: f64, // KB/s
    pub estimated_time: Option<u64>, // seconds
}

#[derive(Debug, Clone)]
pub enum DownloadStatus {
    Pending,
    InProgress,
    Completed,
    Failed(String),
    Paused,
    Cancelled,
}

impl Download {
    pub fn new(url: String, filename: String) -> Self {
        let id = format!("dl_{}", Utc::now().timestamp());
        let file_path = std::env::current_dir()
            .unwrap_or_else(|_| PathBuf::from("."))
            .join("downloads")
            .join(&filename);

        Self {
            id,
            url,
            filename,
            file_path,
            progress: 0.0,
            status: DownloadStatus::Pending,
            created_at: Utc::now(),
            file_size: 0,
            download_speed: 0.0,
            estimated_time: None,
        }
    }

    /// Berechne geschätzte verbleibende Zeit
    pub fn calculate_estimated_time(&mut self) {
        if self.download_speed > 0.0 && self.progress < 100.0 {
            let remaining_bytes = (self.file_size as f64) * (100.0 - self.progress) / 100.0;
            let remaining_kb = remaining_bytes / 1024.0;
            self.estimated_time = Some((remaining_kb / self.download_speed) as u64);
        } else {
            self.estimated_time = None;
        }
    }

    /// Formatiere Dateigröße
    pub fn format_file_size(&self) -> String {
        format_bytes(self.file_size)
    }

    /// Formatiere Download-Geschwindigkeit
    pub fn format_speed(&self) -> String {
        if self.download_speed > 0.0 {
            format!("{:.1} KB/s", self.download_speed)
        } else {
            "-- KB/s".to_string()
        }
    }

    /// Formatiere geschätzte Zeit
    pub fn format_estimated_time(&self) -> String {
        match self.estimated_time {
            Some(seconds) => {
                if seconds < 60 {
                    format!("{}s", seconds)
                } else if seconds < 3600 {
                    format!("{}m {}s", seconds / 60, seconds % 60)
                } else {
                    format!("{}h {}m", seconds / 3600, (seconds % 3600) / 60)
                }
            }
            None => "--".to_string(),
        }
    }
}

pub struct DownloadManager {
    downloads: HashMap<String, Download>,
    download_directory: PathBuf,
    max_concurrent_downloads: usize,
    total_downloaded: u64,
    active_downloads: usize,
}

impl DownloadManager {
    pub fn new() -> Result<Self> {
        let download_directory = std::env::current_dir()?.join("downloads");
        
        // Erstelle Downloads-Verzeichnis falls nicht vorhanden
        if !download_directory.exists() {
            std::fs::create_dir_all(&download_directory)?;
        }

        Ok(Self {
            downloads: HashMap::new(),
            download_directory,
            max_concurrent_downloads: 3,
            total_downloaded: 0,
            active_downloads: 0,
        })
    }

    /// Starte neuen Download
    pub async fn start_download(&mut self, url: &str, filename: &str) -> Result<String> {
        if self.active_downloads >= self.max_concurrent_downloads {
            return Err(anyhow::anyhow!(
                "Maximum concurrent downloads reached: {}", 
                self.max_concurrent_downloads
            ));
        }

        let mut download = Download::new(url.to_string(), filename.to_string());
        download.status = DownloadStatus::InProgress;
        
        let download_id = download.id.clone();
        self.downloads.insert(download_id.clone(), download);
        self.active_downloads += 1;

        println!("📥 Starting download: {} → {}", url, filename);
        
        // Simuliere Download-Progress (in echter Implementation würde hier HTTP-Request stehen)
        self.simulate_download(&download_id).await?;
        
        Ok(download_id)
    }

    /// Simuliere Download-Progress
    async fn simulate_download(&mut self, download_id: &str) -> Result<()> {
        if let Some(download) = self.downloads.get_mut(download_id) {
            // Simuliere Dateigröße
            download.file_size = 1024 * 1024 * 5; // 5MB
            
            // Simuliere Progress
            for i in 0..=10 {
                download.progress = (i as f64) * 10.0;
                download.download_speed = 150.0 + (i as f64) * 10.0; // Variable Geschwindigkeit
                download.calculate_estimated_time();
                
                println!("📊 Download progress: {}% ({})", download.progress, download.format_speed());
                tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
            }

            download.status = DownloadStatus::Completed;
            download.progress = 100.0;
            download.download_speed = 0.0;
            download.estimated_time = None;
            
            self.active_downloads = self.active_downloads.saturating_sub(1);
            self.total_downloaded += download.file_size;
            
            println!("✅ Download completed: {}", download.filename);
        }

        Ok(())
    }

    /// Pausiere Download
    pub fn pause_download(&mut self, download_id: &str) -> Result<()> {
        if let Some(download) = self.downloads.get_mut(download_id) {
            match download.status {
                DownloadStatus::InProgress => {
                    download.status = DownloadStatus::Paused;
                    self.active_downloads = self.active_downloads.saturating_sub(1);
                    println!("⏸️ Download paused: {}", download.filename);
                    Ok(())
                }
                _ => Err(anyhow::anyhow!("Cannot pause download in current state"))
            }
        } else {
            Err(anyhow::anyhow!("Download not found: {}", download_id))
        }
    }

    /// Setze Download fort
    pub async fn resume_download(&mut self, download_id: &str) -> Result<()> {
        if self.active_downloads >= self.max_concurrent_downloads {
            return Err(anyhow::anyhow!("Maximum concurrent downloads reached"));
        }

        if let Some(download) = self.downloads.get_mut(download_id) {
            match download.status {
                DownloadStatus::Paused => {
                    download.status = DownloadStatus::InProgress;
                    self.active_downloads += 1;
                    println!("▶️ Download resumed: {}", download.filename);
                    
                    // Setze Download fort
                    let id = download_id.to_string();
                    self.simulate_download(&id).await?;
                    Ok(())
                }
                _ => Err(anyhow::anyhow!("Cannot resume download in current state"))
            }
        } else {
            Err(anyhow::anyhow!("Download not found: {}", download_id))
        }
    }

    /// Brich Download ab
    pub fn cancel_download(&mut self, download_id: &str) -> Result<()> {
        if let Some(download) = self.downloads.get_mut(download_id) {
            match download.status {
                DownloadStatus::InProgress | DownloadStatus::Paused | DownloadStatus::Pending => {
                    download.status = DownloadStatus::Cancelled;
                    if matches!(download.status, DownloadStatus::InProgress) {
                        self.active_downloads = self.active_downloads.saturating_sub(1);
                    }
                    
                    // Lösche unvollständige Datei
                    if download.file_path.exists() {
                        let _ = std::fs::remove_file(&download.file_path);
                    }
                    
                    println!("❌ Download cancelled: {}", download.filename);
                    Ok(())
                }
                _ => Err(anyhow::anyhow!("Cannot cancel download in current state"))
            }
        } else {
            Err(anyhow::anyhow!("Download not found: {}", download_id))
        }
    }

    /// Entferne Download aus Liste
    pub fn remove_download(&mut self, download_id: &str) -> Result<()> {
        if let Some(download) = self.downloads.remove(download_id) {
            // Lösche Datei falls gewünscht
            if download.file_path.exists() && 
               matches!(download.status, DownloadStatus::Failed(_) | DownloadStatus::Cancelled) {
                let _ = std::fs::remove_file(&download.file_path);
            }
            
            println!("🗑️ Download removed: {}", download.filename);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Download not found: {}", download_id))
        }
    }

    /// Bereinige abgeschlossene Downloads
    pub fn cleanup_completed(&mut self) -> usize {
        let initial_count = self.downloads.len();
        
        self.downloads.retain(|_, download| {
            !matches!(download.status, DownloadStatus::Completed | DownloadStatus::Failed(_))
        });
        
        let removed = initial_count - self.downloads.len();
        if removed > 0 {
            println!("🧹 Cleaned up {} completed downloads", removed);
        }
        
        removed
    }

    /// Hole Download-Statistiken
    pub fn get_stats(&self) -> DownloadStats {
        let mut stats = DownloadStats::default();
        
        for download in self.downloads.values() {
            match download.status {
                DownloadStatus::Pending => stats.pending += 1,
                DownloadStatus::InProgress => stats.in_progress += 1,
                DownloadStatus::Completed => stats.completed += 1,
                DownloadStatus::Failed(_) => stats.failed += 1,
                DownloadStatus::Paused => stats.paused += 1,
                DownloadStatus::Cancelled => stats.cancelled += 1,
            }
        }
        
        stats.total_downloaded = self.total_downloaded;
        stats.active_downloads = self.active_downloads;
        stats
    }

    /// Hole Downloads-Display
    pub fn get_downloads_display(&self) -> Vec<String> {
        let mut result = vec![
            "📥 DOWNLOAD MANAGER".to_string(),
            "==================".to_string(),
            "".to_string(),
        ];

        let stats = self.get_stats();
        result.extend(vec![
            format!("📊 Statistiken:"),
            format!("   • Aktiv: {} / {}", self.active_downloads, self.max_concurrent_downloads),
            format!("   • Abgeschlossen: {}", stats.completed),
            format!("   • Fehlgeschlagen: {}", stats.failed),
            format!("   • Gesamt heruntergeladen: {}", format_bytes(self.total_downloaded)),
            "".to_string(),
        ]);

        if self.downloads.is_empty() {
            result.push("📭 Keine Downloads vorhanden".to_string());
        } else {
            result.push("📋 AKTUELLE DOWNLOADS:".to_string());
            for download in self.downloads.values() {
                let status_str = match &download.status {
                    DownloadStatus::Pending => "⏳ Wartend",
                    DownloadStatus::InProgress => "📥 Lädt...",
                    DownloadStatus::Completed => "✅ Fertig",
                    DownloadStatus::Failed(err) => &format!("❌ Fehler: {}", err),
                    DownloadStatus::Paused => "⏸️ Pausiert",
                    DownloadStatus::Cancelled => "❌ Abgebrochen",
                };

                result.push(format!("📄 {}", download.filename));
                result.push(format!("   URL: {}", download.url));
                result.push(format!("   Status: {} ({:.1}%)", status_str, download.progress));
                result.push(format!("   Größe: {} | Geschwindigkeit: {}", 
                    download.format_file_size(), download.format_speed()));
                
                if download.estimated_time.is_some() {
                    result.push(format!("   Verbleibend: {}", download.format_estimated_time()));
                }
                
                result.push(format!("   Erstellt: {}", download.created_at.format("%d.%m.%Y %H:%M")));
                result.push("".to_string());
            }
        }

        result
    }

    /// Suche Downloads
    pub fn search_downloads(&self, query: &str) -> Vec<&Download> {
        let query_lower = query.to_lowercase();
        self.downloads.values()
            .filter(|download| {
                download.filename.to_lowercase().contains(&query_lower) ||
                download.url.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    // Getter & Setter
    pub fn get_downloads(&self) -> &HashMap<String, Download> {
        &self.downloads
    }

    pub fn get_download(&self, download_id: &str) -> Option<&Download> {
        self.downloads.get(download_id)
    }

    pub fn set_max_concurrent_downloads(&mut self, max: usize) {
        self.max_concurrent_downloads = max.max(1);
        println!("⚙️ Max concurrent downloads set to: {}", self.max_concurrent_downloads);
    }

    pub fn get_download_directory(&self) -> &PathBuf {
        &self.download_directory
    }

    pub fn set_download_directory(&mut self, path: PathBuf) -> Result<()> {
        if !path.exists() {
            std::fs::create_dir_all(&path)?;
        }
        self.download_directory = path;
        println!("📁 Download directory changed to: {:?}", self.download_directory);
        Ok(())
    }
}

#[derive(Debug, Default)]
pub struct DownloadStats {
    pub pending: usize,
    pub in_progress: usize,
    pub completed: usize,
    pub failed: usize,
    pub paused: usize,
    pub cancelled: usize,
    pub total_downloaded: u64,
    pub active_downloads: usize,
}

/// Formatiere Bytes in menschenlesbares Format
fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
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

impl Default for DownloadManager {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| Self {
            downloads: HashMap::new(),
            download_directory: PathBuf::from("downloads"),
            max_concurrent_downloads: 3,
            total_downloaded: 0,
            active_downloads: 0,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_creation() {
        let download = Download::new("https://example.com/file.txt".to_string(), "file.txt".to_string());
        assert_eq!(download.filename, "file.txt");
        assert_eq!(download.progress, 0.0);
        assert!(matches!(download.status, DownloadStatus::Pending));
    }

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1024 * 1024), "1.0 MB");
        assert_eq!(format_bytes(1024 * 1024 * 1024), "1.0 GB");
    }

    #[tokio::test]
    async fn test_download_manager() {
        let mut manager = DownloadManager::new().unwrap();
        
        let download_id = manager.start_download("https://example.com/test.txt", "test.txt").await.unwrap();
        assert!(manager.get_download(&download_id).is_some());
        
        let stats = manager.get_stats();
        assert!(stats.completed > 0 || stats.in_progress > 0);
    }

    #[test]
    fn test_download_search() {
        let mut manager = DownloadManager::new().unwrap();
        let mut download = Download::new("https://example.com/test.pdf".to_string(), "test.pdf".to_string());
        download.id = "test_id".to_string();
        manager.downloads.insert("test_id".to_string(), download);
        
        let results = manager.search_downloads("pdf");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].filename, "test.pdf");
    }

    #[test]
    fn test_download_stats() {
        let mut manager = DownloadManager::new().unwrap();
        let mut download = Download::new("https://example.com/test.txt".to_string(), "test.txt".to_string());
        download.status = DownloadStatus::Completed;
        manager.downloads.insert("test".to_string(), download);
        
        let stats = manager.get_stats();
        assert_eq!(stats.completed, 1);
    }
}