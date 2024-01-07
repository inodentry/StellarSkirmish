use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

// Global Constants
pub const DAMPENING_FACTOR: f32 = 0.995;

pub const RESTITUTION_COEF: f32 = 0.6;

pub const KINETIC_DAMAGE_COEF: f32 = 0.01;

// This is a global scaling factor used for all sprite textures in the game.
pub const GLOBAL_RESCALE_V: Vec3 = Vec3 {
    x: 0.5,
    y: 0.5,
    z: 0.5,
};

pub const GLOBAL_RESCALE_C: f32 = 0.5;

// Bevy Components
#[derive(Component)]
pub struct Ship {
    pub thrust: f32,
    pub angle: f32,
    pub turn_speed: f32,
    pub primary_weapon: PrimaryWeaponSystem,
    pub secondary_weapon: SecondaryWeaponSystem,
    pub tertiary_weapon: TertiaryWeaponSystem,
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
    pub projectile_type: ProjectileType,
    pub damage_type: DamageType,
    pub damage_value: f32,
}

/// Entities with the Clipping component are capable of colliding with both Clipping and Phase entities.
#[derive(Component)]
pub struct Clipping {
    pub cd_timer: Timer,
}

/// Entities with the Phase component can ONLY collide with NoPhase entities, NOT with other Phase entities.
#[derive(Component)]
pub struct Phase {}

/// Describes the shape (circle or square) of a collision box and the value (length or radius) of it.
#[derive(Component)]
pub struct CollisionBox {
    pub shape: Shape,
    pub width_radius: f32,
    pub height: f32,
}

/// Entities with the "drag" component have their velocities subjected to global dampening. They gradually slow down.
/// Entities without this component will slow to a point, then slowly drift indefinitely due to inertia.
#[derive(Component)]
pub struct Drag {}

#[derive(Component)]
pub struct MainCamera {}

// Resources
#[derive(Resource, Default)]
pub struct WorldCoords {
    pub coords: Vec2,
}
// Structs and Enums that are NOT Components

pub enum Shape {
    Circle,
    Square,
}

#[derive(Clone, PartialEq)]
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
    pub proj_mass: f32,
    pub dmg_type: DamageType,
    pub cd_timer: Timer,
}

/// This struct is the second, situational weapon system of a ship.
pub struct SecondaryWeaponSystem {
    pub speed: f32,
    pub fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub proj_mass: f32,
    pub dmg_type: DamageType,
    pub cd_timer: Timer,
}

/// This struct is the tertiary weapon system / utility system of a ship.
pub struct TertiaryWeaponSystem {
    pub speed: f32,
    pub fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub proj_mass: f32,
    pub dmg_type: DamageType,
    pub cd_timer: Timer,
}

#[derive(Clone)]
pub enum ProjectileType {
    Laser,
    Missile,
    Torpedo,
    Shells,
}
