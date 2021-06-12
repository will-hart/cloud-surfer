mod actions;
mod audio;
pub mod game_map;
mod loading;
mod menu;
mod obstacles;
mod player;
mod score;

use crate::actions::ActionsPlugin;
use crate::audio::InternalAudioPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;
use crate::obstacles::ObstaclePlugin;
use crate::player::PlayerPlugin;
use crate::score::ScorePlugin;

use bevy::app::AppBuilder;
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(ActionsPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(ObstaclePlugin)
            .add_plugin(ScorePlugin)
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}
