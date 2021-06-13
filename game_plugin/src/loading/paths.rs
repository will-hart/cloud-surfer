pub struct AssetPaths {
    pub fira_sans: &'static str,
    pub audio_flying: &'static str,
    pub cloud_001: &'static str,
    pub player_left: &'static str,
    pub player_right: &'static str,
    pub laser: &'static str,
    pub grass: &'static str,
}

pub const PATHS: AssetPaths = AssetPaths {
    fira_sans: "fonts/FiraSans-Bold.ttf",
    audio_flying: "audio/flying.ogg",
    cloud_001: "textures/cloud_001.png",
    player_left: "textures/player_left.png",
    player_right: "textures/player_right.png",
    laser: "textures/laser.png",
    grass: "textures/grass.png",
};
