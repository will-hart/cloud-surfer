use std::time::Duration;

use bevy::prelude::*;

use crate::{GameState, SystemLabels};

pub const GAME_TIME_DOUBLING_TIME: f32 = 60.; // e.g. 60 == double speed every minute

pub struct GameTime {
    pub multiplier: f32,
    pub elapsed: f32,
    pub delta: f32,
    pub delta_duration: Duration,
    pub fixed_update: bool,
    next_fixed_update: f64,
}

pub struct GameTimePlugin;

impl Plugin for GameTimePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(GameTime {
            multiplier: 1.,
            elapsed: 0.,
            delta: 0.,
            delta_duration: Duration::from_secs(0),
            fixed_update: false,
            next_fixed_update: 0.5,
        })
        .add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(setup_game_time.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_game_time.system().label(SystemLabels::UpdateTime))
                .with_system(speed_up_game_over_time.system()),
        );
    }
}

/// Resets the game timer to start a new game
fn setup_game_time(mut game_time: ResMut<GameTime>) {
    game_time.multiplier = 1.;
    game_time.elapsed = 0.;
    game_time.delta = 0.;
    game_time.delta_duration = Duration::from_secs(0);
    game_time.fixed_update = false;
    game_time.next_fixed_update = 0.5;
}

/// Updates the game timer
fn update_game_time(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    let dt = time.delta_seconds() * game_time.multiplier;

    game_time.elapsed += dt;
    game_time.delta = dt;
    game_time.delta_duration = time.delta().mul_f32(game_time.multiplier);

    if time.seconds_since_startup() > game_time.next_fixed_update {
        game_time.next_fixed_update = time.seconds_since_startup() + 0.1;
        game_time.fixed_update = true;
    } else {
        game_time.fixed_update = false;
    }
}

fn speed_up_game_over_time(mut game_time: ResMut<GameTime>) {
    game_time.multiplier = 1. + game_time.elapsed / GAME_TIME_DOUBLING_TIME;
}
