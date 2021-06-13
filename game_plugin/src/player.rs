use bevy::prelude::*;

use crate::actions::Actions;
use crate::game_map::GameMap;
use crate::game_time::GameTime;
use crate::loading::TextureAssets;
use crate::GameState;
use crate::SystemLabels;

macro_rules! by_side {
    ($side:expr, $left:expr, $right:expr) => {{
        match $side {
            PlayerShipSide::Left => $left,
            PlayerShipSide::Right => $right,
        }
    }};
}

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
                .with_system(
                    move_player
                        .system()
                        .label(SystemLabels::MovePlayer)
                        .after(SystemLabels::UpdateTime),
                )
                .with_system(
                    is_player_dead_checks
                        .system()
                        .after(SystemLabels::MovePlayer),
                )
                .with_system(update_laser.system().after(SystemLabels::MovePlayer)),
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

        max_separation: 5. * game_map.sprite_size,
        separation_strain: 0.,
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
                            -game_map.sprite_size / 2.,
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
                            game_map.sprite_size / 2.,
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
    let texture_atlas = TextureAtlas::from_grid(textures.laser.clone(), Vec2::new(32., 16.0), 5, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    // spawn the laser between the two ships
    let mut laser_tx = Transform::from_translation(Vec3::new(0., game_map.bottom_y(), 0.5));
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
    time: Res<GameTime>,
    actions: Res<Actions>,
    game_map: Res<GameMap>,
    ship: Res<PlayerShip>,
    mut ship_sides: Query<(&mut Transform, &PlayerShipSide)>,
) {
    // if we don't have a player, don't move
    if ship.is_dead {
        return;
    }

    // remove the multiplier from delta_move so the ship doesn't get faster over time
    let delta_move = ship.speed * 1.5 * (time.delta / time.multiplier);

    // calculate movement
    let moves = (actions.player_left_move, actions.player_right_move);
    let sides = get_ship_sides(&mut ship_sides);
    let x_bound = game_map.get_x_bound();
    let target_x = (
        (sides.0.x + (moves.0 as f32) * delta_move).clamp(-x_bound, x_bound),
        sides.1.x + (moves.1 as f32) * delta_move.clamp(-x_bound, x_bound),
    );

    let rotations = (
        if moves.0 == 0 {
            0.
        } else {
            moves.0.signum() as f32 * -std::f32::consts::FRAC_PI_8
        },
        if moves.1 == 0 {
            0.
        } else {
            moves.1.signum() as f32 * -std::f32::consts::FRAC_PI_8
        },
    );

    // update the ship side positions and rotations
    // TODO could make this a bit nicer with a macro
    for (mut tx, side) in ship_sides.iter_mut() {
        tx.translation.x = by_side!(side, target_x.0, target_x.1);
        tx.rotation = Quat::from_axis_angle(Vec3::Z, by_side!(side, rotations.0, rotations.1));
    }
}

/// Gest the ship sides from the PlayerShipSide query
fn get_ship_sides(ship_sides: &mut Query<(&mut Transform, &PlayerShipSide)>) -> (Vec3, Vec3) {
    ship_sides
        .iter_mut()
        .fold((Vec3::ZERO, Vec3::ZERO), |acc, (tx, side)| {
            by_side!(side, (tx.translation, acc.1), (acc.0, tx.translation))
        })
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
    time: Res<GameTime>,
    game_map: Res<GameMap>,
    mut ship: ResMut<PlayerShip>,
    mut lasers: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Timer), With<Laser>>,
    mut ship_sides: Query<(&Transform, &PlayerShipSide), Without<Laser>>,
) {
    if ship.is_dead {
        return;
    }

    let sides = ship_sides
        .iter_mut()
        .fold((Vec3::ZERO, Vec3::ZERO), |acc, (tx, side)| {
            by_side!(side, (tx.translation, acc.1), (acc.0, tx.translation))
        });

    // stretch the laser to fit between the two sides
    let dx = sides.1.x - sides.0.x;
    let x_scale = 0.4 + dx / game_map.sprite_size;

    // update separation strain
    if dx > ship.max_separation {
        // increase strain
        ship.separation_strain += 0.3 * time.delta;
    } else {
        // reduce strain
        if ship.separation_strain > 0. {
            ship.separation_strain = (ship.separation_strain - time.delta).clamp(0., 1.);
        }
    }

    for (mut laser, mut sprite, mut timer) in lasers.iter_mut() {
        // reposition the laser
        laser.scale.x = x_scale;
        laser.translation.x = sides.0.x + dx / 2.;

        // update the animation frame for the laser
        // show flickering if stress > 0.5
        timer.tick(time.delta_duration);
        if timer.just_finished() {
            let frame_count = if dx > ship.max_separation {
                if ship.separation_strain > 0.5 {
                    5
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
