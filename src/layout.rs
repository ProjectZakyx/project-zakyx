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

    pub fn address_bar_dimensions(&self) -> (i32, i32, i32, i32) {
        let height = 30;
        let y_offset = 25; // Platz für Label
        (0, y_offset, self.window_width, height)
    }

    pub fn webview_dimensions(&self) -> (i32, i32, i32, i32) {
        let (_, y, _, address_bar_height) = self.address_bar_dimensions();
        let total_address_area = y + address_bar_height;
        (
            0,
            total_address_area,
            self.window_width,
            self.window_height - total_address_area,
        )
    }
}