use crate::components::*;
use bevy::prelude::*;

pub fn update_velocities_system(
    mut velocity_query: Query<(&mut Velocity, &mut Transform), With<Velocity>>,
) {
    for (mut velocity, mut transform) in velocity_query.iter_mut() {
        transform.translation += velocity.velocity;
        if velocity.velocity.length() > 0.0 {
            velocity.velocity *= DAMPENING_FACTOR;
        }
    }
}

pub fn move_projectiles_system(
    mut commands: Commands,
    mut proj_query: Query<(Entity, &mut Projectile, &mut Transform)>,
    time: Res<Time>,
) {
    for (entity, mut projectile, mut transform) in proj_query.iter_mut() {
        let move_dir = transform.up() * projectile.speed * time.delta_seconds();
        transform.translation += move_dir;
        projectile.fuel -= 1.0;
        if projectile.fuel <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

pub fn check_collisions_system(
    mut ship_query: Query<
        (&Transform, &CollisionBox, &mut Velocity, &Mass),
        (With<NoPhase>, With<Ship>, Without<Asteroid>),
    >,
    mut asteroid_query: Query<
        (&Transform, &CollisionBox, &mut Velocity, &Mass),
        (With<NoPhase>, With<Asteroid>, Without<Ship>),
    >,
) {
    for (ship_t, ship_box, mut ship_vel, ship_m) in ship_query.iter_mut() {
        for (roid_t, roid_box, mut roid_vel, roid_m) in asteroid_query.iter_mut() {
            let distance = ship_t.translation.distance(roid_t.translation);
            let ship_radius = ship_box.width_radius;
            let asteroid_radius = roid_box.width_radius;
            if distance < ship_radius + asteroid_radius {
                println!("Collision Detected!");
                let rel_vel = ship_vel.velocity - roid_vel.velocity;
                let old_ship_vel = ship_vel.velocity;
                let old_roid_vel = roid_vel.velocity;
                ship_vel.velocity =
                    old_ship_vel - (2.0 * roid_m.value / (roid_m.value + ship_m.value)) * rel_vel;
                roid_vel.velocity =
                    old_roid_vel - (2.0 * ship_m.value / (roid_m.value + ship_m.value)) * rel_vel;
            }
        }
    }
}
