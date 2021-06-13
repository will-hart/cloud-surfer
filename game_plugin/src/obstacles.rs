use bevy::prelude::*;
use rand::{seq::SliceRandom, thread_rng, Rng};

use crate::{
    game_map::GameMap,
    game_time::GameTime,
    loading::TextureAssets,
    player::{IsDead, Player, PlayerShip, PlayerShipSide},
    score::Score,
    GameState, SystemLabels,
};

/// Possible spawn patterns for obstacles, specified as sprite sized offsets from the main
#[derive(Clone)]
pub struct SpawnPattern {
    pub offsets: Vec<Vec2>,
    pub min_score: f32,
}

pub struct AvailableSpawnPatterns {
    pub patterns: Vec<SpawnPattern>,
}

impl AvailableSpawnPatterns {
    fn new() -> Self {
        AvailableSpawnPatterns {
            patterns: vec![
                SpawnPattern {
                    offsets: vec![Vec2::ZERO],
                    min_score: -1.,
                },
                SpawnPattern {
                    offsets: vec![Vec2::new(-1., 0.), Vec2::ZERO, Vec2::new(1., 0.)],
                    min_score: -1.,
                },
            ],
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
                    .with_system(
                        spawn_obstacles
                            .system()
                            .label(SystemLabels::SpawnObstacles)
                            .after(SystemLabels::UpdateTime),
                    )
                    .with_system(
                        move_obstacles
                            .system()
                            .label(SystemLabels::MoveObstacles)
                            .after(SystemLabels::SpawnObstacles),
                    )
                    .with_system(
                        check_for_obstacle_collisions
                            .system()
                            .after(SystemLabels::MoveObstacles),
                    )
                    .with_system(
                        remove_dead_obstacles
                            .system()
                            .after(SystemLabels::MoveObstacles),
                    ),
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Playing).with_system(despawn_obstacles.system()),
            );
    }
}

/// Starts the obstacle spawn timer
fn setup_obstacle_spawning(mut commands: Commands) {
    commands
        .spawn()
        .insert(SpawnTimer)
        .insert(Timer::from_seconds(3., true));
}

/// Spawns obstacles at the top of the screen
fn spawn_obstacles(
    mut commands: Commands,
    time: Res<GameTime>,
    ship: Res<PlayerShip>,
    textures: Res<TextureAssets>,
    game_map: Res<GameMap>,
    patterns: Res<AvailableSpawnPatterns>,
    score: Res<Score>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut timers: Query<&mut Timer, With<SpawnTimer>>,
) {
    if ship.is_dead {
        return;
    }

    let mut timer = timers.single_mut().unwrap();
    timer.tick(time.delta_duration);
    if !timer.just_finished() {
        return;
    }

    let mut rng = thread_rng();
    let x_extents = -(game_map.width / 2.)..=(game_map.width / 2.);

    let spawn_x = rng.gen_range(x_extents).floor() * game_map.sprite_size;
    let spawn_patterns = patterns
        .patterns
        .iter()
        .filter_map(|pattern| {
            if pattern.min_score < score.current {
                Some(pattern.clone())
            } else {
                None
            }
        })
        .collect::<Vec<_>>();
    let spawn_pattern = spawn_patterns.choose(&mut rng).unwrap();

    println!("Spawning obstacle");
    let material = materials.add(textures.cloud_001.clone().into());

    for offset in spawn_pattern.offsets.iter() {
        commands
            .spawn_bundle(SpriteBundle {
                material: material.clone(),
                transform: Transform::from_translation(Vec3::new(
                    spawn_x + offset.x * game_map.sprite_size,
                    game_map.top_y()
                        + game_map.pad_y * 3. * game_map.sprite_size
                        + offset.y * game_map.sprite_size, // spawn out of sight
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
}

/// Moves the obstacles down towards the player
fn move_obstacles(
    time: Res<GameTime>,
    ship: Res<PlayerShip>,
    mut obstacles: Query<&mut Transform, With<Obstacle>>,
) {
    if ship.is_dead {
        return;
    }

    for mut tx in obstacles.iter_mut() {
        tx.translation.y -= 150. * time.delta;
    }
}

/// Compares player and ship positions and kills the ship if it hits an obstacle
fn check_for_obstacle_collisions(
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
fn remove_dead_obstacles(
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

/// despawns all obstacles and the spawn timer
fn despawn_obstacles(
    mut commands: Commands,
    obstacles: Query<Entity, With<Obstacle>>,
    spawn_timers: Query<Entity, With<SpawnTimer>>,
) {
    for timer in spawn_timers.iter() {
        commands.entity(timer).despawn();
    }

    for obstacle in obstacles.iter() {
        commands.entity(obstacle).despawn();
    }
}
