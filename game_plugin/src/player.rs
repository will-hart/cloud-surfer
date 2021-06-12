use bevy::prelude::*;

use crate::actions::Actions;
use crate::game_map::GameMap;
use crate::loading::TextureAssets;
use crate::GameState;

pub struct PlayerPlugin;

pub struct Player;

pub struct IsDead;

pub struct Laser;

#[derive(Debug, Copy, Clone)]
pub struct PlayerShip {
    pub is_dead: bool,
    pub speed: f32,

    pub max_separation: f32,
    pub separation_strain: f32,

    pub target_left: f32,
    pub target_right: f32,
}

pub enum PlayerShipSide {
    Left,
    Right,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(GameState::Playing)
                .with_system(spawn_player.system())
                .with_system(spawn_camera.system()),
        )
        .add_system_set(
            SystemSet::on_update(GameState::Playing)
                .with_system(move_player.system().label("move_player"))
                .with_system(is_player_dead_checks.system().after("move_player"))
                .with_system(update_laser.system().after("move_player")),
        )
        .add_system_set(SystemSet::on_exit(GameState::Playing).with_system(despawn_level.system()));
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}

fn spawn_player(
    mut commands: Commands,
    textures: Res<TextureAssets>,
    game_map: Res<GameMap>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning player");

    let ship = PlayerShip {
        is_dead: false,
        speed: 150.,

        max_separation: 3. * game_map.sprite_size,
        separation_strain: 0.,

        target_left: -game_map.sprite_size,
        target_right: game_map.sprite_size,
    };

    commands.insert_resource(ship);

    // spawn the player + player ships
    commands
        .spawn()
        .insert(Transform::from_translation(Vec3::ZERO))
        .insert(GlobalTransform::from_translation(Vec3::ZERO))
        .insert(Player)
        .with_children(|parent| {
            parent
                .spawn_bundle({
                    SpriteBundle {
                        material: materials.add(textures.player_left.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            ship.target_left,
                            game_map.bottom_y(),
                            1.,
                        )),
                        sprite: Sprite {
                            size: Vec2::new(32., 32.),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .insert(PlayerShipSide::Left);

            parent
                .spawn_bundle({
                    SpriteBundle {
                        material: materials.add(textures.player_right.clone().into()),
                        transform: Transform::from_translation(Vec3::new(
                            ship.target_right,
                            game_map.bottom_y(),
                            1.,
                        )),
                        sprite: Sprite {
                            size: Vec2::new(32., 32.),
                            ..Default::default()
                        },
                        ..Default::default()
                    }
                })
                .insert(PlayerShipSide::Right);
        });

    // spawn the laser texture atlas
    let texture_atlas = TextureAtlas::from_grid(textures.laser.clone(), Vec2::new(32., 16.0), 3, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // spawn the laser between the two ships
    let mut laser_tx = Transform::from_translation(Vec3::new(
        (ship.target_left + ship.target_right) / 2.,
        game_map.bottom_y(),
        0.5,
    ));

    laser_tx.scale.x = 1.4;

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: laser_tx,
            ..Default::default()
        })
        .insert(Laser)
        .insert(Timer::from_seconds(0.2, true));
}

/// Moves a player based on input towards their target position
fn move_player(
    time: Res<Time>,
    actions: Res<Actions>,
    game_map: Res<GameMap>,
    mut ship: ResMut<PlayerShip>,
    mut ship_side_tx_query: Query<(&mut Transform, &PlayerShipSide)>,
) {
    // if we don't have a player, don't move
    if ship.is_dead {
        return;
    }

    ship.target_left = ship.target_left + (actions.player_left_move as f32) * game_map.sprite_size;
    ship.target_right =
        ship.target_right + (actions.player_right_move as f32) * game_map.sprite_size;

    // update the ship side positions - remember that there is a rope connecting them, so we must also calculate
    // the total applied movement and update the strain on the rope
    let elapsed = time.delta_seconds();
    for (mut tx, side) in ship_side_tx_query.iter_mut() {
        tx.translation = match side {
            PlayerShipSide::Left => {
                get_ship_position(elapsed * ship.speed, &tx.translation, ship.target_left)
            }
            PlayerShipSide::Right => {
                get_ship_position(elapsed * ship.speed, &tx.translation, ship.target_right)
            }
        };
    }
}

/// Helper function which moves a ship towards its target position
fn get_ship_position(normalised_speed: f32, pos: &Vec3, target: f32) -> Vec3 {
    if pos.x == target {
        // in position
        pos.clone()
    } else {
        // move the ship towards its preferred location
        let delta_x = target - pos.x;
        let max_move = delta_x.signum() * normalised_speed;

        Vec3::new(
            pos.x
                + if delta_x < 0. {
                    delta_x.clamp(max_move, 0.)
                } else {
                    delta_x.clamp(0., max_move)
                },
            pos.y,
            pos.z,
        )
    }
}

/// check if a player is ded
pub fn is_player_dead_checks(
    mut commands: Commands,
    game_map: Res<GameMap>,
    mut ship: ResMut<PlayerShip>,
    players: Query<Entity, (With<Player>, Without<IsDead>)>,
    ship_side_tx_query: Query<&Transform, With<PlayerShipSide>>,
) {
    match players.single() {
        Ok(player) => {
            // first check if the players bash into each other
            let diff = ship_side_tx_query
                .iter()
                .fold(0., |acc, tx| tx.translation.x - acc);

            if diff.abs() < game_map.sprite_size * 0.75 {
                println!("Bashed into each other!");
                ship.is_dead = true;
                commands.entity(player).insert(IsDead);
            } else if ship.separation_strain > 3. {
                println!("Tether broke!");
                ship.is_dead = true;
                commands.entity(player).insert(IsDead);
            }
        }
        Err(_) => {}
    }
}

/// Draws and animates "laser" between the two ships
fn update_laser(
    time: Res<Time>,
    game_map: Res<GameMap>,
    mut ship: ResMut<PlayerShip>,
    mut lasers: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Timer), With<Laser>>,
    mut ship_sides: Query<(&Transform, &PlayerShipSide), Without<Laser>>,
) {
    if ship.is_dead {
        return;
    }

    let sides =
        ship_sides
            .iter_mut()
            .fold((Vec3::ZERO, Vec3::ZERO), |acc, (tx, side)| match side {
                PlayerShipSide::Left => (tx.translation, acc.1),
                PlayerShipSide::Right => (acc.0, tx.translation),
            });

    // stretch the laser to fit between the two sides
    let dx = sides.1.x - sides.0.x;
    let x_scale = 0.4 + dx / game_map.sprite_size;

    // update separation strain
    if dx > ship.max_separation {
        // increase strain
        ship.separation_strain += 0.3 * time.delta_seconds();
    } else {
        // reduce strain
        if ship.separation_strain > 0. {
            ship.separation_strain = (ship.separation_strain - time.delta_seconds()).clamp(0., 1.);
        }
    }

    for (mut laser, mut sprite, mut timer) in lasers.iter_mut() {
        // reposition the laser
        laser.scale.x = x_scale;
        laser.translation.x = sides.0.x + dx / 2.;

        // update the animation frame for the laser
        // show flickering if stress > 0.5
        timer.tick(time.delta());
        if timer.just_finished() {
            let frame_count = if dx > ship.max_separation {
                if ship.separation_strain > 0.5 {
                    4
                } else {
                    3
                }
            } else {
                2
            };
            sprite.index = (sprite.index + 1) % frame_count;
        }
    }
}

/// Despawns the player and related objects
fn despawn_level(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
    lasers: Query<Entity, With<Laser>>,
) {
    for player in players.iter() {
        commands.entity(player).despawn_recursive();
    }

    for laser in lasers.iter() {
        commands.entity(laser).despawn_recursive();
    }
}
