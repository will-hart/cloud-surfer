use crate::loading::AudioAssets;
use crate::GameState;
use bevy::prelude::*;
use bevy_kira_audio::{Audio, AudioChannel, AudioPlugin};

pub struct InternalAudioPlugin;

impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AudioChannels {
            effects: AudioChannel::new("effects".to_owned()),
            music: AudioChannel::new("music".to_owned()),
        })
        .add_plugin(AudioPlugin)
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(play_menu_music.system()))
        .add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(play_game_music.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(stop_game_music.system()),
        );
    }
}

struct AudioChannels {
    effects: AudioChannel,
    music: AudioChannel,
}

fn play_menu_music(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    audio.stop_channel(&channels.music);
    audio.set_volume_in_channel(0.5, &channels.music);
    audio.play_looped_in_channel(audio_assets.menu_music.clone(), &channels.music);
}

fn play_game_music(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    audio.set_volume_in_channel(0.4, &channels.music);
    audio.stop_channel(&channels.music);
    audio.play_looped_in_channel(audio_assets.music.clone(), &channels.music);
}

fn stop_game_music(audio: Res<Audio>, channels: Res<AudioChannels>) {
    audio.stop_channel(&channels.effects);
    audio.stop_channel(&channels.music);
}
