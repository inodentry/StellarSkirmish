use crate::components::*;
use crate::events::*;
use bevy::prelude::*;

pub fn movement_system(
    mut velocity_query: Query<
        (Entity, &mut Velocity, &mut Transform),
        (With<Velocity>, Without<Projectile>),
    >,
    drag_query: Query<&Drag>,
) {
    // Update velocities. Cap max speed, if needed.
    for (entity, mut velocity, mut transform) in velocity_query.iter_mut() {
        // Velocities are in meters per second, and need to be converted to pixels per second.
        transform.translation += velocity.velocity * MS_TO_PS;
        if velocity.velocity.length() > 200.0 {
            velocity.velocity = velocity.velocity.normalize() * 200.0
        }

        // If this entity has the Drag component, apply dampening to its velocity.
        // Otherwise, apply global dampening, but don't slow to 0.0. The object can drift slowly indefinitely.
        // The component isn't actually used, we're just checking whether the entity has it.
        if let Ok(drag) = drag_query.get(entity) {
            if velocity.velocity.length() > 0.0 {
                velocity.velocity *= drag.dampening_factor;
            }
        } else if velocity.velocity.length() > 0.25 {
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
        let move_dir = vel.velocity * MS_TO_PS * time.delta_seconds();
        transform.translation += move_dir;
        projectile.fuel -= 1.0;
        if projectile.fuel <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

/// This system is checks for collisions between entities with the Clipping component and calculates the
/// physics result of the collision to be sent as Events.
pub fn collision_calculation_system(
    mut q_thing: Query<
        (
            Entity,
            &mut Transform,
            &CollisionBox,
            &mut Velocity,
            &Mass,
            &mut Clipping,
        ),
        (With<Clipping>),
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
                    // Thump! ( this is disabled now, it's annoying and lags the audio since it happens so often.
                    // commands.spawn(AudioBundle {
                    //     source: asset_server.load("sounds/impactSoft_medium_001.ogg"),
                    //     ..default()
                    // });

                    // Get the pre-collision speed sum, which will be the max post-collision speed.
                    let max_speed = thing1_v.velocity.length() + thing2_v.velocity.length();

                    // Calculate the total mass to use in the physics calculations.
                    let total_mass = thing1_m.value + thing2_m.value;

                    // Get the initial kinetic energy of each object in the collision.
                    let initial_ke = 0.5
                        * (thing1_m.value * thing1_v.velocity.dot(thing1_v.velocity)
                            + thing2_m.value * thing2_v.velocity.dot(thing2_v.velocity));

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
                    let mut updated_thing1_v = final_thing1_v_proj + thing1_perp_v;
                    let mut updated_thing2_v = final_thing2_v_proj + thing2_perp_v;

                    // Get the final total kinetic energy of each object in the collision.
                    // We get this prior to our adjustments, since the adjustments are fudge factors which
                    // violate the conservation of energy for the sake of gameplay stabilization.
                    // We want to ignore those adjustments when we calculate energy transfer for other purposes.
                    let final_ke = 0.5
                        * (thing1_m.value * updated_thing1_v.length().powf(2.0)
                            + thing2_m.value * updated_thing2_v.length().powf(2.0));

                    // We add a very slight repulsive force to each object in the direction opposite to the impact.
                    // This helps prevent objects from getting tangled.
                    updated_thing1_v += -thing1_line_of_impact * REPULSION_FORCE;
                    updated_thing2_v += -thing2_line_of_impact * REPULSION_FORCE;

                    // To avoid unrealistic behavior, like a light object glancing off an object of
                    // high mass and somehow getting a speed boost, we will cap the post-collision
                    // speed at the combined speed of both objects pre-collision. This cap makes
                    // the physics behavior seem a bit more realistic.
                    // We add a tiny value to max_speed so that stationary overlapping objects can
                    // still be gently "de-tangled."
                    if updated_thing1_v.length() > max_speed {
                        updated_thing1_v = updated_thing1_v.normalize() * (max_speed + 0.001);
                    }
                    if updated_thing2_v.length() > max_speed {
                        updated_thing2_v = updated_thing2_v.normalize() * (max_speed + 0.001);
                    }

                    // Write a CollisionEvent for each object in the collision. This system uses an immutable query
                    // to get around having potentially two references to the same underlying structures (since it
                    // needs to check every permutation of objects for collisions). These events are handled safely
                    // by another system that can mutate the underlying data.
                    collision_writer.send(CollisionEvent {
                        entity: thing1_e,
                        new_velocity: updated_thing1_v,
                    });
                    collision_writer.send(CollisionEvent {
                        entity: thing2_e,
                        new_velocity: updated_thing2_v,
                    });

                    // Calculate how much kinetic energy must have been absorbed in the collision.
                    let ke_absorbed = initial_ke - final_ke;

                    // We don't want to bother dinging objects will little damage for every trivial bump.
                    // However, if non-trivial kinetic energy is absorbed, this causes damage.
                    // "Non-trivial" is set at 2000 joules, because that would mean each object absorbed roughly
                    // 1000 joules, which would be converted into 1 damage based on the default global
                    // KE_TO_DMG constant.
                    // We write the kinetic energy absorbed by each object to a DamageEvent, allowing another system
                    // to read them and handle them, factoring in resistances etc. as needed.
                    if ke_absorbed > 2000.0 {
                        damage_writer.send(DamageEvent {
                            target: thing1_e,
                            damage_type: DamageType::Kinetic,
                            damage_value: KE_TO_DMG * ke_absorbed / 2.0,
                        });
                        damage_writer.send(DamageEvent {
                            target: thing2_e,
                            damage_type: DamageType::Kinetic,
                            damage_value: KE_TO_DMG * ke_absorbed / 2.0,
                        });
                    }
                }
            }
        }
    }
}

