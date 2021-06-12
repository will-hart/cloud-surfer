pub struct GameMap {
    pub width: f32,
    pub height: f32,
    pub pad_x: f32,
    pub pad_y: f32,
    pub sprite_size: f32,
}

impl Default for GameMap {
    fn default() -> Self {
        GameMap {
            width: 16.,
            height: 16.,
            pad_x: 3.,
            pad_y: 1.,
            sprite_size: 32.,
        }
    }
}

impl GameMap {
    /// Determines the "bottom" of the map in world coordinates
    pub fn bottom_y(&self) -> f32 {
        -(self.height / 2.) * self.sprite_size
    }

    /// Determines the "top" of the map in world coordinates
    pub fn top_y(&self) -> f32 {
        (self.height / 2.) * self.sprite_size
    }
}
