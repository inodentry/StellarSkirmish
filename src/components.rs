use crate::traits::*;
use bevy::prelude::*;

/// This file contains the ECS Components to make the game run in addition to containing
/// global constants and utility structs.

// Global Constants
pub const DAMPENING_FACTOR: f32 = 0.995;

// Global baseline restitution factor. The game's physics system begins by assuming perfect elasticity during
// a collision, resulting in a very bouncy experience. Setting a restitution factor below 1 reduces the amount of
// elasticity in the collision, so the colliding objects bounce less. However, less elasticity also means that
// more of the energy in the collision is absorbed, leading to more damage.
pub const RESTITUTION_COEF: f32 = 0.35;

// During collisions, objects are repulsed by this factor.
pub const REPULSION_FORCE: f32 = 0.05;

// This is a global scaling factor used for all sprite textures in the game.
pub const GLOBAL_RESCALE_V: Vec3 = Vec3 {
    x: 0.35,
    y: 0.35,
    z: 0.35,
};

pub const GLOBAL_RESCALE_C: f32 = 0.35;

// Based on ship speeds in pixels in the game, we are claiming that moving
// 1 pixel per second is equal to moving 0.33 meters per second.
// This may be useful to keep the numbers in various calculations somewhat believable
// and relatable to reality. It lets us give projectiles realistic speeds and calculate
// somewhat realistic kinetic energy absorbed in collisions.
// Speed and velocity literals should be in m/s, and converted to px/s just for movement.
pub const PS_TO_MS: f32 = 0.33;
pub const MS_TO_PS: f32 = 3.0;

// This is a global baseline for how much damage absorbed kinetic energy does. For example, a ship
// absorbing 1000 joules of kinetic energy sustains 1000 * KE_TO_DMG damage. This ensures that we can use
// physically accurate values and calculations for energy, but not need to compensate with millions of hit points.
pub const KE_TO_DMG: f32 = 0.001;

pub const MAX_SPEED: f32 = 300.0;

// Bevy Components

// Components relating to ships
#[derive(Component)]
pub struct Ship {
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
    pub on_spawn_sprite_path: String,
    pub sound_path: String,
    pub proj_type: ProjectileType,
    pub proj_mass: f32,
    pub dmg_type: DamageType,
    pub cd_timer: Timer,
    pub dmg: f32,
}
impl Weapon<(Projectile, Phase, Velocity)> for WeaponSystem {
    fn fire(&mut self, vel_dir: Vec3, origin_speed: f32) -> (Projectile, Phase, Velocity) {
        println!("vel_dir: {:?} | origin speed: {}", vel_dir, origin_speed);
        self.cd_timer.reset();
        (
            Projectile {
                speed: self.proj_speed,
                fuel: self.proj_fuel,
                projectile_type: self.proj_type.clone(),
                damage_type: self.dmg_type.clone(),
                mass: self.proj_mass,
                damage_value: self.dmg,
            },
            Phase {},
            Velocity {
                velocity: vel_dir * (self.proj_speed + origin_speed),
            },
        )
    }
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
pub struct Missile {
    pub turn_speed: f32,
    pub fuel: f32,
}

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

#[derive(Component)]
pub struct SelfDestruct {
    pub cd_timer: Timer,
}

#[derive(Component)]
pub struct AITimer {
    pub cd_timer: Timer,
}

#[derive(Component)]
pub struct AITimer2 {
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
pub struct Drag {
    pub dampening_factor: f32,
}

#[derive(Component)]
pub struct MainCamera {}

#[derive(Component, PartialEq)]
pub enum EntityType {
    Player,
    Ship,
    Asteroid,
}

// ----------------
// -- AI Markers --
// ----------------
#[derive(Component)]
pub struct DefenderAI {}

#[derive(Component)]
pub struct DroneAI {}

#[derive(Component)]
pub struct LunkerAI {}

#[derive(Component)]
pub struct MineLayerAI {}

#[derive(Component)]
pub struct PicketAI {}

#[derive(Component)]
pub struct RammerAI {}

#[derive(Component)]
pub struct SpeedyAI {}

#[derive(Component)]
pub struct TurretAI {}

// ---------------
// -- Resources --
// ---------------
#[derive(Resource, Default)]
pub struct WorldCoords {
    pub coords: Vec2,
}

#[derive(Resource)]
pub struct ExplosionSound {
    sound: Handle<AudioSource>,
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
