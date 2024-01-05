mod camera_sys;
mod player_sys;
mod components;

use bevy::prelude::*;
use crate::camera_sys::spawn_camera;
use crate::player_sys::{player_movement, spawn_player};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, (spawn_camera, spawn_player))
        .add_systems(Update, player_movement)
        .run();
}
