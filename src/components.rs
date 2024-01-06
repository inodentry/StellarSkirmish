use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

// Global Constants
pub const DAMPENING_FACTOR: f32 = 0.995;

// Bevy Components
#[derive(Component)]
pub struct Ship {
    pub thrust: f32,
    pub angle: f32,
    pub turn_speed: f32,
    pub primary_weapon: PrimaryWeaponSystem,
    pub secondary_weapon: SecondaryWeaponSystem,
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
pub struct Asteroid {}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub fuel: f32,
}

// Structs and Enums that are NOT Components

/// This struct is the primary, heavy-hitting weapon system of a ship.
pub struct PrimaryWeaponSystem {
    pub speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub cd_timer: Timer,
}

/// This struct is the second, situational weapon system of a ship.
pub struct SecondaryWeaponSystem {
    pub speed: f32,
    pub fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub cd_timer: Timer,
}

/// This struct is the tertiary weapon system / utility system of a ship.
pub struct TertiaryWeaponSystem {
    pub speed: f32,
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
