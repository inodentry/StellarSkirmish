use crate::components::*;
use crate::events::*;
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
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &Phase,
        ),
        (With<Phase>, With<Ship>, Without<Asteroid>),
    >,
    mut asteroid_query: Query<
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &mut Phase,
        ),
        (With<Phase>, With<Asteroid>, Without<Ship>),
    >,
    mut damage_writer: EventWriter<DamageEvent>,
) {
    for (ship_e, mut ship_t, ship_box, mut ship_vel, ship_m, _) in ship_query.iter_mut() {
        for (roid_e, mut roid_t, roid_box, mut roid_vel, roid_m, mut roid_phase) in
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
                // Get the pre-collision speed sum, which will be the max post-collision speed.
                let max_speed = ship_vel.velocity.length() + roid_vel.velocity.length();

                // Reset the phase timer. This timer prevents repeated collisions every tick.
                roid_phase.cd_timer.reset();
                let total_mass = ship_m.value + roid_m.value;

                // Get the kinetic energy of the impact (for use in other systems, not here).
                let ke =
                    0.5 * total_mass * (ship_vel.velocity - roid_vel.velocity).length().powf(2.0);

                // If the kinetic energy is non-trivial, send kinetic damage events.

                /*
                I wanted to make this into events. If kinetic damage occurs, send a damage event
                that contains data on whom to target, the damage type, and the damage value.
                However, I couldn't find any way to send a reference to the target, such as Health.
                This would be something useful to solve later.
                 */
                if ke > 50.0 {
                    damage_writer.send(DamageEvent {
                        target: ship_e,
                        damage_type: DamageType::Kinetic,
                        damage_value: ke,
                    });
                    damage_writer.send(DamageEvent {
                        target: roid_e,
                        damage_type: DamageType::Kinetic,
                        damage_value: ke,
                    });
                }

                // Get unit vectors indicating the directionality of the collision.
                let ship_line_of_impact = (roid_t.translation - ship_t.translation).normalize();
                let roid_line_of_impact = -ship_line_of_impact;

                // Project the velocity of each object onto the line of impact.
                let ship_v_proj = ship_vel.velocity.dot(ship_line_of_impact) * ship_line_of_impact;
                let roid_v_proj = roid_vel.velocity.dot(roid_line_of_impact) * roid_line_of_impact;

                // Get the perpendicular velocities.
                let ship_perp_vel = ship_vel.velocity - ship_v_proj;
                let roid_perp_vel = roid_vel.velocity - roid_v_proj;

                // Calculate the updated projections onto the lines of impact.
                // This formula calculates the updated velocities along the lines of collision
                // according to conservation of momentum.
                let final_ship_v_proj = ((ship_m.value - RESTITUTION_COEF * roid_m.value)
                    * ship_v_proj
                    + 2.0 * RESTITUTION_COEF * roid_m.value * roid_v_proj)
                    / total_mass;
                let final_roid_v_proj = ((roid_m.value - RESTITUTION_COEF * ship_m.value)
                    * roid_v_proj
                    + 2.0 * RESTITUTION_COEF * ship_m.value * ship_v_proj)
                    / total_mass;

                // Add the velocities.
                let mut updated_ship_vel = final_ship_v_proj + ship_perp_vel;
                let mut updated_roid_vel = final_roid_v_proj + roid_perp_vel;

                // To avoid unrealistic behavior, like a light object glancing off an object of
                // high mass and somehow getting a speed boost, we will cap the post-collision
                // speed at the combined speed of both objects pre-collision. This cap makes
                // the physics behavior seem a bit more realistic.
                if updated_ship_vel.length() > max_speed {
                    println!("{}", "Fixed max speed".to_string());
                    updated_ship_vel = updated_ship_vel.normalize() * max_speed;
                }
                if updated_roid_vel.length() > max_speed {
                    println!("{}", "Fixed max speed".to_string());
                    updated_roid_vel = updated_roid_vel.normalize() * max_speed;
                }
                ship_vel.velocity = updated_ship_vel;
                roid_vel.velocity = updated_roid_vel;
            }
        }
    }
}

pub fn inflict_damage_system(
    mut damage_reader: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for ev in damage_reader.read() {
        println!(
            "Entity {:?} incurred {} damage!",
            ev.target,
            ev.damage_value * KINETIC_DAMAGE_COEF
        );
        if let Ok(mut target_health) = health_query.get_mut(ev.target) {
            target_health.value -= ev.damage_value * KINETIC_DAMAGE_COEF;
            println!(
                "Entity {:?} now has {:?} Health!",
                ev.target, target_health.value
            )
        }
    }
}
