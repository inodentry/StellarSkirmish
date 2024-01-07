use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

// Global Constants
pub const DAMPENING_FACTOR: f32 = 0.995;

pub const RESTITUTION_COEF: f32 = 0.35;

// During collisions, objects are repulsed by this factor.
pub const REPULSION_FORCE: f32 = 0.05;

// This is a global scaling factor used for all sprite textures in the game.
pub const GLOBAL_RESCALE_V: Vec3 = Vec3 {
    x: 0.5,
    y: 0.5,
    z: 0.5,
};

pub const GLOBAL_RESCALE_C: f32 = 0.5;

// Bevy Components

// Components relating to ships
#[derive(Component)]
pub struct Ship {
    pub angle: f32,
    pub turn_speed: f32,
    pub primary_weapon: WeaponSystem,
    pub secondary_weapon: WeaponSystem,
    pub tertiary_weapon: WeaponSystem,
}

/// This struct is the primary, heavy-hitting weapon system of a ship.
#[derive(Component)]
pub struct WeaponSystem {
    pub name: String,
    pub value: f32,
    pub proj_speed: f32,
    // "fuel" decrements each tick, and the projectile is destroyed when it hits 0
    pub proj_fuel: f32,
    pub cooldown: f32,
    pub sprite_path: String,
    pub proj_type: ProjectileType,
    pub proj_mass: f32,
    pub dmg_type: DamageType,
    pub cd_timer: Timer,
}

#[derive(Component)]
pub struct Thruster {
    pub name: String,
    pub value: f32,
    pub force: f32,
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
    pub mass: f32,
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

#[derive(Clone, PartialEq)]
pub enum ProjectileType {
    Laser,
    Missile,
    Torpedo,
    Shells,
}
