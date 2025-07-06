use crate::browser_state::{BrowserState, BrowserSettings};
use crate::error::OraBrowserError;

#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, BrowserState>) -> Result<BrowserSettings, OraBrowserError> {
    let settings = state.settings.read().await;
    Ok(settings.clone())
}

#[tauri::command]
pub async fn update_settings(
    state: tauri::State<'_, BrowserState>,
    new_settings: BrowserSettings,
) -> Result<(), OraBrowserError> {
    let mut settings = state.settings.write().await;
    *settings = new_settings;
    
    // Save to file
    if let Err(e) = save_settings_to_file(&settings) {
        println!("❌ Failed to save settings: {}", e);
        return Err(OraBrowserError::storage_error("write", &format!("Failed to save settings: {}", e), Some("settings.json")));
    }
    
    println!("⚙️ Settings updated successfully");
    Ok(())
}

fn save_settings_to_file(settings: &BrowserSettings) -> Result<(), OraBrowserError> {
    let json = serde_json::to_string_pretty(settings)
        .map_err(|e| OraBrowserError::storage_error("write", &format!("Failed to serialize settings: {}", e), Some("settings.json")))?;
    
    std::fs::write("settings.json", json)
        .map_err(|e| OraBrowserError::storage_error("write", &format!("Failed to write settings file: {}", e), Some("settings.json")))?;
    
    Ok(())
}

pub fn load_settings_from_file() -> Result<BrowserSettings, OraBrowserError> {
    let content = std::fs::read_to_string("settings.json")
        .map_err(|e| OraBrowserError::storage_error("read", &format!("Failed to read settings file: {}", e), Some("settings.json")))?;
    
    let settings: BrowserSettings = serde_json::from_str(&content)
        .map_err(|e| OraBrowserError::storage_error("read", &format!("Failed to parse settings JSON: {}", e), Some("settings.json")))?;
    
    Ok(settings)
} 