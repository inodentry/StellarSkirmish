mod camera_sys;
mod components;
mod player_sys;
mod projectile_sys;

use crate::camera_sys::spawn_camera;
use crate::player_sys::{move_player_system, player_weapons_system, spawn_player_system};
use crate::projectile_sys::*;
use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player_system))
        .add_systems(Update, (move_player_system, player_weapons_system))
        .add_systems(Update, move_projectiles)
        .run();
}
