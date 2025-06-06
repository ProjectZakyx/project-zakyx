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

    // Navigation-Buttons Dimensionen (nach unten verschoben für Tab-Leiste)
    pub fn back_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, tab_bar_height) = (0, self.tab_bar_dimensions().3);
        let width = 80;
        let height = 30;
        let x = 10;
        let y = tab_bar_height + 25;
        (x, y, width, height)
    }

    pub fn forward_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (back_x, back_y, back_width, _back_height) = self.back_button_dimensions();
        let width = 80;
        let height = 30;
        let x = back_x + back_width + 5;
        let y = back_y;
        (x, y, width, height)
    }

    pub fn refresh_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (forward_x, forward_y, forward_width, _forward_height) = self.forward_button_dimensions();
        let width = 100;
        let height = 30;
        let x = forward_x + forward_width + 5;
        let y = forward_y;
        (x, y, width, height)
    }

    // Aktualisierte Address Bar (verschoben wegen Buttons)
    pub fn address_bar_dimensions(&self) -> (i32, i32, i32, i32) {
        let (refresh_x, refresh_y, refresh_width, _refresh_height) = self.refresh_button_dimensions();
        let height = 30;
        let x = refresh_x + refresh_width + 10;
        let y = refresh_y;
        let width = self.window_width - x - 10;
        (x, y, width, height)
    }

    // Label für die URL-Eingabe
    pub fn address_label_dimensions(&self) -> (i32, i32, i32, i32) {
        let (x, y, width, _) = self.address_bar_dimensions();
        (x, y - 18, width, 15)
    }

    // WebView-Bereich (angepasst für Navigation-Leiste und horizontale Favoriten)
    pub fn webview_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, _, _, bookmarks_height) = self.bookmarks_area_dimensions();
        let (_, bookmarks_y, _, _) = self.bookmarks_area_dimensions();
        let webview_y = bookmarks_y + bookmarks_height + 10;
        (
            0,
            webview_y,
            self.window_width,
            self.window_height - webview_y,
        )
    }

    // Tab-Leiste (oberste Ebene)
    pub fn tab_bar_dimensions(&self) -> (i32, i32, i32, i32) {
        let height = 30;
        (0, 0, self.window_width, height)
    }

    // Tab-Button Dimensionen
    pub fn tab_button_dimensions(&self, tab_index: usize, tab_count: usize) -> (i32, i32, i32, i32) {
        let (_, tab_bar_y, tab_bar_width, tab_bar_height) = self.tab_bar_dimensions();
        let new_tab_button_width = 30; // "+" Button
        let available_width = tab_bar_width - new_tab_button_width;
        
        let tab_width = if tab_count > 0 {
            (available_width / tab_count as i32).min(200).max(120) // Min 120px, Max 200px
        } else {
            120
        };
        
        let x = tab_index as i32 * tab_width;
        (x, tab_bar_y, tab_width, tab_bar_height)
    }

    // Neuer Tab Button
    pub fn new_tab_button_dimensions(&self, tab_count: usize) -> (i32, i32, i32, i32) {
        let (_, tab_bar_y, tab_bar_width, tab_bar_height) = self.tab_bar_dimensions();
        let new_tab_button_width = 30;
        let available_width = tab_bar_width - new_tab_button_width;
        
        let tab_width = if tab_count > 0 {
            (available_width / tab_count as i32).min(200).max(120)
        } else {
            120
        };
        
        let x = tab_count as i32 * tab_width;
        (x, tab_bar_y, new_tab_button_width, tab_bar_height)
    }

    // Navigation-Leiste Label (verschoben nach unten)
    pub fn nav_label_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, tab_bar_height) = (0, self.tab_bar_dimensions().3);
        let (back_x, _, _, _) = self.back_button_dimensions();
        (back_x, tab_bar_height + 7, 300, 15)
    }

    // Lesezeichen-Button (neben URL-Eingabe)
    pub fn bookmark_add_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (addr_x, addr_y, addr_width, addr_height) = self.address_bar_dimensions();
        let width = 120;
        let height = addr_height;
        let x = addr_x + addr_width + 5;
        let y = addr_y;
        (x, y, width, height)
    }

    // Horizontale Favoritenleiste (Reihen-Layout)
    pub fn bookmarks_area_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, nav_bar_y, _, nav_bar_height) = self.address_bar_dimensions();
        let area_height = 120; // Platz für 3-4 Reihen von Favoriten
        let x = 10;
        let y = nav_bar_y + nav_bar_height + 10;
        let width = self.window_width - 20;
        (x, y, width, area_height)
    }

    // Einzelner Favoriten-Button (horizontal angeordnet)
    pub fn bookmark_button_dimensions(&self, index: usize, bookmarks_per_row: usize) -> (i32, i32, i32, i32) {
        let (area_x, area_y, _area_width, _) = self.bookmarks_area_dimensions();
        let button_width = 130; // Schmaler um Platz für "X" Button zu schaffen
        let button_height = 25;
        let total_width = 155; // Button + X Button + Abstand
        let spacing_y = 5;
        
        let row = index / bookmarks_per_row;
        let col = index % bookmarks_per_row;
        
        let x = area_x + col as i32 * total_width;
        let y = area_y + 20 + row as i32 * (button_height + spacing_y); // +20 für Header
        
        (x, y, button_width, button_height)
    }

    // "X" Button zum Löschen einzelner Favoriten
    pub fn bookmark_delete_button_dimensions(&self, index: usize, bookmarks_per_row: usize) -> (i32, i32, i32, i32) {
        let (bookmark_x, bookmark_y, bookmark_width, bookmark_height) = self.bookmark_button_dimensions(index, bookmarks_per_row);
        let delete_width = 20;
        let x = bookmark_x + bookmark_width + 3; // 3px Abstand
        let y = bookmark_y;
        
        (x, y, delete_width, bookmark_height)
    }

    // Favoriten-Header (horizontal)
    pub fn bookmarks_header_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, _) = self.bookmarks_area_dimensions();
        (area_x, area_y, area_width, 18)
    }

    // Management Buttons (Clear All / Info) - horizontal Layout
    pub fn clear_bookmarks_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, area_height) = self.bookmarks_area_dimensions();
        let button_width = 100;
        let button_height = 25;
        let x = area_x + area_width - 220; // Rechts in der Bookmarks-Area
        let y = area_y + area_height - 30;
        (x, y, button_width, button_height)
    }

    pub fn info_bookmarks_button_dimensions(&self) -> (i32, i32, i32, i32) {
        let (area_x, area_y, area_width, area_height) = self.bookmarks_area_dimensions();
        let button_width = 100;
        let button_height = 25;
        let x = area_x + area_width - 110; // Rechts vom Clear Button
        let y = area_y + area_height - 30;
        (x, y, button_width, button_height)
    }
}