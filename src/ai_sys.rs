use crate::components::*;
use bevy::prelude::*;
use libm::atan2f;
use std::f32::consts::PI;

pub fn enemy_ai_sys(
    mut q_enemy: Query<(&Ship, &mut Transform), (With<Enemy>, Without<Player>)>,
    q_player: Query<(&Transform), (With<Player>, Without<Enemy>)>,
) {
    // Start out very basic AI to have the other ship look at the player.
    let (enemy_ship, mut enemy_transform) = q_enemy.get_single_mut().unwrap();
    let (player_transform) = q_player.get_single().unwrap();

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
}
