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
pub struct Enemy{}
