use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub fn update_velocities_system(
    mut velocity_query: Query<
        (&mut Velocity, &mut Transform),
        (With<Velocity>, Without<Projectile>),
    >,
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
    mut projectile_query: Query<
        (Entity, &mut Projectile, &mut Transform, &Velocity),
        With<Projectile>,
    >,
    time: Res<Time>,
) {
    for (entity, mut projectile, mut transform, vel) in projectile_query.iter_mut() {
        let move_dir = vel.velocity * time.delta_seconds();
        transform.translation += move_dir;
        projectile.fuel -= 1.0;
        if projectile.fuel <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// This system is responsible for checking for collisions between entity that have the NoPhase
/// component, indicating that they cannot phase through each other. This includes things like
/// ships and asteroids, as opposed to Phase entities. NoPhase objects can collide with anything.
/// Phase entities can only collide with NoPhase entities. This prevents the 10,000 laser beams
/// and missiles from needing to check for collisions with each other.
/// The physics results of the collision are included in this system.
pub fn check_nophase_collisions(
    mut ship_query: Query<
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &NoPhase,
        ),
        (With<NoPhase>, With<Ship>, Without<Asteroid>),
    >,
    mut asteroid_query: Query<
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &mut NoPhase,
        ),
        (With<NoPhase>, With<Asteroid>, Without<Ship>),
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

pub fn collision_calculation_system(
    mut q_thing: Query<
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &mut NoPhase,
        ),
        (With<NoPhase>),
    >,
    mut damage_writer: EventWriter<DamageEvent>,
    mut collision_writer: EventWriter<CollisionEvent>,
) {
    for (thing1_e, mut thing1_t, thing1_b, mut thing1_v, thing1_m, thing1_p) in q_thing.iter() {
        for (thing2_e, mut thing2_t, thing2_b, mut thing2_v, thing2_m, mut thing2_p) in
            q_thing.iter()
        {
            {
                if !(thing1_p.cd_timer.finished() && thing2_p.cd_timer.finished())
                    || (thing1_e == thing2_e)
                {
                    continue;
                }
                let distance = thing1_t.translation.distance(thing2_t.translation);
                let ship_radius = thing1_b.width_radius;
                let asteroid_radius = thing2_b.width_radius;
                if distance < ship_radius + asteroid_radius {
                    println!("Collision Detected!");
                    // Get the pre-collision speed sum, which will be the max post-collision speed.
                    let max_speed = thing1_v.velocity.length() + thing2_v.velocity.length();

                    // Calculate the total mass to use in the physics calculations.
                    let total_mass = thing1_m.value + thing2_m.value;

                    // Get the kinetic energy of the impact (for use in damage event, not in physics).
                    let ke = 0.5
                        * total_mass
                        * (thing1_v.velocity - thing2_v.velocity).length().powf(2.0);

                    // If the kinetic energy is non-trivial, send kinetic damage events.
                    if ke > 50.0 {
                        damage_writer.send(DamageEvent {
                            target: thing1_e,
                            damage_type: DamageType::Kinetic,
                            damage_value: ke,
                        });
                        damage_writer.send(DamageEvent {
                            target: thing2_e,
                            damage_type: DamageType::Kinetic,
                            damage_value: ke,
                        });
                    }

                    // Get unit vectors indicating the directionality of the collision.
                    let thing1_line_of_impact =
                        (thing2_t.translation - thing1_t.translation).normalize();
                    let thing2_line_of_impact = -thing1_line_of_impact;

                    // Project the velocity of each object onto the line of impact.
                    let thing1_v_proj =
                        thing1_v.velocity.dot(thing1_line_of_impact) * thing1_line_of_impact;
                    let thing2_v_proj =
                        thing2_v.velocity.dot(thing2_line_of_impact) * thing2_line_of_impact;

                    // Get the perpendicular velocities.
                    let thing1_perp_v = thing1_v.velocity - thing1_v_proj;
                    let thing2_perp_v = thing2_v.velocity - thing2_v_proj;

                    // Calculate the updated projections onto the lines of impact.
                    // This formula calculates the updated velocities along the lines of collision
                    // according to conservation of momentum.
                    let final_thing1_v_proj =
                        ((thing1_m.value - RESTITUTION_COEF * thing2_m.value) * thing1_v_proj
                            + 2.0 * RESTITUTION_COEF * thing2_m.value * thing2_v_proj)
                            / total_mass;
                    let final_thing2_v_proj =
                        ((thing2_m.value - RESTITUTION_COEF * thing1_m.value) * thing2_v_proj
                            + 2.0 * RESTITUTION_COEF * thing1_m.value * thing1_v_proj)
                            / total_mass;

                    // Add the velocities.
                    // We subtract the line of impact vector to include a very slight repulsive force to the collision.
                    // This helps prevent objects from getting tangled.
                    let mut updated_thing1_v =
                        final_thing1_v_proj + thing1_perp_v - (thing1_line_of_impact * 0.1);
                    let mut updated_thing2_v =
                        final_thing2_v_proj + thing2_perp_v - (thing2_line_of_impact * 0.1);

                    // To avoid unrealistic behavior, like a light object glancing off an object of
                    // high mass and somehow getting a speed boost, we will cap the post-collision
                    // speed at the combined speed of both objects pre-collision. This cap makes
                    // the physics behavior seem a bit more realistic.
                    if updated_thing1_v.length() > max_speed {
                        println!("{}", "Fixed max speed".to_string());
                        updated_thing1_v = updated_thing1_v.normalize() * max_speed * 1.001;
                    }
                    if updated_thing2_v.length() > max_speed {
                        println!("{}", "Fixed max speed".to_string());
                        updated_thing2_v = updated_thing2_v.normalize() * max_speed * 1.001;
                    }

                    collision_writer.send(CollisionEvent {
                        entity: thing1_e,
                        new_velocity: updated_thing1_v,
                    });
                    collision_writer.send(CollisionEvent {
                        entity: thing2_e,
                        new_velocity: updated_thing2_v,
                    });
                }
            }
        }
    }
}

