pub struct GameMap {
    pub width: u8,
    pub height: u8,
    pub pad_x: u8,
    pub pad_y: u8,
    pub sprite_size: f32,
}

impl Default for GameMap {
    fn default() -> Self {
        GameMap {
            width: 16,
            height: 16,
            pad_x: 3,
            pad_y: 1,
            sprite_size: 32.,
        }
    }
}

impl GameMap {
    /// Converts an x_index to a world coordinate
    pub fn idx_to_x(&self, x_idx: u8) -> f32 {
        (x_idx as i8 - (self.width / 2) as i8) as f32 * self.sprite_size
    }

    /// Determines the "bottom" of the map in world coordinates
    pub fn bottom_y(&self) -> f32 {
        -((self.height / 2) as f32) * self.sprite_size
    }

    /// Determines the "top" of the map in world coordinates
    pub fn top_y(&self) -> f32 {
        (self.height / 2) as f32 * self.sprite_size
    }
}
