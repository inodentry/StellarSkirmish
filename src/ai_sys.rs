use crate::components::*;
use crate::traits::*;
use bevy::prelude::*;
use libm::atan2f;
use std::f32::consts::PI;

/// Takes a transform A, a turn speed, and the angle between points A and B, and rotates the transform
/// A toward B at the turn speed.
fn turn_toward(transform: &mut Transform, turn_speed: f32, angle_between: f32) {
    if angle_between > 0.0 {
        transform.rotate_z(turn_speed);
    } else {
        transform.rotate_z(-turn_speed);
    }
}

pub fn turret_ai_system(
    mut commands: Commands,
    mut q_enemy: Query<
        (&mut Ship, &mut Transform, &Velocity),
        (With<Enemy>, With<TurretAI>, Without<Player>),
    >,
    q_player: Query<(&Transform), (With<Player>, Without<Enemy>)>,
    asset_server: Res<AssetServer>,
) {
    // Simple turret AI. Turn toward the player, and fire repeatedly.
    for (mut enemy_ship, mut enemy_transform, vel) in q_enemy.iter_mut() {
        if let Ok(player_transform) = q_player.get_single() {
            // Calculate the angle between the enemy and the player.
            let y = player_transform.translation.y - enemy_transform.translation.y;
            let x = player_transform.translation.x - enemy_transform.translation.x;
            let target_angle = atan2f(y, x);

            let angle_between = enemy_transform
                .rotation
                .angle_between(Quat::from_rotation_z(target_angle))
                - PI / 2.0;

            turn_toward(&mut enemy_transform, enemy_ship.turn_speed, angle_between);

            if (-0.15 < angle_between)
                && (angle_between < 0.15)
                && enemy_ship.primary_weapon.cd_timer.finished()
            {
                // ######### FIRE! ##########
                // The projectile's transform should originate from the firing ship.
                let mut projectile_transform = Transform::from_xyz(
                    enemy_transform.translation.x,
                    enemy_transform.translation.y,
                    0.0,
                )
                .with_scale(GLOBAL_RESCALE_V);
                // Modify it a little so that it originates from just in front of the firing ship.
                projectile_transform.translation += enemy_transform.up() * 75.0 * GLOBAL_RESCALE_V;
                // Ensure that it is rotated in a way that aligns with the firing ship.
                projectile_transform.rotation = enemy_transform.rotation.clone();
                commands.spawn((
                    SpriteBundle {
                        transform: projectile_transform,
                        texture: asset_server.load(enemy_ship.primary_weapon.sprite_path.clone()),
                        ..default()
                    },
                    // The Projectile is granted value's from the ship's primary_weapon component.
                    // This depends on the type of projectile the cannon fires.
                    enemy_ship
                        .primary_weapon
                        .fire(projectile_transform.up(), vel.velocity.length()),
                ));
            }
        }
    }
}

pub fn speedy_ai_system(
    mut commands: Commands,
    mut q_enemy: Query<
        (
            &mut Ship,
            &mut Transform,
            &mut Velocity,
            &mut AITimer,
            &mut AITimer2,
            &Mass,
            &Thruster,
        ),
        (With<Enemy>, With<SpeedyAI>, Without<Player>),
    >,
    q_player: Query<(&Transform), (With<Player>, Without<Enemy>)>,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    // The intended behavior of the "speedy" enemy is to fly into mid-range of the player.
    // Once mid-range, alternatively fire on the player and fly away to change position.
    // Can be viewed as a FSM.
    // State 1: Fly toward player.
    // State 2: Turn and Fire at Player.
    // State 3: Turn and briefly engage thrusters to reposition.
    // If far from player, enter state 1. Otherwiser alternate states 2 and 3 every few seconds.

    for (
        mut enemy_ship,
        mut enemy_transform,
        mut vel,
        mut ai_timer,
        mut ai_timer2,
        mass,
        thruster,
    ) in q_enemy.iter_mut()
    {
        if let Ok(player_transform) = q_player.get_single() {
            // Calculate the distance between the enemy and the player.
            let distance_between = enemy_transform
                .translation
                .distance(player_transform.translation);

            // Calculate the angle between the enemy and the player.
            let y = player_transform.translation.y - enemy_transform.translation.y;
            let x = player_transform.translation.x - enemy_transform.translation.x;
            let target_angle = atan2f(y, x);

            let angle_between = enemy_transform
                .rotation
                .angle_between(Quat::from_rotation_z(target_angle))
                - PI / 2.0;

            // If we are too far from player, move toward the player by turning and engaging thruster.
            if distance_between > 500.0 {
                turn_toward(&mut enemy_transform, enemy_ship.turn_speed, angle_between);
                let acceleration = enemy_transform.up() * thruster.force / mass.value;
                vel.velocity += acceleration * time.delta_seconds();
                // There should be a global max speed and an individual max speed.
                // For now instead of dealing with it, just put in a reasonable literal.
                if vel.velocity.length() > MAX_SPEED {
                    vel.velocity = vel.velocity.clamp_length_max(MAX_SPEED)
                }
            } else if !ai_timer.cd_timer.finished() {
                ai_timer.cd_timer.tick(time.delta());
                // Enemy is close enough to the player and is transitioning to state 2. Turn and fire on player.
                turn_toward(&mut enemy_transform, enemy_ship.turn_speed, angle_between);

                if enemy_ship.primary_weapon.cd_timer.finished() {
                    let mut projectile_transform = Transform::from_xyz(
                        enemy_transform.translation.x,
                        enemy_transform.translation.y,
                        0.0,
                    )
                    .with_scale(GLOBAL_RESCALE_V);
                    // Modify it a little so that it originates from just in front of the firing ship.
                    projectile_transform.translation +=
                        enemy_transform.up() * 75.0 * GLOBAL_RESCALE_V;
                    // Ensure that it is rotated in a way that aligns with the firing ship.
                    projectile_transform.rotation = enemy_transform.rotation.clone();
                    commands.spawn((
                        SpriteBundle {
                            transform: projectile_transform,
                            texture: asset_server.load(&enemy_ship.primary_weapon.sprite_path),
                            ..default()
                        },
                        enemy_ship
                            .primary_weapon
                            .fire(enemy_transform.up(), vel.velocity.length()),
                    ));
                }
            } else if !ai_timer2.cd_timer.finished() {
                // Enemy is transitioning to state 3. Activate thrusters and turn a little just to change position.
                ai_timer2.cd_timer.tick(time.delta());
                enemy_transform.rotate_z(enemy_ship.turn_speed * 0.5);
                let acceleration = enemy_transform.up() * thruster.force / mass.value;
                vel.velocity += acceleration * time.delta_seconds();
                if vel.velocity.length() > MAX_SPEED {
                    vel.velocity = vel.velocity.clamp_length_max(MAX_SPEED)
                }
            } else {
                ai_timer.cd_timer.reset();
                ai_timer2.cd_timer.reset();
            }
        }
    }
}
