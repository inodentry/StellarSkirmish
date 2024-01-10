use crate::components::*;
use crate::events::{SpawnGuidedMissileEvent, SpawnMineEvent};
use crate::ship_parts::*;
use crate::ships::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_player_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0)
                .with_scale(GLOBAL_RESCALE_V),
            texture: asset_server.load("sprites/ships/playerShip1_blue.png"),
            ..default()
        },
        Player {},
        Ship {
            turn_speed: f32::to_radians(1.25),
            primary_weapon: load_basic_torpedo(),
            secondary_weapon: load_basic_laser(),
            tertiary_weapon: load_basic_cannon(),
        },
        Velocity {
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
        load_basic_thruster(),
        Clipping {
            cd_timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
        Drag {
            dampening_factor: 0.995,
        },
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 500.0 },
        Mass { value: 100000.0 },
        EntityType::Ship,
    ));
}

pub fn spawn_ships_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    level_data: Res<LevelScript>,
    q_window: Query<&Window, With<PrimaryWindow>>,
) {
    let mut rng = thread_rng();
    let window = q_window.get_single().unwrap();
    level_data.txt.as_str().trim().lines().for_each(|line| {
        // Read the data in the line to determine what ship to spawn and where.
        let level_vec = line.split(',').collect::<Vec<&str>>();
        let ship_type = level_vec[0];
        let mut x = level_vec[1].parse::<f32>().unwrap();
        let mut y = level_vec[2].parse::<f32>().unwrap();

        // -1.0 of x or y signifies that we want to randomize the coordinates.
        if x == -1.0 {
            x = rng.gen::<f32>() * window.width()
        }
        if y == -1.0 {
            y = rng.gen::<f32>() * window.height()
        }

        let ship_sprite_path = match ship_type {
            "picket" => "sprites/ships/picket.png".to_string(),
            "drone" => "sprites/ships/drone.png".to_string(),
            _ => "sprites/ships/turret.png".to_string(),
        };

        match ship_type {
            "picket" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_picket_ship(),
                ));
            }
            "drone" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_drone_ship(),
                ));
            }
            "speedy" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_speedy_ship(),
                ));
            }
            "lunker" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_lunker_ship(),
                ));
            }
            "mine_layer" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_minelayer_ship(),
                ));
            }
            "rammer" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_rammer_ship(),
                ));
            }
            "turret" => {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform::from_xyz(x, y, 0.0).with_scale(GLOBAL_RESCALE_V),
                        texture: asset_server.load(ship_sprite_path),
                        ..default()
                    },
                    Velocity {
                        velocity: Vec3 {
                            x: 0.0,
                            y: 0.0,
                            z: 0.0,
                        },
                    },
                    load_turret_ship(),
                ));
            }
            _ => {
                println!("ship_type not recognized!")
            }
        }
    });
}

pub fn spawn_asteroid_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = thread_rng();
    for _ in 0..30 {
        let random_x = rng.gen::<f32>() * window.width();
        let random_y = rng.gen::<f32>() * window.height();
        let asteroid_rescaler = rng.gen::<f32>() * 2.0;

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0)
                    .with_scale(GLOBAL_RESCALE_V * asteroid_rescaler),
                texture: asset_server.load("sprites/environmental/meteorGrey_big1.png"),
                ..default()
            },
            Asteroid {},
            Clipping {
                cd_timer: Timer::from_seconds(0.15, TimerMode::Once),
            },
            CollisionBox {
                shape: Shape::Circle,
                width_radius: 42.0 * GLOBAL_RESCALE_C * asteroid_rescaler,
                height: 42.0 * GLOBAL_RESCALE_C * asteroid_rescaler,
            },
            Health { value: 10.0 },
            Mass {
                value: 10000.0 * asteroid_rescaler,
            },
            Velocity {
                velocity: Vec3 {
                    x: rng.gen::<f32>() * 0.1,
                    y: rng.gen::<f32>() * 0.1,
                    z: 0.0,
                },
            },
            EntityType::Asteroid,
        ));
    }
}

pub fn despawn_dead_system(
    mut commands: Commands,
    entity_query: Query<(Entity, &Health, &EntityType)>,
    asset_server: Res<AssetServer>,
) {
    for (entity, health, et) in entity_query.iter() {
        // If an entity's health has dropped to or below 0, despawn it.
        if health.value <= 0.0 {
            if *et == EntityType::Ship || *et == EntityType::Missile {
                commands.spawn(AudioBundle {
                    source: asset_server.load("sounds/explosionCrunch_003.ogg"),
                    ..default()
                });
            }
            commands.entity(entity).despawn();
        }
    }
}

pub fn handle_self_destruct_system(
    mut commands: Commands,
    mut entity_query: Query<(Entity, &mut SelfDestruct), With<SelfDestruct>>,
) {
    for (entity, mut self_destruct) in entity_query.iter_mut() {
        if self_destruct.cd_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

pub fn setup_background_stars_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    let win = window_query.get_single().unwrap();
    let mut rng = thread_rng();
    for _ in 0..30 {
        commands.spawn(
            (SpriteBundle {
                transform: Transform::from_xyz(
                    rng.gen::<f32>() * win.width(),
                    rng.gen::<f32>() * win.height(),
                    -1.0,
                )
                .with_scale(GLOBAL_RESCALE_V),
                texture: asset_server.load("sprites/effects/star2.png"),
                ..default()
            }),
        );
    }
}

pub fn spawn_missile_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_reader: EventReader<SpawnGuidedMissileEvent>,
) {
    for ev in spawn_reader.read() {
        commands.spawn((
            SpriteBundle {
                transform: ev.transform,
                texture: asset_server.load("sprites/projectiles/guided_missile_red.png"),
                ..default()
            },
            Clipping {
                cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
            CollisionBox {
                shape: Shape::Circle,
                width_radius: 30.0 * GLOBAL_RESCALE_C,
                height: 30.0 * GLOBAL_RESCALE_C,
            },
            Health { value: 10.0 },
            Mass { value: 100.0 },
            Drag {
                dampening_factor: 0.995,
            },
            Velocity {
                velocity: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            load_missile_thruster(),
            Missile {
                turn_speed: 0.10,
                fuel: 800.0,
            },
            EntityType::Missile,
        ));
    }
}

pub fn spawn_mine_system(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut spawn_reader: EventReader<SpawnMineEvent>,
) {
    for ev in spawn_reader.read() {
        commands.spawn((
            SpriteBundle {
                transform: ev.transform.with_scale(GLOBAL_RESCALE_V * 0.75),
                texture: asset_server.load("sprites/projectiles/mine.png"),
                ..default()
            },
            Clipping {
                cd_timer: Timer::from_seconds(0.0, TimerMode::Once),
            },
            CollisionBox {
                shape: Shape::Circle,
                width_radius: 20.0 * GLOBAL_RESCALE_C,
                height: 20.0 * GLOBAL_RESCALE_C,
            },
            Health { value: 10.0 },
            Mass { value: 100.0 },
            Drag {
                dampening_factor: 0.995,
            },
            Velocity {
                velocity: Vec3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
            },
            Missile {
                turn_speed: 0.0,
                fuel: 0.0,
            },
            EntityType::Missile,
        ));
    }
}
