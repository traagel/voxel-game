use bevy_ecs::prelude::*;
use crate::ecs::{
    components::camera::{Camera, WorldMapCamera, LocalMapCamera},
    resources::{
        game_view::GameViewRes,
        game_view::GameView,
    },
};
use crate::input::actions::Action;
use crate::input::poll_actions;
use macroquad::prelude::*;

pub fn handle_camera_input(
    mut world_camera_query: Query<&mut Camera, With<WorldMapCamera>>,
    mut local_camera_query: Query<&mut Camera, (With<LocalMapCamera>, Without<WorldMapCamera>)>,
    game_view: Res<GameViewRes>,
) {
    let move_speed = 200.0 * get_frame_time();
    let zoom_speed = 0.2;
    
    for action in poll_actions() {
        match action {
            Action::PanCamera { dx, dy } => {
                match game_view.active_view {
                    GameView::WorldMap | GameView::CityInfo => {
                        if let Ok(mut camera) = world_camera_query.single_mut() {
                            camera.x += dx * move_speed;
                            camera.y += dy * move_speed;
                        }
                    },
                    GameView::LocalMap => {
                        if let Ok(mut camera) = local_camera_query.single_mut() {
                            camera.x += dx * move_speed;
                            camera.y += dy * move_speed;
                        }
                    },
                    _ => {}
                }
            },
            Action::Zoom { delta, x: _, y: _ } => {
                match game_view.active_view {
                    GameView::WorldMap | GameView::CityInfo => {
                        if let Ok(mut camera) = world_camera_query.single_mut() {
                            camera.zoom = (camera.zoom + delta * zoom_speed).clamp(1.0, 10.0);
                        }
                    },
                    GameView::LocalMap => {
                        if let Ok(mut camera) = local_camera_query.single_mut() {
                            camera.zoom = (camera.zoom + delta * zoom_speed).clamp(1.0, 10.0);
                        }
                    },
                    _ => {}
                }
            },
            _ => {}
        }
    }
} 