use crate::components::*;
use bevy::prelude::*;

pub fn movement_system(mut velocity_query: Query<(&mut Velocity, &mut Transform), With<Velocity>>) {
    for (mut velocity, mut transform) in velocity_query.iter_mut() {
        transform.translation += velocity.velocity;
        if velocity.velocity.length() > 0.0 {
            velocity.velocity *= DAMPENING_FACTOR;
        }
    }
}

pub fn move_projectiles(
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
