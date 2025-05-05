#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum GameView {
    MainMenu,
    WorldGen,
    WorldMap,
    LocalMap,
    CityInfo,
    RegionMap,
}

pub mod world_map;
pub mod local_map;
pub mod city_info;
pub mod main_menu; 