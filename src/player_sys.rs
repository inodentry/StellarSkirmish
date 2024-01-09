use crate::components::*;
use crate::ship_parts::*;
use crate::traits::*;
use bevy::prelude::*;
use libm::atan2f;
use std::f32::consts::PI;

/// Reads keyboard input and runs the code to turn or accelerate the player's ship accordingly.
pub fn update_player_velocity_system(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Ship, &mut Velocity, &mut Transform, &Mass, &Thruster), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((ship, mut velocity, mut transform, mass, thruster)) = player_query.get_single_mut() {
        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let acceleration = transform.up() * thruster.force / mass.value;
            velocity.velocity += acceleration * time.delta_seconds();
            // There should be a global max speed and an individual max speed.
            // For now instead of dealing with it, just put in a reasonable literal.
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

/// Reads player input and fires the player ship's weapon systems accordingly.
pub fn player_weapons_system(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    mouse_input: Res<Input<MouseButton>>,
    mut player_query: Query<(&mut Ship, &Transform, &Velocity), With<Player>>,
    asset_server: Res<AssetServer>,
    mouse_coords: Res<WorldCoords>,
) {
    if let Ok((mut ship, transform, vel)) = player_query.get_single_mut() {
        // Fire Primary Weapon
        if keyboard_input.pressed(KeyCode::Space) && ship.primary_weapon.cd_timer.finished() {
            // The projectile's transform should originate from the firing ship.
            let mut projectile_transform =
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0)
                    .with_scale(GLOBAL_RESCALE_V);
            // Modify it a little so that it originates from just in front of the firing ship.
            projectile_transform.translation += transform.up() * 75.0 * GLOBAL_RESCALE_V;
            // Ensure that it is rotated in a way that aligns with the firing ship.
            projectile_transform.rotation = transform.rotation.clone();
            // Spawn the initial graphic
            commands.spawn((
                (SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.primary_weapon.on_spawn_sprite_path),
                    ..default()
                }),
                SelfDestruct {
                    cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
                },
            ));
            commands.spawn((
                SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.primary_weapon.sprite_path),
                    ..default()
                },
                // The Projectile is granted value's from the ship's primary_weapon component.
                // This depends on the type of projectile the cannon fires.
                ship.primary_weapon
                    .fire(transform.up(), vel.velocity.length()),
            ));
            println!(
                "Projectile velocity: {:?}",
                transform.up() * (ship.primary_weapon.proj_speed + vel.velocity.length())
            );
            println!("Ship velocity: {:?}", vel.velocity);
        }
        // Fire Secondary Weapon
        if mouse_input.pressed(MouseButton::Left) && ship.secondary_weapon.cd_timer.finished() {
            let mut projectile_transform =
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0)
                    .with_scale(GLOBAL_RESCALE_V);
            projectile_transform.translation += transform.up() * 75.0 * GLOBAL_RESCALE_V;
            projectile_transform.rotation = transform.rotation.clone();
            commands.spawn(AudioBundle {
                source: asset_server.load(&ship.secondary_weapon.sound_path),
                ..default()
            });
            // Spawn the initial graphic
            commands.spawn((
                (SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.secondary_weapon.on_spawn_sprite_path),
                    ..default()
                }),
                SelfDestruct {
                    cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
                },
            ));
            commands.spawn((
                SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.secondary_weapon.sprite_path),
                    ..default()
                },
                // The Projectile is granted value's from the ship's secondary_weapon component.
                // This depends on the type of projectile the cannon fires.
                ship.secondary_weapon
                    .fire(transform.up(), vel.velocity.length()),
            ));
        }

        // Fire Tertiary Weapon
        if mouse_input.pressed(MouseButton::Right) && ship.tertiary_weapon.cd_timer.finished() {
            let position = mouse_coords.coords;
            let y = position.y - transform.translation.y;
            let x = position.x - transform.translation.x;
            let aim = atan2f(y, x);

            let mut projectile_transform =
                Transform::from_xyz(transform.translation.x, transform.translation.y, 0.0)
                    .with_scale(GLOBAL_RESCALE_V * 0.5)
                    .with_rotation(Quat::from_rotation_z(aim - PI / 2.0));
            projectile_transform.translation += projectile_transform.up() * 75.0 * GLOBAL_RESCALE_V;
            commands.spawn(AudioBundle {
                source: asset_server.load(&ship.tertiary_weapon.sound_path),
                ..default()
            });
            // Spawn the initial graphic
            commands.spawn((
                (SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.tertiary_weapon.on_spawn_sprite_path),
                    ..default()
                }),
                SelfDestruct {
                    cd_timer: Timer::from_seconds(0.05, TimerMode::Once),
                },
            ));
            commands.spawn((
                SpriteBundle {
                    transform: projectile_transform,
                    texture: asset_server.load(&ship.tertiary_weapon.sprite_path),
                    ..default()
                },
                // The Projectile is granted value's from the ship's tertiary_weapon component.
                // This depends on the type of projectile the cannon fires.
                ship.tertiary_weapon
                    .fire(projectile_transform.up(), vel.velocity.length()),
            ));
        }
    }
}

pub fn test_weapon_toggle(
    mut player_query: Query<&mut Ship, With<Player>>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::T) {
        if let Ok(mut player_ship) = player_query.get_single_mut() {
            player_ship.primary_weapon = load_test_torpedo();
        }
    }
}
