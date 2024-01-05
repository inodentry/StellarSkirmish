use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub angle: f32,
    pub turn_speed: f32,
    pub cannon: WeaponSystem,
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
}

#[derive(Clone)]
pub enum ProjectileType {
    Laser,
    Missile,
}
