use minifb::{Key, Window};

pub struct InputHandler;

impl InputHandler {
    pub fn is_ctrl_o_pressed(window: &Window) -> bool {
        window.is_key_pressed(Key::O, minifb::KeyRepeat::No) 
            && (window.is_key_down(Key::LeftCtrl) || window.is_key_down(Key::RightCtrl))
    }
    
    pub fn is_left_arrow_pressed(window: &Window) -> bool {
        window.is_key_pressed(Key::Left, minifb::KeyRepeat::No) ||
        window.is_key_pressed(Key::A, minifb::KeyRepeat::No)
    }
    
    pub fn is_right_arrow_pressed(window: &Window) -> bool {
        window.is_key_pressed(Key::Right, minifb::KeyRepeat::No) ||
        window.is_key_pressed(Key::D, minifb::KeyRepeat::No)
    }
    
    pub fn is_clear_error_pressed(window: &Window) -> bool {
        window.is_key_pressed(Key::C, minifb::KeyRepeat::No)
    }
    
    pub fn is_help_requested(window: &Window) -> bool {
        window.is_key_pressed(Key::H, minifb::KeyRepeat::No) ||
        window.is_key_pressed(Key::F1, minifb::KeyRepeat::No)
    }
    
    pub fn is_info_requested(window: &Window) -> bool {
        window.is_key_pressed(Key::I, minifb::KeyRepeat::No)
    }
    
    pub fn is_escape_pressed(window: &Window) -> bool {
        window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No)
    }
    
    pub fn is_quit_requested(window: &Window) -> bool {
        // Check for Ctrl+Q, Cmd+Q, or Escape
        let ctrl_q = (window.is_key_down(Key::LeftCtrl) || window.is_key_down(Key::RightCtrl)) 
                    && window.is_key_pressed(Key::Q, minifb::KeyRepeat::No);
        let cmd_q = (window.is_key_down(Key::LeftSuper) || window.is_key_down(Key::RightSuper)) 
                   && window.is_key_pressed(Key::Q, minifb::KeyRepeat::No);
        let escape = Self::is_escape_pressed(window);
        
        ctrl_q || cmd_q || escape
    }
} 