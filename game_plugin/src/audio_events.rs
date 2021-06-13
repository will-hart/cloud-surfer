use bevy::prelude::*;
use bevy_kira_audio::Audio;

use crate::{audio::AudioChannels, loading::AudioAssets, GameState};

pub enum AudioEffect {
    Collect,
}

pub struct PlayAudioEffectEvent(pub AudioEffect);

pub struct AudioEventsPlugin;

impl Plugin for AudioEventsPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_event::<PlayAudioEffectEvent>()
            .add_system_set(
                SystemSet::on_update(GameState::Playing).with_system(handle_audio_events.system()),
            )
            .add_system_set(
                SystemSet::on_enter(GameState::GameOver).with_system(play_game_over_sound.system()),
            );
    }
}

fn handle_audio_events(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
    mut audio_events: EventReader<PlayAudioEffectEvent>,
) {
    let mut has_played_collect = false;

    for ev in audio_events.iter() {
        match ev.0 {
            AudioEffect::Collect => {
                if !has_played_collect {
                    has_played_collect = true;
                    audio.play_in_channel(audio_assets.collect.clone(), &channels.effects)
                }
            }
        }
    }
}

fn play_game_over_sound(
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    channels: Res<AudioChannels>,
) {
    println!("Playing game over sounds");
    audio.play_in_channel(audio_assets.tether_break.clone(), &channels.effects)
}