pub fn collision_resolution_system(
    mut object_query: Query<(&mut Velocity, &mut NoPhase), With<NoPhase>>,
    mut collision_reader: EventReader<CollisionEvent>,
) {
    for ev in collision_reader.read() {
        if let Ok((mut vel, mut nophase)) = object_query.get_mut(ev.entity) {
            vel.velocity = ev.new_velocity;
            nophase.cd_timer.reset();
        }
    }
}

pub fn check_projectile_collisions(
    mut commands: Commands,
    mut nophase_query: Query<
        (Entity, &mut Transform, &CollisionBox),
        (With<NoPhase>, Without<Phase>),
    >,
    mut phase_query: Query<
        (
            Entity,
            &mut Transform,
            //&CollisionBox,
            //&mut Velocity,
            //&Mass,
            &Projectile,
        ),
        (With<Phase>, Without<NoPhase>),
    >,
    mut damage_writer: EventWriter<DamageEvent>,
) {
    for (n_e, mut n_t, n_c) in nophase_query.iter_mut() {
        for (p_e, mut p_t, p_p) in phase_query.iter_mut() {
            let distance = n_t.translation.distance(p_t.translation);
            let n_radius = n_c.width_radius;
            // Replace this with an actual collision box later!
            let p_radius = 20.0 * GLOBAL_RESCALE_C;
            if distance < n_radius + p_radius {
                println!("Hit Detected!");
                commands.entity(p_e).despawn();
                damage_writer.send(DamageEvent {
                    target: n_e,
                    damage_type: p_p.damage_type.clone(),
                    damage_value: p_p.damage_value,
                });
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
            let mut total_damage = ev.damage_value;
            if ev.damage_type == DamageType::Kinetic {
                total_damage *= KINETIC_DAMAGE_COEF;
            }
            target_health.value -= total_damage;
            println!(
                "Entity {:?} now has {:?} Health!",
                ev.target, target_health.value
            )
        }
    }
}
