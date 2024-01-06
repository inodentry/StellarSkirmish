use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

pub const DAMPENING_FACTOR: f32 = 0.995;

#[derive(Component)]
pub struct Ship {
    pub thrust: f32,
    pub angle: f32,
    pub turn_speed: f32,
    pub cannon: WeaponSystem,
    pub mass: f32,
}

#[derive(Component)]
pub struct Velocity {
    pub velocity: Vec3,
}

#[derive(Component)]
pub struct Player {}

#[derive(Component)]
pub struct Enemy {}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub fuel: f32,
}

pub struct WeaponSystem {
    pub speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub cd_timer: Timer,
}

#[derive(Clone)]
pub enum ProjectileType {
    Laser,
    Missile,
}