/// This system reads in CollisionEvent events and updates the velocities of the entity involved in the event.
pub fn collision_resolution_system(
    mut object_query: Query<(&mut Velocity, &mut Clipping), With<Clipping>>,
    mut collision_reader: EventReader<CollisionEvent>,
) {
    // CollisionEvent would be more flexible if it held change in velocity or force rather than new velocity.
    for ev in collision_reader.read() {
        if let Ok((mut vel, mut clipping)) = object_query.get_mut(ev.entity) {
            vel.velocity = ev.new_velocity;
            clipping.cd_timer.reset();
        }
    }
}

/// This system checks for collisions between entities that have the Clipping component and those that have the
/// Phase component, but it does not check for collisions between Clipping/Clipping or Phase/Phase. Sends
/// damage events if a projectile hits an object.
pub fn check_projectile_collisions(
    mut commands: Commands,
    mut clipping_query: Query<
        (Entity, &mut Transform, &CollisionBox),
        (With<Clipping>, Without<Phase>),
    >,
    mut phase_query: Query<(Entity, &mut Transform, &Projectile), (With<Phase>, Without<Clipping>)>,
    asset_server: Res<AssetServer>,
    mut damage_writer: EventWriter<DamageEvent>,
) {
    for (n_e, mut n_t, n_c) in clipping_query.iter_mut() {
        for (p_e, mut p_t, p_p) in phase_query.iter_mut() {
            let distance = n_t.translation.distance(p_t.translation);
            let n_radius = n_c.width_radius;
            // Replace this with an actual collision box later!
            let p_radius = 20.0 * GLOBAL_RESCALE_C;
            if distance < n_radius + p_radius {
                commands.entity(p_e).despawn();

                // Lasers have no mass, their damage is based on their base damage value
                // Shells do have mass, and their damage is their kinetic energy.
                if p_p.projectile_type == ProjectileType::Laser {
                    damage_writer.send(DamageEvent {
                        target: n_e,
                        damage_type: p_p.damage_type.clone(),
                        damage_value: p_p.damage_value,
                    });
                } else {
                    commands.spawn((
                        SpriteBundle {
                            transform: p_t.clone().with_scale(GLOBAL_RESCALE_V * 0.5),
                            texture: asset_server.load("sprites/effects/explosion_tmp.png"),
                            ..default()
                        },
                        SelfDestruct {
                            cd_timer: Timer::from_seconds(0.1, TimerMode::Once),
                        },
                    ));
                    damage_writer.send(DamageEvent {
                        target: n_e,
                        damage_type: p_p.damage_type.clone(),
                        damage_value: p_p.damage_value
                            + KE_TO_DMG * 0.5 * p_p.mass * p_p.speed.powf(2.0),
                    });
                }
            }
        }
    }
}

