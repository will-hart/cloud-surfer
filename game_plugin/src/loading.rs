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
    pub collide: Handle<AudioSource>,
    pub collide_obstacle: Handle<AudioSource>,
    pub music: Handle<AudioSource>,
}

pub struct TextureAssets {
    pub cloud_001: Handle<Texture>,
    pub player_left: Handle<Texture>,
    pub player_right: Handle<Texture>,
    pub laser: Handle<Texture>,
    pub grass: Handle<Texture>,
}

fn start_loading(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut fonts: Vec<HandleUntyped> = vec![];
    fonts.push(asset_server.load_untyped(PATHS.fira_sans));

    let mut audio: Vec<HandleUntyped> = vec![];
    audio.push(asset_server.load_untyped(PATHS.audio_collect));
    audio.push(asset_server.load_untyped(PATHS.audio_collide));
    audio.push(asset_server.load_untyped(PATHS.audio_collide_obstacle));
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
}

fn check_state(
    mut commands: Commands,
    mut state: ResMut<State<GameState>>,
    asset_server: Res<AssetServer>,
    loading_state: Res<LoadingState>,
) {
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.fonts.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.textures.iter().map(|handle| handle.id))
    {
        return;
    }
    if LoadState::Loaded
        != asset_server.get_group_load_state(loading_state.audio.iter().map(|handle| handle.id))
    {
        return;
    }

    commands.insert_resource(FontAssets {
        fira_sans: asset_server.get_handle(PATHS.fira_sans),
    });

    commands.insert_resource(AudioAssets {
        collect: asset_server.get_handle(PATHS.audio_collect),
        collide: asset_server.get_handle(PATHS.audio_collide),
        collide_obstacle: asset_server.get_handle(PATHS.audio_collide_obstacle),
        music: asset_server.get_handle(PATHS.audio_music),
    });

    commands.insert_resource(TextureAssets {
        cloud_001: asset_server.get_handle(PATHS.cloud_001),
        player_left: asset_server.get_handle(PATHS.player_left),
        player_right: asset_server.get_handle(PATHS.player_right),
        laser: asset_server.get_handle(PATHS.laser),
        grass: asset_server.get_handle(PATHS.grass),
    });

    state.set(GameState::Menu).unwrap();
}
