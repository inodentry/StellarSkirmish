use crate::components::*;
use bevy::prelude::*;

pub fn update_velocities_system(
    mut velocity_query: Query<(&mut Velocity, &mut Transform), With<Velocity>>,
) {
    for (mut velocity, mut transform) in velocity_query.iter_mut() {
        transform.translation += velocity.velocity;
        if velocity.velocity.length() > 500.0 {
            velocity.velocity = velocity.velocity.normalize() * 500.0
        }
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
        (&mut Transform, &CollisionBox, &mut Velocity, &Mass, &Phase),
        (With<Phase>, With<Ship>, Without<Asteroid>),
    >,
    mut asteroid_query: Query<
        (
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &mut Phase,
        ),
        (With<Phase>, With<Asteroid>, Without<Ship>),
    >,
) {
    for (mut ship_t, ship_box, mut ship_vel, ship_m, _) in ship_query.iter_mut() {
        for (mut roid_t, roid_box, mut roid_vel, roid_m, mut roid_phase) in
            asteroid_query.iter_mut()
        {
            if !roid_phase.cd_timer.finished() {
                continue;
            }
            let distance = ship_t.translation.distance(roid_t.translation);
            let ship_radius = ship_box.width_radius;
            let asteroid_radius = roid_box.width_radius;
            if distance < ship_radius + asteroid_radius {
                println!("Collision Detected!");
                roid_phase.cd_timer.reset();
                let total_mass = ship_m.value + roid_m.value;

                let ship_line_of_impact = (roid_t.translation - ship_t.translation).normalize();
                let roid_line_of_impact = (ship_t.translation - roid_t.translation).normalize();

                let ship_v_proj = ship_vel.velocity.dot(ship_line_of_impact) * ship_line_of_impact;
                let roid_v_proj = roid_vel.velocity.dot(roid_line_of_impact) * roid_line_of_impact;

                let ship_perp_vel = ship_vel.velocity - ship_v_proj;
                let roid_perp_vel = roid_vel.velocity - roid_v_proj;

                /*
                The - sign is unconventional. The correct formula for updating the velocity along
                the line of impact according to the conservation of momentum should have a + sign.
                However, this resulted in the object of greater mass moving in the direction
                opposite to what we would expect. I was unable to find the error after 2 hours and
                getting this to be textbook accurate is not a priority, so I just swapped the sign
                and decided to move on.
                */

                let final_ship_v_proj = ((ship_m.value - RESTITUTION_COEF * roid_m.value)
                    * ship_v_proj
                    + 2.0 * RESTITUTION_COEF * roid_m.value * roid_v_proj)
                    / total_mass;
                let final_roid_v_proj = ((roid_m.value - RESTITUTION_COEF * ship_m.value)
                    * roid_v_proj
                    + 2.0 * RESTITUTION_COEF * ship_m.value * ship_v_proj)
                    / total_mass;
                println!("{:?}", final_ship_v_proj);
                println!("{:?}", final_roid_v_proj);

                ship_vel.velocity += final_ship_v_proj + ship_perp_vel;
                roid_vel.velocity += final_roid_v_proj + roid_perp_vel;
            }
        }
    }
}
