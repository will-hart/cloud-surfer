pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_collect: &'static str,
    pub audio_music: &'static str,
    pub audio_game_over: &'static str,
    pub cloud_001: &'static str,
    pub player_left: &'static str,
    pub player_right: &'static str,
    pub laser: &'static str,
    pub grass: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_collect: "audio/collect.ogg",
    audio_music: "audio/music.ogg",
    audio_game_over: "audio/game_over.ogg",
    cloud_001: "textures/cloud_001.png",
    player_left: "textures/player_left.png",
    player_right: "textures/player_right.png",
    laser: "textures/laser.png",
    grass: "textures/grass.png",
};
