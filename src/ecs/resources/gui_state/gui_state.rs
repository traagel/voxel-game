use crate::gui::GuiState;
use bevy_ecs::prelude::*;

#[derive(Resource, Default)]
pub struct GuiStateRes(pub GuiState); 