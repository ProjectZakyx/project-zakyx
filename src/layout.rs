pub struct BrowserLayout {
    window_width: i32,
    window_height: i32,
}

impl BrowserLayout {
    pub fn new(window_width: i32, window_height: i32) -> Self {
        Self {
            window_width,
            window_height,
        }
    }

    // Public getters for window dimensions
    pub fn get_window_width(&self) -> i32 {
        self.window_width
    }
    
    pub fn get_window_height(&self) -> i32 {
        self.window_height
    }

    // Helper function to get scaled dimensions based on window size
    fn scale_width(&self, percentage: f32) -> i32 {
        (self.window_width as f32 * percentage) as i32
    }

    fn scale_height(&self, percentage: f32) -> i32 {
        (self.window_height as f32 * percentage) as i32
    }

    // Navigation-Buttons Dimensionen (optimiert - kompakte Größe)
    pub fn back_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let tab_bar_height = self.scale_height(0.08); // 8% der Fensterhöhe für Tab-Bereich
        let width = self.scale_width(0.06); // 6% der Fensterbreite - halbiert
        let height = self.scale_height(0.05); // 5% der Fensterhöhe - halbiert
        let x = self.scale_width(0.01); // 1% Margin
        let y = tab_bar_height + self.scale_height(0.01);
        (x, y, width, height)
    }

    pub fn forward_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (back_x, back_y, back_width, _) = self.back_button_dimensions();
        let width = self.scale_width(0.06); // 6% - halbiert
        let height = self.scale_height(0.05); // 5% der Fensterhöhe - halbiert
        let x = back_x + back_width + self.scale_width(0.005);
        let y = back_y;
        (x, y, width, height)
    }

    pub fn refresh_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (forward_x, forward_y, forward_width, _) = self.forward_button_dimensions();
        let width = self.scale_width(0.08); // 8% - deutlich reduziert
        let height = self.scale_height(0.05); // 5% der Fensterhöhe - halbiert
        let x = forward_x + forward_width + self.scale_width(0.005);
        let y = forward_y;
        (x, y, width, height)
    }

    // Address Bar (kompakt optimiert)
    pub fn address_bar_dimensions(&self) -> (i32, i32, i32, i32) {
        let (refresh_x, refresh_y, refresh_width, _) = self.refresh_button_dimensions();
        let height = self.scale_height(0.05); // 5% der Fensterhöhe - halbiert
        let x = refresh_x + refresh_width + self.scale_width(0.01);
        let y = refresh_y;
        let width = self.scale_width(0.40); // 40% der Fensterbreite - etwas breiter
        (x, y, width, height)
    }

    // Address Label (kompakt)
    pub fn address_label_dimensions(&self) -> (i32, i32, i32, i32) {
        let (x, y, width, _) = self.address_bar_dimensions();
        let height = self.scale_height(0.02); // Kompaktes Label
        (x, y - height - 2, width, height)
    }

    // WebView-Bereich (größer und responsiv)
    pub fn webview_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, _, _, bookmarks_height) = self.bookmarks_area_dimensions();
        let (_, bookmarks_y, _, _) = self.bookmarks_area_dimensions();
        let webview_y = bookmarks_y + bookmarks_height + self.scale_height(0.01);
        (
            0,
            webview_y,
            self.window_width,
            self.window_height - webview_y,
        )
    }

    // Tab-Leiste (kompakt optimiert)
    pub fn tab_bar_dimensions(&self) -> (i32, i32, i32, i32) {
        let height = self.scale_height(0.06); // 6% der Fensterhöhe - deutlich reduziert
        (0, 0, self.window_width, height)
    }

    // Tab-Button Dimensionen (kompakt)
    pub fn tab_button_dimensions(&self, tab_index: usize, tab_count: usize) -> (i32, i32, i32, i32) {
        let (_, tab_bar_y, tab_bar_width, tab_bar_height) = self.tab_bar_dimensions();
        let new_tab_button_width = self.scale_width(0.03); // 3% für "+" Button - kleiner
        let available_width = tab_bar_width - new_tab_button_width;
        
        let tab_width = if tab_count > 0 {
            let calculated = available_width / tab_count as i32;
            calculated.min(self.scale_width(0.15)).max(self.scale_width(0.08)) // Min 8%, Max 15% - kompakter
        } else {
            self.scale_width(0.08)
        };
        
        let x = tab_index as i32 * tab_width;
        (x, tab_bar_y, tab_width, tab_bar_height)
    }

    // Neuer Tab Button (kompakt)
    pub fn new_tab_button_dimensions(&self, tab_count: usize) -> (i32, i32, i32, i32) {
        let (_, tab_bar_y, _, tab_bar_height) = self.tab_bar_dimensions();
        let new_tab_button_width = self.scale_width(0.03); // 3% - kleiner
        let available_width = self.window_width - new_tab_button_width;
        
        let tab_width = if tab_count > 0 {
            let calculated = available_width / tab_count as i32;
            calculated.min(self.scale_width(0.15)).max(self.scale_width(0.08)) // kompakter
        } else {
            self.scale_width(0.08)
        };
        
        let x = tab_count as i32 * tab_width;
        (x, tab_bar_y, new_tab_button_width, tab_bar_height)
    }

    // Navigation Label (kompakt)
    pub fn nav_label_dimensions(&self) -> (i32, i32, i32, i32) {
        let tab_bar_height = self.scale_height(0.06); // Angepasst an neue Tab-Höhe
        let x = self.scale_width(0.01);
        let height = self.scale_height(0.02); // Kompaktes Label
        (x, tab_bar_height + 3, self.scale_width(0.2), height)
    }

    // Bookmark Add Button (kompakt)
    pub fn bookmark_add_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (addr_x, addr_y, addr_width, addr_height) = self.address_bar_dimensions();
        let width = self.scale_width(0.08); // 8% - reduziert
        let height = addr_height;
        let x = addr_x + addr_width + self.scale_width(0.01);
        let y = addr_y;
        (x, y, width, height)
    }

    // Add bookmark button - alias for consistency
    pub fn add_bookmark_button_dimensions(&self) -> (i32, i32, i32, i32) {
        self.bookmark_add_button_dimensions()
    }

    // Manage bookmarks button - alias for info_bookmarks_button
    pub fn manage_bookmarks_button_dimensions(&self) -> (i32, i32, i32, i32) {
        self.info_bookmarks_button_dimensions()
    }

    // Bookmarks Area (kompakter Bereich)
    pub fn bookmarks_area_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, nav_bar_y, _, nav_bar_height) = self.address_bar_dimensions();
        let area_height = self.scale_height(0.08); // 8% der Fensterhöhe - halbiert
        let x = self.scale_width(0.01);
        let y = nav_bar_y + nav_bar_height + self.scale_height(0.01);
        let width = self.window_width - self.scale_width(0.02);
        (x, y, width, area_height)
    }

    // Bookmark Button (kompakt)
    pub fn bookmark_button_dimensions(&self, index: usize, bookmarks_per_row: usize) -> (i32, i32, i32, i32) {
        let (area_x, area_y, _, _) = self.bookmarks_area_dimensions();
        let button_width = self.scale_width(0.12); // 12% der Fensterbreite - reduziert
        let button_height = self.scale_height(0.03); // 3% der Fensterhöhe - reduziert
        let total_width = self.scale_width(0.14); // Mit Spacing - reduziert
        let spacing_y = self.scale_height(0.005);
        
        let row = index / bookmarks_per_row;
        let col = index % bookmarks_per_row;
        
        let x = area_x + col as i32 * total_width;
        let y = area_y + self.scale_height(0.015) + row as i32 * (button_height + spacing_y);
        
        (x, y, button_width, button_height)
    }

    // Delete Button (kompakt)
    pub fn bookmark_delete_button_dimensions(&self, index: usize, bookmarks_per_row: usize) -> (i32, i32, i32, i32) {
        let (bookmark_x, bookmark_y, bookmark_width, bookmark_height) = self.bookmark_button_dimensions(index, bookmarks_per_row);
        let delete_width = self.scale_width(0.02); // 2% der Fensterbreite - kleiner
        let x = bookmark_x + bookmark_width + self.scale_width(0.003);
        
        (x, bookmark_y, delete_width, bookmark_height)
    }

    // Bookmarks Header (kompakt)
    pub fn bookmarks_header_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, _) = self.bookmarks_area_dimensions();
        let height = self.scale_height(0.015); // kleiner
        (area_x, area_y, area_width, height)
    }

    // Management Buttons (kompakt)
    pub fn clear_bookmarks_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, area_height) = self.bookmarks_area_dimensions();
        let button_width = self.scale_width(0.08); // 8% - kleiner
        let button_height = self.scale_height(0.03); // 3% - kleiner
        let x = area_x + area_width - self.scale_width(0.17); // angepasst
        let y = area_y + area_height - self.scale_height(0.035);
        (x, y, button_width, button_height)
    }

    pub fn info_bookmarks_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, area_height) = self.bookmarks_area_dimensions();
        let button_width = self.scale_width(0.08); // 8% - kleiner
        let button_height = self.scale_height(0.03); // 3% - kleiner
        let x = area_x + area_width - self.scale_width(0.085); // angepasst
        let y = area_y + area_height - self.scale_height(0.035);
        (x, y, button_width, button_height)
    }

    // Homepage Button (kompakt)
    pub fn homepage_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (bookmark_x, bookmark_y, bookmark_width, bookmark_height) = self.bookmark_add_button_dimensions();
        let width = self.scale_width(0.08); // 8% - deutlich reduziert
        let height = bookmark_height;
        let x = bookmark_x + bookmark_width + self.scale_width(0.01);
        (x, bookmark_y, width, height)
    }
}
