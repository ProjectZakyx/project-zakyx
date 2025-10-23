// 📂 TAB MANAGEMENT - Tab Manager
use super::tab::BrowserTab;

#[derive(Debug)]
#[allow(dead_code)]
pub struct TabManager {
    tabs: Vec<BrowserTab>,
    active_tab_id: Option<u32>,
    next_tab_id: u32,
    max_tabs: usize,
}

impl TabManager {
    #[allow(dead_code)]
    pub fn new() -> Self {
        let mut manager = TabManager {
            tabs: Vec::new(),
            active_tab_id: None,
            next_tab_id: 1,
            max_tabs: 10,
        };

        // Create initial tab
        manager.create_new_tab("google.com");
        manager
    }

    #[allow(dead_code)]
    pub fn create_new_tab(&mut self, url: &str) -> u32 {
        if self.tabs.len() >= self.max_tabs {
            println!("⚠️ Maximum tabs ({}) reached!", self.max_tabs);
            return 0;
        }

        let tab_id = self.next_tab_id;
        self.next_tab_id += 1;

        let mut new_tab = BrowserTab::new(tab_id, url);
        
        // Deactivate all other tabs
        for tab in &mut self.tabs {
            tab.active = false;
        }

        new_tab.active = true;
        self.tabs.push(new_tab);
        self.active_tab_id = Some(tab_id);

        println!("📂 New tab created: {} ({})", tab_id, url);
        tab_id
    }

    #[allow(dead_code)]
    pub fn close_tab(&mut self, tab_id: u32) -> bool {
        if self.tabs.len() <= 1 {
            println!("⚠️ Cannot close last tab!");
            return false;
        }

        if let Some(pos) = self.tabs.iter().position(|t| t.id == tab_id) {
            let was_active = self.tabs[pos].active;
            self.tabs.remove(pos);

            if was_active {
                // Activate the tab to the left, or the first tab if none to the left
                let new_active_pos = if pos > 0 { pos - 1 } else { 0 };
                if let Some(tab) = self.tabs.get_mut(new_active_pos) {
                    tab.active = true;
                    self.active_tab_id = Some(tab.id);
                }
            }

            println!("🗑️ Tab closed: {}", tab_id);
            true
        } else {
            false
        }
    }

    #[allow(dead_code)]
    pub fn switch_to_tab(&mut self, tab_id: u32) -> Option<&BrowserTab> {
        // Deactivate all tabs
        for tab in &mut self.tabs {
            tab.active = false;
        }

        // Activate the selected tab
        if let Some(tab) = self.tabs.iter_mut().find(|t| t.id == tab_id) {
            tab.active = true;
            self.active_tab_id = Some(tab_id);
            println!("🔄 Switched to tab: {} ({})", tab_id, tab.url);
            Some(tab)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn get_active_tab(&mut self) -> Option<&mut BrowserTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter_mut().find(|t| t.id == id))
    }

    #[allow(dead_code)]
    pub fn get_active_tab_readonly(&self) -> Option<&BrowserTab> {
        self.active_tab_id.and_then(|id| self.tabs.iter().find(|t| t.id == id))
    }

    #[allow(dead_code)]
    pub fn get_all_tabs(&self) -> &Vec<BrowserTab> {
        &self.tabs
    }

    #[allow(dead_code)]
    pub fn navigate_active_tab(&mut self, url: &str) -> Option<&BrowserTab> {
        if let Some(tab) = self.get_active_tab() {
            tab.navigate_to(url);
            Some(tab)
        } else {
            None
        }
    }

    #[allow(dead_code)]
    pub fn go_back_active_tab(&mut self) -> Option<String> {
        self.get_active_tab().and_then(|tab| tab.go_back())
    }

    #[allow(dead_code)]
    pub fn go_forward_active_tab(&mut self) -> Option<String> {
        self.get_active_tab().and_then(|tab| tab.go_forward())
    }

    #[allow(dead_code)]
    pub fn get_tab_count(&self) -> usize {
        self.tabs.len()
    }
} 
