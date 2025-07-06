// 🖱️ DRAG SYSTEM - Tab Drag Information
#[derive(Debug, Clone)]
pub struct TabDragInfo {
    pub tab_id: String,
    pub is_dragging: bool,
    pub start_x: i32,
    pub start_y: i32,
    pub current_x: i32,
    pub current_y: i32,
} 