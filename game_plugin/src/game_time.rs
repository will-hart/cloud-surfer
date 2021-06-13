use std::time::Duration;

use bevy::prelude::*;

use crate::{GameState, SystemLabels};

pub struct GameTime {
    pub multiplier: f32,
    pub elapsed: f32,
    pub delta: f32,
    pub delta_duration: Duration,
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GameTime {
            multiplier: 1.,
            elapsed: 0.,
            delta: 0.,
            delta_duration: Duration::from_secs(0),
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_game_time.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_game_time.system().label(SystemLabels::UpdateTime)),
        );
    }
}

/// Resets the game timer to start a new game
fn setup_game_time(mut game_time: ResMut<GameTime>) {
    game_time.multiplier = 1.;
    game_time.elapsed = 0.;
    game_time.delta = 0.;
    game_time.delta_duration = Duration::from_secs(0);
}

/// Updates the game timer
fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    let dt = time.delta_seconds() * game_time.multiplier;

    game_time.elapsed += dt;
    game_time.delta = dt;
    game_time.delta_duration = time.delta().mul_f32(game_time.multiplier);
}
