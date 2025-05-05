use crate::gui::windows::window_manager::WindowManager;

pub fn render(window_manager: &mut WindowManager) {
    window_manager.main_menu.draw();
} 