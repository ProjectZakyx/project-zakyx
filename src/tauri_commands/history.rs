use crate::browser_state::BrowserState;
use crate::error::ZAKYXBrowserError;

#[tauri::command]
pub async fn get_history(state: tauri::State<'_, BrowserState>) -> Result<Vec<String>, ZAKYXBrowserError> {
    let history = state.history.read().await;
    Ok(history.clone())
}

#[tauri::command]
pub async fn clear_history(state: tauri::State<'_, BrowserState>) -> Result<(), ZAKYXBrowserError> {
    let mut history = state.history.write().await;
    history.clear();
    
    // Save to file
    if let Err(e) = save_history_to_file(&history) {
        println!("❌ Failed to save history: {}", e);
        return Err(ZAKYXBrowserError::storage_error("write", &format!("Failed to save history: {}", e), Some("history.json")));
    }
    
    println!("🗑️ History cleared successfully");
    Ok(())
}

#[tauri::command]
pub async fn add_to_history(
    state: tauri::State<'_, BrowserState>,
    url: String,
) -> Result<(), ZAKYXBrowserError> {
    let mut history = state.history.write().await;
    
    // Avoid duplicates
    if !history.contains(&url) {
        history.push(url.clone());
        
        // Keep only last 1000 entries
        if history.len() > 1000 {
            history.remove(0);
        }
        
        println!("📝 Added to history: {}", url);
    }
    
    Ok(())
}

fn save_history_to_file(history: &[String]) -> Result<(), ZAKYXBrowserError> {
    let json = serde_json::to_string_pretty(history)
        .map_err(|e| ZAKYXBrowserError::storage_error("write", &format!("Failed to serialize history: {}", e), Some("history.json")))?;
    
    std::fs::write("history.json", json)
        .map_err(|e| ZAKYXBrowserError::storage_error("write", &format!("Failed to write history file: {}", e), Some("history.json")))?;
    
    Ok(())
}

pub fn load_history_from_file() -> Result<Vec<String>, ZAKYXBrowserError> {
    let content = std::fs::read_to_string("history.json")
        .map_err(|e| ZAKYXBrowserError::storage_error("read", &format!("Failed to read history file: {}", e), Some("history.json")))?;
    
    let history: Vec<String> = serde_json::from_str(&content)
        .map_err(|e| ZAKYXBrowserError::storage_error("read", &format!("Failed to parse history JSON: {}", e), Some("history.json")))?;
    
    Ok(history)
} 
