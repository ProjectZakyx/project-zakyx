use crate::browser_state::{BrowserState, Bookmark};
use crate::error::OraBrowserError;

#[tauri::command]
pub async fn add_bookmark(
    state: tauri::State<'_, BrowserState>,
    title: String,
    url: String,
) -> Result<Bookmark, OraBrowserError> {
    let mut bookmarks = state.bookmarks.write().await;
    
    let bookmark = Bookmark {
        id: uuid::Uuid::new_v4().to_string(),
        title,
        url,
    };
    
    bookmarks.push(bookmark.clone());
    
    println!("📖 Bookmark added: {}", bookmark.title);
    Ok(bookmark)
}

#[tauri::command]
pub async fn get_bookmarks(state: tauri::State<'_, BrowserState>) -> Result<Vec<Bookmark>, OraBrowserError> {
    let bookmarks = state.bookmarks.read().await;
    Ok(bookmarks.clone())
}

#[tauri::command]
pub async fn remove_bookmark(
    state: tauri::State<'_, BrowserState>,
    bookmark_id: String,
) -> Result<(), OraBrowserError> {
    let mut bookmarks = state.bookmarks.write().await;
    
    if let Some(pos) = bookmarks.iter().position(|b| b.id == bookmark_id) {
        let removed = bookmarks.remove(pos);
        println!("🗑️ Bookmark removed: {}", removed.title);
        
        // Save to file
        if let Err(e) = save_bookmarks_to_file(&bookmarks) {
            println!("❌ Failed to save bookmarks: {}", e);
            return Err(OraBrowserError::storage_error("write", &format!("Failed to save bookmarks: {}", e), Some("bookmarks.json")));
        }
    }
    
    Ok(())
}

#[tauri::command]
pub async fn sync_bookmarks(
    state: tauri::State<'_, BrowserState>,
) -> Result<Vec<Bookmark>, OraBrowserError> {
    let bookmarks = state.bookmarks.read().await;
    
    // Save to file
    if let Err(e) = save_bookmarks_to_file(&bookmarks) {
        println!("❌ Failed to save bookmarks: {}", e);
        return Err(OraBrowserError::storage_error("write", &format!("Failed to save bookmarks: {}", e), Some("bookmarks.json")));
    }
    
    println!("✅ Bookmarks synced successfully");
    Ok(bookmarks.clone())
}

fn save_bookmarks_to_file(bookmarks: &[Bookmark]) -> Result<(), OraBrowserError> {
    let json = serde_json::to_string_pretty(bookmarks)
        .map_err(|e| OraBrowserError::storage_error("write", &format!("Failed to serialize bookmarks: {}", e), Some("bookmarks.json")))?;
    
    std::fs::write("bookmarks.json", json)
        .map_err(|e| OraBrowserError::storage_error("write", &format!("Failed to write bookmarks file: {}", e), Some("bookmarks.json")))?;
    
    Ok(())
}

pub fn load_bookmarks_from_file() -> Result<Vec<Bookmark>, OraBrowserError> {
    let content = std::fs::read_to_string("bookmarks.json")
        .map_err(|e| OraBrowserError::storage_error("read", &format!("Failed to read bookmarks file: {}", e), Some("bookmarks.json")))?;
    
    let bookmarks: Vec<Bookmark> = serde_json::from_str(&content)
        .map_err(|e| OraBrowserError::storage_error("read", &format!("Failed to parse bookmarks JSON: {}", e), Some("bookmarks.json")))?;
    
    Ok(bookmarks)
} 