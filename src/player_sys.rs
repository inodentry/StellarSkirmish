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
                speed: 800.0,
                fuel: 100.0,
                proj_type: ProjectileType::Laser,
                sprite_path: "sprites/projectiles/laserBlue01.png".to_string(),
                cooldown: 0.5,
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
    player_query: Query<(&Ship, &Transform), With<Player>>,
    asset_server: Res<AssetServer>,
) {
    let (ship, transform) = player_query.get_single().unwrap();

    // Fire Primary Cannon
    if keyboard_input.pressed(KeyCode::Space) {
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
    }
}
