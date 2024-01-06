use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_player_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ships/playerShip1_blue.png"),
            ..default()
        },
        Player {},
        Ship {
            thrust: 10.0,
            mass: 1.0,
            angle: f32::to_radians(90.0),
            turn_speed: f32::to_radians(1.25),
            primary_weapon: PrimaryWeaponSystem {
                speed: 600.0,
                fuel: 300.0,
                proj_type: ProjectileType::Missile,
                sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
                cooldown: 0.5,
                cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
            secondary_weapon: SecondaryWeaponSystem {
                speed: 800.0,
                fuel: 400.0,
                proj_type: ProjectileType::Laser,
                sprite_path: "sprites/projectiles/laserBlue04.png".to_string(),
                cooldown: 0.2,
                cd_timer: Timer::from_seconds(0.2, TimerMode::Once),
            },
        },
        Velocity {
            velocity: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            },
        },
    ));
}

pub fn update_velocity_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Ship, &mut Velocity, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((ship, mut velocity, mut transform)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let acceleration = transform.up() * ship.thrust / ship.mass;
            velocity.velocity += acceleration * time.delta_seconds();
            if velocity.velocity.length() > 300.0 {
                velocity.velocity = velocity.velocity.clamp_length_max(300.0)
            }
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            // Using angles, so if turning left hits 360.0 degrees, it wraps around to 0.0.
            transform.rotate_z(ship.turn_speed);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            // Using angles, so if turning right hits 0.0 degrees, it wraps around to 360.0.
            transform.rotate_z(-ship.turn_speed);
        }
    }
}

pub fn player_weapons_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_query: Query<(&mut Ship, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (mut ship, transform) = player_query.get_single_mut().unwrap();

    // Fire Primary Weapon
    if keyboard_input.pressed(KeyCode::Space) && ship.primary_weapon.cd_timer.finished() {
        let mut projectile_transform =
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0);
        projectile_transform.rotation = transform.rotation.clone();
        commands.spawn((
            SpriteBundle {
                transform: projectile_transform,
                texture: asset_server.load(ship.primary_weapon.sprite_path.clone()),
                ..default()
            },
            // The Projectile is granted value's from the ship's Cannon component.
            // This depends on the type of projectile the cannon fires.
            Projectile {
                speed: ship.primary_weapon.speed,
                fuel: ship.primary_weapon.fuel,
            },
        ));
        ship.primary_weapon.cd_timer.reset()
    }
    // Fire Secondary Weapon
    if mouse_input.pressed(MouseButton::Left) && ship.secondary_weapon.cd_timer.finished() {
        let mut projectile_transform =
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0);
        projectile_transform.rotation = transform.rotation.clone();
        commands.spawn((
            SpriteBundle {
                transform: projectile_transform,
                texture: asset_server.load(ship.secondary_weapon.sprite_path.clone()),
                ..default()
            },
            // The Projectile is granted value's from the ship's Cannon component.
            // This depends on the type of projectile the cannon fires.
            Projectile {
                speed: ship.secondary_weapon.speed,
                fuel: ship.secondary_weapon.fuel,
            },
        ));
        ship.secondary_weapon.cd_timer.reset()
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = 32.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}
