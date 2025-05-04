use bevy_ecs::prelude::*;
use crate::gui::windows::city_info::portraits::CivPortraits;

#[derive(Resource)]
pub struct CivPortraitsRes(pub CivPortraits); 