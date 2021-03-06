use crate::{actions::Actions, GameState};
use bevy::prelude::*;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<ButtonMaterials>()
            .add_startup_system(spawn_ui_camera.system())
            .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup_menu.system()))
            .add_system_set(
                SystemSet::on_update(GameState::Menu).with_system(click_play_button.system()),
            )
            .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(despawn_menu.system()));
    }
}

struct ButtonMaterials {
    normal: Handle<ColorMaterial>,
    hovered: Handle<ColorMaterial>,
}

impl FromWorld for ButtonMaterials {
    fn from_world(world: &mut World) -> Self {
        let mut materials = world.get_resource_mut::<Assets<ColorMaterial>>().unwrap();
        ButtonMaterials {
            normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
            hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
        }
    }
}

struct MenuItem;

struct PlayButton;

fn spawn_ui_camera(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
}

fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    button_materials: Res<ButtonMaterials>,
) {
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: button_materials.normal.clone(),
            ..Default::default()
        })
        .insert(MenuItem)
        .with_children(|node| {
            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "HyperFarmer!1!!".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Oh no! Its about to rain and we need to save our hay.".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Hmm... What if (hear me out)... what if we use a big laser?"
                            .to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value:
                            "Capture the hay with your laser tether, but don't move too far apart"
                                .to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "or <insert reason here>, and we all know how bad that could be!"
                            .to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value:
                            "Use A/D to move the left tractor, and J/L to move the right tractor."
                                .to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(TextBundle {
                style: Style {
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(0.),
                        bottom: Val::Px(20.),
                    },
                    ..Default::default()
                },
                text: Text {
                    sections: vec![TextSection {
                        value: "Hit the space bar or play below to start.".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 20.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            });

            node.spawn_bundle(ButtonBundle {
                style: Style {
                    size: Size::new(Val::Px(120.0), Val::Px(50.0)),
                    margin: Rect {
                        left: Val::Px(0.),
                        right: Val::Px(0.),
                        top: Val::Px(20.),
                        bottom: Val::Px(0.),
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: button_materials.normal.clone(),
                ..Default::default()
            })
            .insert(PlayButton)
            .with_children(|parent| {
                parent.spawn_bundle(TextBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: "Play".to_string(),
                            style: TextStyle {
                                font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::rgb(0.9, 0.9, 0.9),
                            },
                        }],
                        alignment: Default::default(),
                    },
                    ..Default::default()
                });
            });
        });
}

type ButtonInteraction<'a> = (
    Entity,
    &'a Interaction,
    &'a mut Handle<ColorMaterial>,
    &'a Children,
);

fn click_play_button(
    actions: Res<Actions>,
    button_materials: Res<ButtonMaterials>,
    mut state: ResMut<State<GameState>>,
    mut interaction_query: Query<ButtonInteraction, (Changed<Interaction>, With<Button>)>,
) {
    if actions.restart_requested {
        state.set(GameState::Playing).unwrap();
        return;
    }

    for (_, interaction, mut material, _) in interaction_query.iter_mut() {
        match *interaction {
            Interaction::Clicked => {
                state.set(GameState::Playing).unwrap();
            }
            Interaction::Hovered => {
                *material = button_materials.hovered.clone();
            }
            Interaction::None => {
                *material = button_materials.normal.clone();
            }
        }
    }
}

fn despawn_menu(mut commands: Commands, items: Query<Entity, With<MenuItem>>) {
    for item in items.iter() {
        commands.entity(item).despawn_recursive();
    }
}
