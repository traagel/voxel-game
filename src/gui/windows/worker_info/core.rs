use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui};
use crate::gui::windows::worker_info::state::WorkerInfoState;

pub fn draw_worker_info_window(state: &mut WorkerInfoState) {
    let win_pos = vec2(40.0, 40.0);
    let win_size = vec2(300.0, 200.0);
    root_ui().window(hash!("worker_info_window"), win_pos, win_size, |ui| {
        ui.label(None, "[Worker Info Window - TODO]");
    });
} 