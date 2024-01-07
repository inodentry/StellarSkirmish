use crate::components::*;
use crate::ship_crafting::*;
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
            thrust: 100.0,
            angle: f32::to_radians(90.0),
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
        Clipping {
            cd_timer: Timer::from_seconds(0.0, TimerMode::Once),
        },
        Drag {},
        CollisionBox {
            shape: Shape::Circle,
            width_radius: 38.0 * GLOBAL_RESCALE_C,
            height: 38.0 * GLOBAL_RESCALE_C,
        },
        Health { value: 100.0 },
        Mass { value: 10.0 },
    ));
}

pub fn spawn_asteroid_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window = window_query.get_single().unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..20 {
        let random_x = rng.gen::<f32>() * window.width();
        let random_y = rng.gen::<f32>() * window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(random_x, random_y, 0.0)
                    .with_scale(GLOBAL_RESCALE_V),
                texture: asset_server.load("sprites/environmental/meteorGrey_big1.png"),
                ..default()
            },
            Asteroid {},
            Clipping {
                cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
            },
            CollisionBox {
                shape: Shape::Circle,
                width_radius: 42.0 * GLOBAL_RESCALE_C,
                height: 42.0 * GLOBAL_RESCALE_C,
            },
            Health { value: 10.0 },
            Mass { value: 25.0 },
            Velocity {
                velocity: Vec3 {
                    x: rng.gen::<f32>() * 0.1,
                    y: rng.gen::<f32>() * 0.1,
                    z: 0.0,
                },
            },
        ));
    }
}

pub fn despawn_dead(mut commands: Commands, entity_query: Query<(Entity, &Health)>) {
    for (entity, health) in entity_query.iter() {
        // If an entity's health has dropped to or below 0, despawn it.
        if health.value <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}
