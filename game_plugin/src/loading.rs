mod paths;

use crate::loading::paths::PATHS;
use crate::GameState;
use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy_kira_audio::AudioSource;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Loading).with_system(start_loading.system()),
        )
        .add_system_set(SystemSet::on_update(GameState::Loading).with_system(check_state.system()));
    }
}

pub struct LoadingState {
    textures: Vec<HandleUntyped>,
    fonts: Vec<HandleUntyped>,
    audio: Vec<HandleUntyped>,
}

pub struct FontAssets {
    pub fira_sans: Handle<Font>,
}

pub struct AudioAssets {
    pub collect: Handle<AudioSource>,
    pub music: Handle<AudioSource>,
    pub tether_break: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub cloud_001: Handle<Texture>,
    pub player_left: Handle<Texture>,
    pub player_right: Handle<Texture>,
    pub laser: Handle<Texture>,
    pub grass: Handle<Texture>,
}

pub struct LoadingItem;
pub struct LoadingText;

fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_collect));
    audio.push(asset_server.load_untyped(PATHS.audio_music));

    let mut textures: Vec<HandleUntyped> = vec![];
    textures.push(asset_server.load_untyped(PATHS.cloud_001));
    textures.push(asset_server.load_untyped(PATHS.player_left));
    textures.push(asset_server.load_untyped(PATHS.player_right));
    textures.push(asset_server.load_untyped(PATHS.laser));
    textures.push(asset_server.load_untyped(PATHS.grass));

    commands.insert_resource(LoadingState {
        textures,
        fonts,
        audio,
    });

    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.), Val::Percent(100.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                flex_direction: FlexDirection::ColumnReverse,
                ..Default::default()
            },
            material: materials.add(Color::BLACK.into()),
            ..Default::default()
        })
        .insert(LoadingItem)
        .with_children(|node| {
            node.spawn_bundle(TextBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: "Loading".to_string(),
                        style: TextStyle {
                            font: asset_server.get_handle("fonts/FiraSans-Bold.ttf"),
                            font_size: 30.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    }],
                    alignment: Default::default(),
                },
                ..Default::default()
            })
            .insert(LoadingText);
        });
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
    mut loading_text: Query<&mut Text, With<LoadingText>>,
    loading_items: Query<Entity, With<LoadingItem>>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        loading_text.single_mut().unwrap().sections[0].value = "Loading fonts...".into();
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        loading_text.single_mut().unwrap().sections[0].value = "Loading textures...".into();
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        loading_text.single_mut().unwrap().sections[0].value = "Loading audio...".into();
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        collect: asset_server.get_handle(PATHS.audio_collect),
        music: asset_server.get_handle(PATHS.audio_music),
        tether_break: asset_server.get_handle(PATHS.audio_game_over),
    });

    commands.insert_resource(TextureAssets {
        cloud_001: asset_server.get_handle(PATHS.cloud_001),
        player_left: asset_server.get_handle(PATHS.player_left),
        player_right: asset_server.get_handle(PATHS.player_right),
        laser: asset_server.get_handle(PATHS.laser),
        grass: asset_server.get_handle(PATHS.grass),
    });

    state.set(GameState::Menu).unwrap();

    for item in loading_items.iter() {
        commands.entity(item).despawn_recursive();
    }
}