/// This system reads DamageEvent events and handles both the calculations and resolutions involved.
pub fn inflict_damage_system(
    mut damage_reader: EventReader<DamageEvent>,
    mut health_query: Query<&mut Health>,
) {
    for ev in damage_reader.read() {
        if let Ok(mut target_health) = health_query.get_mut(ev.target) {
            let total_damage = ev.damage_value;
            println! {"Handled {} dmg.", {total_damage}};
            target_health.value -= total_damage;
        }
    }
}

/// This system checks specifically for missile collisions with objects that have clipping.
/// When a missile hits any entity that has clipping, it detonates.
pub fn check_missile_collisions_system(
    q_missile: Query<(Entity, &CollisionBox, &Transform), With<Missile>>,
    q_clipping: Query<(&CollisionBox, &Transform), (Without<Missile>, Without<MineLayerAI>)>,
    mut detonation_event_writer: EventWriter<MissileDetonationEvent>,
) {
    for (missile_entity, missile_box, missile_transform) in q_missile.iter() {
        for (clip_box, clip_transform) in q_clipping.iter() {
            let distance = clip_transform
                .translation
                .distance(missile_transform.translation);
            let n_radius = clip_box.width_radius;
            // Replace this with an actual collision box later!
            let p_radius = missile_box.width_radius;
            if distance < n_radius + p_radius {
                // The missile collided with something, register the detonation event.
                detonation_event_writer.send(MissileDetonationEvent {
                    entity: missile_entity,
                });
            }
        }
    }
}

pub fn handle_denotation_event_system(
    mut commands: Commands,
    mut q_missile: Query<&Transform, With<Missile>>,
    q_clip: Query<(Entity, &Transform, &CollisionBox), (With<Clipping>, Without<Missile>)>,
    mut detonation_reader: EventReader<MissileDetonationEvent>,
    mut damage_writer: EventWriter<DamageEvent>,
    asset_server: Res<AssetServer>,
) {
    for ev in detonation_reader.read() {
        if let Ok((missile_transform)) = q_missile.get(ev.entity) {
            commands.spawn((
                AudioBundle {
                    source: asset_server.load("sounds/explosionCrunch_003.ogg"),
                    ..default()
                },
                SelfDestruct {
                    cd_timer: Timer::from_seconds(0.5, TimerMode::Once),
                },
            ));
            commands.spawn((
                SpriteBundle {
                    transform: missile_transform.clone(),
                    texture: asset_server.load("sprites/effects/explosion_tmp.png"),
                    ..default()
                },
                SelfDestruct {
                    cd_timer: Timer::from_seconds(0.25, TimerMode::Once),
                },
            ));
            for (entity, clip_transform, clip_box) in q_clip.iter() {
                let distance = clip_transform
                    .translation
                    .distance(missile_transform.translation);
                let n_radius = clip_box.width_radius;
                // Replace this with an actual collision box later!
                let explosion_radius = 200.0;
                if distance < n_radius + explosion_radius {
                    // The missile collided with something, register the detonation event.
                    damage_writer.send(DamageEvent {
                        target: entity,
                        damage_type: DamageType::Kinetic,
                        damage_value: 50.0,
                    });
                }
            }
            commands.entity(ev.entity).despawn();
        }
    }
}
