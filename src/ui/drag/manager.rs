// 🖱️ DRAG SYSTEM - Tab Drag Manager
use super::info::TabDragInfo;

pub struct TabDragManager {
    drag_info: Option<TabDragInfo>,
    enabled: bool,
}

impl TabDragManager {
    pub fn new() -> Self {
        TabDragManager {
            drag_info: None,
            enabled: true,
        }
    }

    pub fn start_drag(&mut self, tab_id: String, x: i32, y: i32) {
        if !self.enabled {
            return;
        }

        self.drag_info = Some(TabDragInfo {
            tab_id: tab_id.clone(),
            is_dragging: true,
            start_x: x,
            start_y: y,
            current_x: x,
            current_y: y,
        });

        println!("🖱️ Tab drag started: {}", tab_id);
    }

    pub fn update_drag(&mut self, x: i32, y: i32) {
        if let Some(ref mut drag) = self.drag_info {
            if drag.is_dragging {
                drag.current_x = x;
                drag.current_y = y;

                let distance = ((x - drag.start_x).pow(2) + (y - drag.start_y).pow(2)) as f64;
                let distance = distance.sqrt();

                if distance > 10.0 {
                    println!(
                        "🖱️ Tab dragging: {} (distance: {:.1})",
                        drag.tab_id, distance
                    );
                }
            }
        }
    }

    pub fn end_drag(&mut self) -> Option<String> {
        if let Some(drag) = self.drag_info.take() {
            if drag.is_dragging {
                println!("🖱️ Tab drag ended: {}", drag.tab_id);
                return Some(drag.tab_id);
            }
        }
        None
    }

    pub fn is_dragging(&self) -> bool {
        self.drag_info.as_ref().is_some_and(|d| d.is_dragging)
    }

    pub fn toggle_drag(&mut self) -> bool {
        self.enabled = !self.enabled;
        println!(
            "🖱️ Tab drag: {}",
            if self.enabled { "Enabled" } else { "Disabled" }
        );
        self.enabled
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
} 