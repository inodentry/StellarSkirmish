use bevy::prelude::*;

#[derive(Component)]
pub struct Ship {
    pub speed: f32,
    pub angle: f32,
    pub turn_speed: f32,
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
