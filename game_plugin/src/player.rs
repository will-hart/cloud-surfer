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
    mut ship_sides: Query<(&mut Transform, &PlayerShipSide)>,
) {
    // if we don't have a player, don't move
    if ship.is_dead {
        return;
    }

    // update the target movement positions
    ship.target_left = ship.target_left + (actions.player_left_move as f32) * game_map.sprite_size;
    ship.target_right =
        ship.target_right + (actions.player_right_move as f32) * game_map.sprite_size;

    // check the current x-difference between the two ships
    let positions = ship_sides
        .iter_mut()
        .fold((Vec3::ZERO, Vec3::ZERO), |acc, (tx, side)| match side {
            PlayerShipSide::Left => (tx.translation.clone(), acc.1),
            PlayerShipSide::Right => (acc.0, tx.translation.clone()),
        });
    let dx = positions.1.x - positions.0.x;

    // calculate the desired movement per ship

    let move_deltas = (
        ship.target_left - positions.0.x,
        ship.target_right - positions.1.x,
    );

    let move_signs = (move_deltas.0.signum(), move_deltas.1.signum());

    let mut move_inputs = (
        move_deltas
            .0
            .abs()
            .clamp(0., move_deltas.0 * ship.speed * time.delta_seconds())
            * move_signs.0,
        move_deltas
            .1
            .abs()
            .clamp(0., move_deltas.1 * ship.speed * time.delta_seconds())
            * move_signs.1,
    );

    // this is going to be a bit of a mouthful. Essentially what we want to do is tether the two
    // ships together. So prevent them moving more than ship.max_separation apart, and let ships
    // drag each other sideways once they reach max distance (assuming one is moving and one is static).
    // If the ships are at max separation, then we need to start increasing the strain on the tether.
    // (overstraining the tether is a loss condition).

    let (lm, rm) = move_inputs;

    let approx_zero = 0.0001;

    // condition 1: both ships are within max separation distance and can move freely,
    //              OR ships are at max separation distance but moving together or not moving
    if dx < ship.max_separation || (lm - rm).abs() < approx_zero {
        // do not modify movement inputs
    } else {
        // condition 2: ships are at max separation and have different movement values
        //              either one is stationary and one moving, or they're moving in opposite directions

        // condition 2a, the signs are not matched (trying to move in opposite directions)
        if lm.signum() != rm.signum() {
            // movement is blocked
            move_inputs = (0., 0.);
        } else {
            // condition 2b: only one is moving, apply movement to both
            // apply the "largest" movement to both ships
            if move_inputs.0.abs() > move_inputs.1.abs() {
                move_inputs = (move_inputs.0, move_inputs.0);
            } else {
                move_inputs = (move_inputs.1, move_inputs.1);
            }
        }
    }

    // now apply the movement
    for (mut tx, side) in ship_sides.iter_mut() {
        match side {
            PlayerShipSide::Left => tx.translation.x += move_inputs.0,
            PlayerShipSide::Right => tx.translation.x += move_inputs.1,
        }
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

            if diff.abs() < game_map.sprite_size * 0.9 {
                println!("Bashed into each other!");
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
    ship: Res<PlayerShip>,
    mut lasers: Query<(&mut Transform, &mut TextureAtlasSprite, &mut Timer), With<Laser>>,
    mut ship_sides: Query<&Transform, (With<PlayerShipSide>, Without<Laser>)>,
) {
    if ship.is_dead {
        return;
    }

    let mut sides = ship_sides
        .iter_mut()
        .map(|tx| tx.translation.clone())
        .collect::<Vec<_>>();

    sides.sort_by_key(|s| (s.x * 1000.) as i32);

    // stretch the laser to fit between the two sides
    let dx = sides.iter().fold(0., |acc, tx| tx.x - acc);
    let x_scale = 0.4 + dx / game_map.sprite_size;

    for (mut laser, mut sprite, mut timer) in lasers.iter_mut() {
        // reposition the laser
        laser.scale.x = x_scale;
        laser.translation.x = sides[0].x + dx / 2.;

        // update the animation frame for the laser
        // show flickering if stress > 0.5
        timer.tick(time.delta());
        if timer.just_finished() {
            let frame_count = if ship.separation_strain > 0.5 { 3 } else { 2 };
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
