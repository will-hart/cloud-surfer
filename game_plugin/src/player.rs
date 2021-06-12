use crate::actions::Actions;
use crate::game_map::GameMap;
use crate::loading::TextureAssets;
use crate::GameState;
use bevy::prelude::*;

pub struct PlayerPlugin;

pub struct Player;

pub struct IsDead;

#[derive(Debug, Copy, Clone)]
pub struct PlayerShip {
    pub is_dead: bool,
    pub speed: f32,
    // TODO: stop movement when the ships are too far apart: pub max_separation: f32,
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
                .with_system(is_player_dead_checks.system().after("move_player")),
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
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    println!("Spawning player");

    let ship = PlayerShip {
        is_dead: false,
        speed: 150.,
        // max_separation: 3. * game_map.sprite_size,
        target_left: -game_map.sprite_size,
        target_right: game_map.sprite_size,
    };

    commands.insert_resource(ship);

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

    // update the ship side positions
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

            if diff.abs() < game_map.sprite_size * 0.9 {
                println!("Bashed into each other!");
                ship.is_dead = true;
                commands.entity(player).insert(IsDead);
            }
        }
        Err(_) => {}
    }
}

fn despawn_level(mut commands: Commands, players: Query<Entity, With<Player>>) {
    for player in players.iter() {
        commands.entity(player).despawn_recursive();
    }
}
