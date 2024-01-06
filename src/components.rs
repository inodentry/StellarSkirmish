use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

// Global Constants
pub const DAMPENING_FACTOR: f32 = 0.995;
pub const RESTITUTION_COEF: f32 = 0.6;

pub const KINETIC_DAMAGE_COEF: f32 = 0.01;

// Bevy Components
#[derive(Component)]
pub struct Ship {
    pub thrust: f32,
    pub angle: f32,
    pub turn_speed: f32,
    pub primary_weapon: PrimaryWeaponSystem,
    pub secondary_weapon: SecondaryWeaponSystem,
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
pub struct Health {
    pub value: f32,
}

#[derive(Component)]
pub struct Mass {
    pub value: f32,
}

#[derive(Component)]
pub struct Projectile {
    pub speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub fuel: f32,
}

#[derive(Component)]
pub struct Phase {
    pub cd_timer: Timer,
}

/// Describes the shape (circle or square) of a collision box and the value (length or radius) of it.
#[derive(Component)]
pub struct CollisionBox {
    pub shape: Shape,
    pub width_radius: f32,
    pub height: f32,
}

// Structs and Enums that are NOT Components

pub enum Shape {
    Circle,
    Square,
}

pub enum DamageType {
    Kinetic,
    Radiant,
}

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
