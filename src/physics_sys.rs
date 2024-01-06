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
    ship_query: Query<(&Transform, &CollisionBox), (With<NoPhase>, With<Ship>)>,
    asteroid_query: Query<(&Transform, &CollisionBox), (With<NoPhase>, With<Asteroid>)>,
) {
    for (ship_t, ship_box) in ship_query.iter() {
        for (asteroid_t, asteroid_box) in asteroid_query.iter() {
            let distance = ship_t.translation.distance(asteroid_t.translation);
            let ship_radius = ship_box.width_radius;
            let asteroid_radius = asteroid_box.width_radius;
            if distance < ship_radius + asteroid_radius {
                println!("Collision Detected!");
            }
        }
    }
}
