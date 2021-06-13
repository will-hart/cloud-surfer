use crate::{
    actions::Actions,
    player::{IsDead, PlayerShip},
    score::Score,
    GameState,
};
use bevy::prelude::*;

pub struct GameOverPlugin;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(transition_to_game_over.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::GameOver).with_system(restart_game.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::GameOver).with_system(despawn_game_over_ui.system()),
        );
    }
}

struct GameOverUiItem;

fn transition_to_game_over(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    score: Res<Score>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    players: Query<&IsDead>,
    mut state: ResMut<State<GameState>>,
) {
    let dead_player = players.single();
    if dead_player.is_err() {
        return;
    }

    let reason = dead_player.unwrap().0.clone();

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(GameOverUiItem)
        .with_children(|node| {
            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: format!(
                            "Oh Noooo! {} You scored {:.0}",
                            reason,
                            score.current.floor()
                        ),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.3, 0.3, 0.3),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Hit space to return to the menu".into(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.3, 0.3, 0.3),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });
        });
    state.set(GameState::GameOver).unwrap();
}

fn despawn_game_over_ui(mut commands: Commands, items: Query<Entity, With<GameOverUiItem>>) {
    for item in items.iter() {
        commands.entity(item).despawn_recursive();
    }
}

fn restart_game(
    ship: Res<PlayerShip>,
    mut actions: ResMut<Actions>,
    mut state: ResMut<State<GameState>>,
) {
    if !ship.is_dead {
        return;
    }

    if actions.restart_requested {
        state.set(GameState::Menu).unwrap();
        actions.restart_requested = false;
        return;
    }
}
