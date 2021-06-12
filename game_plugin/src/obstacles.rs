use bevy::prelude::*;
use rand::{thread_rng, Rng};

use crate::{
    game_map::GameMap,
    loading::TextureAssets,
    player::{IsDead, Player, PlayerShip, PlayerShipSide},
    GameState,
};

/// Possible spawn patterns for obstacles, uses a "phone pad grid" to specify positions
pub struct SpawnPattern(Vec<u8>);

pub struct AvailableSpawnPatterns {
    pub patterns: Vec<SpawnPattern>,
}

impl AvailableSpawnPatterns {
    fn new() -> Self {
        AvailableSpawnPatterns {
            patterns: vec![SpawnPattern(vec![5])],
        }
    }
}

pub struct Obstacle;

pub struct SpawnTimer;

pub struct ObstaclePlugin;

impl Plugin for ObstaclePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(AvailableSpawnPatterns::new())
            .add_system_set(
                SystemSet::on_enter(GameState::Playing)
                    .with_system(setup_obstacle_spawning.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::Playing)
                    .with_system(spawn_obstacles.system().label("spawn_obstacles"))
                    .with_system(
                        move_obstacles
                            .system()
                            .label("move_obstacles")
                            .after("spawn_obstacles"),
                    )
                    .with_system(
                        check_for_obstacle_collisions
                            .system()
                            .after("move_obstacles"),
                    )
                    .with_system(remove_dead_obstacles.system().after("move_obstacles")),
            );
    }
}

/// Starts the obstacle spawn timer
pub fn setup_obstacle_spawning(mut commands: Commands) {
    commands
        .spawn()
        .insert(SpawnTimer)
        .insert(Timer::from_seconds(3., true));
}

/// Spawns obstacles at the top of the screen
pub fn spawn_obstacles(
    mut commands: Commands,
    time: Res<Time>,
    textures: Res<TextureAssets>,
    game_map: Res<GameMap>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timers: Query<&mut Timer, With<SpawnTimer>>,
    _patterns: Res<AvailableSpawnPatterns>,
) {
    let mut timer = timers.single_mut().unwrap();
    timer.tick(time.delta());
    if !timer.just_finished() {
        return;
    }

    let mut rng = thread_rng();
    let x_extents = -(game_map.width / 2.)..=(game_map.width / 2.);

    println!("Spawning obstacle");
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.add(textures.cloud_001.clone().into()),
            transform: Transform::from_translation(Vec3::new(
                rng.gen_range(x_extents).floor() * game_map.sprite_size,
                game_map.top_y() + game_map.pad_y * 3. * game_map.sprite_size, // spawn out of sight
                1.,
            )),
            sprite: Sprite {
                size: Vec2::new(32., 32.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Obstacle);
}

/// Moves the obstacles down towards the player
pub fn move_obstacles(
    time: Res<Time>,
    ship: Res<PlayerShip>,
    mut obstacles: Query<&mut Transform, With<Obstacle>>,
) {
    if ship.is_dead {
        return;
    }

    let dt = time.delta_seconds();
    for mut tx in obstacles.iter_mut() {
        tx.translation.y -= 150. * dt;
    }
}

/// Compares player and ship positions and kills the ship if it hits an obstacle
pub fn check_for_obstacle_collisions(
    mut commands: Commands,
    game_map: Res<GameMap>,
    mut ship: ResMut<PlayerShip>,
    players: Query<Entity, (With<Player>, Without<IsDead>)>,
    obstacles: Query<&Transform, With<Obstacle>>,
    ship_sides: Query<&Transform, With<PlayerShipSide>>,
) {
    if ship.is_dead {
        return;
    }

    for obs_tx in obstacles.iter() {
        for ship_tx in ship_sides.iter() {
            let dx = ship_tx.translation.x - obs_tx.translation.x;

            if dx.abs() < 0.9 * game_map.sprite_size {
                let dy = ship_tx.translation.y - obs_tx.translation.y;

                if dy.abs() < 0.9 * game_map.sprite_size {
                    println!("hit obstacle");

                    commands.entity(players.single().unwrap()).insert(IsDead);
                    ship.is_dead = true;

                    return;
                }
            }
        }
    }
}

/// removes dead obstacles that are off the map
pub fn remove_dead_obstacles(
    game_map: Res<GameMap>,
    mut commands: Commands,
    mut obstacles: Query<(&mut Transform, Entity), With<Obstacle>>,
) {
    let min_y = game_map.bottom_y() - 3. * game_map.pad_y * game_map.sprite_size;

    for (tx, entity) in obstacles.iter_mut() {
        if tx.translation.y < min_y {
            println!("Destroying obstacle");
            commands.entity(entity).despawn();
        }
    }
}
