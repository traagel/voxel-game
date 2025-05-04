use crate::gui::windows::city_info::CityInfoState;
use bevy_ecs::prelude::*;

#[derive(Resource)]
pub struct CityInfoStateRes(pub CityInfoState);

impl Default for CityInfoStateRes {
    fn default() -> Self {
        Self(CityInfoState::new())
    }
} 