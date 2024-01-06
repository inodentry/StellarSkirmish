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
            speed: 400.0,
            angle: f32::to_radians(90.0),
            turn_speed: f32::to_radians(1.25),
            cannon: WeaponSystem {
                speed: 400.0,
                fuel: 100.0,
                proj_type: ProjectileType::Missile,
                sprite_path: "sprites/projectiles/spaceMissiles_001.png".to_string(),
                cooldown: 0.5,
                cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
            },
        },
    ));
}

pub fn move_player_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Ship, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((ship, mut transform)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let move_dir = transform.up() * ship.speed * time.delta_seconds();
            transform.translation += move_dir;
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
    mut player_query: Query<(&mut Ship, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (mut ship, transform) = player_query.get_single_mut().unwrap();

    // Fire Primary Cannon
    if keyboard_input.pressed(KeyCode::Space) && ship.cannon.cd_timer.finished() {
        let mut projectile_transform =
            Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0);
        projectile_transform.rotation = transform.rotation.clone();
        commands.spawn((
            SpriteBundle {
                transform: projectile_transform,
                texture: asset_server.load(ship.cannon.sprite_path.clone()),
                ..default()
            },
            // The Projectile is granted value's from the ship's Cannon component.
            // This depends on the type of projectile the cannon fires.
            Projectile {
                speed: ship.cannon.speed,
                fuel: ship.cannon.fuel,
            },
        ));
        ship.cannon.cd_timer.reset()
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
