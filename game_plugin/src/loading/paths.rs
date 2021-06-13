pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_collect: &'static str,
    pub audio_collide: &'static str,
    pub audio_collide_obstacle: &'static str,
    pub audio_menu_music: &'static str,
    pub audio_music: &'static str,
    pub cloud_001: &'static str,
    pub player_left: &'static str,
    pub player_right: &'static str,
    pub laser: &'static str,
    pub grass: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_collect: "audio/collect.ogg",
    audio_collide: "audio/collide.ogg",
    audio_collide_obstacle: "audio/collide_obstacle.ogg",
    audio_menu_music: "audio/menu_music.ogg",
    audio_music: "audio/music.ogg",
    cloud_001: "textures/cloud_001.png",
    player_left: "textures/player_left.png",
    player_right: "textures/player_right.png",
    laser: "textures/laser.png",
    grass: "textures/grass.png",
};
