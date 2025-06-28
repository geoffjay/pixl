use minifb::Key;

pub struct InputHandler;

impl InputHandler {
    pub fn is_ctrl_o_pressed(window: &minifb::Window) -> bool {
        window.is_key_pressed(Key::O, minifb::KeyRepeat::No) 
            && (window.is_key_down(Key::LeftCtrl) || window.is_key_down(Key::RightCtrl))
    }
    
    pub fn is_escape_pressed(window: &minifb::Window) -> bool {
        window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No)
    }
    
    pub fn is_left_arrow_pressed(window: &minifb::Window) -> bool {
        window.is_key_pressed(Key::Left, minifb::KeyRepeat::No)
    }
    
    pub fn is_right_arrow_pressed(window: &minifb::Window) -> bool {
        window.is_key_pressed(Key::Right, minifb::KeyRepeat::No)
    }
} 