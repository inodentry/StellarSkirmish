use crate::components::*;
use crate::traits::*;
use bevy::prelude::*;
use libm::atan2f;
use std::f32::consts::PI;

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

            if angle_between > 0.0 {
                enemy_transform.rotate_z(enemy_ship.turn_speed);
            } else {
                enemy_transform.rotate_z(-enemy_ship.turn_speed);
            }
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
