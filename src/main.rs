mod camera_sys;
mod components;
mod player_sys;
mod projectile_sys;

use crate::camera_sys::spawn_camera;
use crate::components::*;
use crate::player_sys::*;
use crate::projectile_sys::*;
use bevy::prelude::*;

fn main() {
    App::new()
        // Bevy Plugins and Basic Startup
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(Startup, (spawn_camera, spawn_player_system))
        .add_systems(
            Update,
            (
                move_player_system,
                player_weapons_system,
                confine_player_movement,
            ),
        )
        .add_systems(Update, move_projectiles)
        .add_systems(Update, tick_timers)
        .run();
}

fn tick_timers(mut timer_query: Query<&mut Ship>, time: Res<Time>) {
    for mut ship in timer_query.iter_mut() {
        ship.cannon.cd_timer.tick(time.delta());
    }
}
