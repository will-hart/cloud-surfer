use bevy::prelude::*;

use crate::{player::PlayerShip, GameState};

pub struct ScorePlugin;

pub struct Score {
    pub current: f32,
    pub multiplier: f32,
}

pub struct ScoreItem;
pub struct ScoreText;

impl Default for Score {
    fn default() -> Self {
        Score {
            current: 0.,
            multiplier: 1.,
        }
    }
}

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_score_ui.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(update_score.system().label("update_score"))
                .with_system(update_score_text_ui.system().after("update_score")),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(despawn_score_ui.system()),
        );
    }
}

/// spawns the score UI
fn spawn_score_ui(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.insert_resource(Score::default());

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::NONE.into()),
            ..Default::default()
        })
        .insert(ScoreItem)
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "".to_string(),
                            style: TextStyle {
                                font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                                font_size: 30.0,
                                color: Color::rgb(0.3, 0.3, 0.3),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                })
                .insert(ScoreText);
        });
}

/// Updates the score UI
fn update_score_text_ui(score: Res<Score>, mut score_text: Query<&mut Text, With<ScoreText>>) {
    for mut text in score_text.iter_mut() {
        text.sections[0].value = format!("{:.0}", score.current.floor());
    }
}

/// Increments the score by the time
fn update_score(time: Res<Time>, ship: Res<PlayerShip>, mut score: ResMut<Score>) {
    if ship.is_dead {
        return;
    }

    score.current += time.delta_seconds() * score.multiplier;
}

/// despawns the score ui
fn despawn_score_ui(mut commands: Commands, items: Query<Entity, With<ScoreItem>>) {
    commands.remove_resource::<Score>();

    for ent in items.iter() {
        commands.entity(ent).despawn_recursive();
    }
}
