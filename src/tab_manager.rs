use anyhow::Result;
use std::collections::HashMap;
use webview2_com::Microsoft::Web::WebView2::Win32::ICoreWebView2;

#[derive(Debug, Clone)]
pub struct Tab {
    pub id: usize,
    pub title: String,
    pub url: String,
    pub is_active: bool,
    pub is_loading: bool,
}

impl Tab {
    pub fn new(id: usize, title: String, url: String) -> Self {
        Self {
            id,
            title: if title.trim().is_empty() { "Neuer Tab".to_string() } else { title },
            url,
            is_active: false,
            is_loading: false,
        }
    }

    pub fn set_active(&mut self, active: bool) {
        self.is_active = active;
    }

    pub fn update_url(&mut self, url: String) {
        self.url = url.clone();
        
        // Extrahiere Titel aus URL
        self.title = if let Some(domain_start) = url.find("://") {
            if let Some(domain_end) = url[domain_start + 3..].find('/') {
                url[domain_start + 3..domain_start + 3 + domain_end].to_string()
            } else {
                url[domain_start + 3..].to_string()
            }
        } else {
            url.clone()
        };
    }

    pub fn get_display_title(&self) -> String {
        if self.title.len() > 20 {
            format!("{}...", &self.title[..17])
        } else {
            self.title.clone()
        }
    }
}

pub struct TabManager {
    tabs: HashMap<usize, Tab>,
    active_tab_id: Option<usize>,
    next_tab_id: usize,
    max_tabs: usize,
}

impl TabManager {
    pub fn new() -> Self {
        Self {
            tabs: HashMap::new(),
            active_tab_id: None,
            next_tab_id: 1,
            max_tabs: 10, // Limit auf 10 Tabs
        }
    }

    pub fn create_tab(&mut self, url: String) -> Result<usize> {
        if self.tabs.len() >= self.max_tabs {
            println!("⚠️  Maximum number of tabs ({}) reached", self.max_tabs);
            return Err(anyhow::anyhow!("Too many tabs"));
        }

        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        // Deaktiviere alle anderen Tabs
        for tab in self.tabs.values_mut() {
            tab.set_active(false);
        }

        let mut tab = Tab::new(tab_id, String::new(), url.clone());
        tab.set_active(true);
        tab.update_url(url);

        self.tabs.insert(tab_id, tab);
        self.active_tab_id = Some(tab_id);

        println!("📂 Created new tab #{} with URL: {}", tab_id, self.tabs[&tab_id].url);
        Ok(tab_id)
    }

    pub fn close_tab(&mut self, tab_id: usize) -> Result<Option<usize>> {
        if !self.tabs.contains_key(&tab_id) {
            return Err(anyhow::anyhow!("Tab not found"));
        }

        // Verhindere das Schließen des letzten Tabs
        if self.tabs.len() == 1 {
            println!("⚠️  Cannot close the last tab");
            return Ok(None);
        }

        let was_active = self.tabs[&tab_id].is_active;
        self.tabs.remove(&tab_id);

        println!("🗑️  Closed tab #{}", tab_id);

        // Wenn der aktive Tab geschlossen wurde, aktiviere einen anderen
        if was_active {
            self.active_tab_id = None;
            if let Some((&first_id, _)) = self.tabs.iter().next() {
                self.switch_to_tab(first_id)?;
                return Ok(Some(first_id));
            }
        }

        Ok(None)
    }

    pub fn switch_to_tab(&mut self, tab_id: usize) -> Result<()> {
        if !self.tabs.contains_key(&tab_id) {
            return Err(anyhow::anyhow!("Tab not found"));
        }

        // Deaktiviere alle Tabs
        for tab in self.tabs.values_mut() {
            tab.set_active(false);
        }

        // Aktiviere den gewünschten Tab
        self.tabs.get_mut(&tab_id).unwrap().set_active(true);
        self.active_tab_id = Some(tab_id);

        println!("🔄 Switched to tab #{}", tab_id);
        Ok(())
    }

    pub fn get_active_tab(&self) -> Option<&Tab> {
        if let Some(tab_id) = self.active_tab_id {
            self.tabs.get(&tab_id)
        } else {
            None
        }
    }

    pub fn get_active_tab_mut(&mut self) -> Option<&mut Tab> {
        if let Some(tab_id) = self.active_tab_id {
            self.tabs.get_mut(&tab_id)
        } else {
            None
        }
    }

    pub fn get_tab(&self, tab_id: usize) -> Option<&Tab> {
        self.tabs.get(&tab_id)
    }

    pub fn get_tabs(&self) -> Vec<&Tab> {
        let mut tabs: Vec<&Tab> = self.tabs.values().collect();
        tabs.sort_by_key(|tab| tab.id);
        tabs
    }

    pub fn get_active_tab_id(&self) -> Option<usize> {
        self.active_tab_id
    }

    pub fn update_active_tab_url(&mut self, url: String) {
        if let Some(tab) = self.get_active_tab_mut() {
            tab.update_url(url);
        }
    }

    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }

    pub fn has_tabs(&self) -> bool {
        !self.tabs.is_empty()
    }
}

impl Default for TabManager {
    fn default() -> Self {
        Self::new()
    }
} 