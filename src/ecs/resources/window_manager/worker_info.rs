use crate::gui::windows::worker_info::WorkerInfoState;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct WorkerInfoStateRes(pub WorkerInfoState);

impl Default for WorkerInfoStateRes {
    fn default() -> Self {
        Self(WorkerInfoState::new())
    }
} 