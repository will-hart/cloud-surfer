use bevy::prelude::*;

use crate::{game_map::GameMap, game_time::GameTime, loading::TextureAssets, GameState};

pub struct ScrollingBackground;

pub struct ScrollingBackgroundPlugin;

impl Plugin for ScrollingBackgroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing).with_system(spawn_background.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing).with_system(animate_background.system()),
        )
        .add_system_set(
            SystemSet::on_exit(GameState::Playing).with_system(despawn_background.system()),
        );
    }
}

fn spawn_background(
    mut commands: Commands,
    game_map: Res<GameMap>,
    textures: Res<TextureAssets>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_atlas =
        TextureAtlas::from_grid(textures.grass.clone(), Vec2::new(32., 32.0), 15, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let w = game_map.width + 2. * game_map.pad_x;
    let h = game_map.height + 2. * game_map.pad_y;

    let offset = Vec3::new(
        game_map.sprite_size / 2. - w * game_map.sprite_size / 2.,
        game_map.sprite_size / 2. - h * game_map.sprite_size / 2.,
        0.,
    );

    let w = w as u32;
    let h = h as u32;

    for x in 0..w {
        for y in 0..h {
            commands
                .spawn_bundle(SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle.clone(),
                    transform: Transform::from_translation(
                        offset
                            + Vec3::new(
                                x as f32 * game_map.sprite_size,
                                y as f32 * game_map.sprite_size,
                                0.,
                            ),
                    ),
                    ..Default::default()
                })
                .insert(ScrollingBackground);
        }
    }
}

fn animate_background(
    game_time: Res<GameTime>,
    mut tiles: Query<&mut TextureAtlasSprite, With<ScrollingBackground>>,
) {
    if !game_time.fixed_update {
        return;
    }

    for mut tile in tiles.iter_mut() {
        tile.index = (tile.index + 1) % 15
    }
}

fn despawn_background(mut commands: Commands, items: Query<Entity, With<ScrollingBackground>>) {
    for item in items.iter() {
        commands.entity(item).despawn_recursive();
    }
}
