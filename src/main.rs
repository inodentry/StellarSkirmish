mod camera_sys;
mod components;
mod events;
mod physics_sys;
mod player_sys;
mod spawn_sys;

use bevy::prelude::*;
use camera_sys::*;
use components::*;
use events::*;
use physics_sys::*;
use player_sys::*;
use spawn_sys::*;

fn main() {
    App::new()
        // Bevy Plugins and Basic Startup
        .add_plugins(DefaultPlugins)
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_systems(
            Startup,
            (spawn_camera, spawn_player_system, spawn_asteroid_system),
        )
        // Register Events
        .add_event::<DamageEvent>()
        // Update Systems
        .add_systems(
            Update,
            (
                player_weapons_system,
                confine_player_movement,
                update_player_velocity,
                despawn_dead,
            ),
        )
        .add_systems(
            Update,
            (
                move_projectiles_system,
                update_velocities_system,
                check_collisions_system,
                inflict_damage_system,
            ),
        )
        .add_systems(Update, tick_timers)
        .run();
}

fn tick_timers(
    mut ship_query: Query<&mut Ship>,
    mut phase_query: Query<&mut Phase>,
    time: Res<Time>,
) {
    for mut ship in ship_query.iter_mut() {
        ship.primary_weapon.cd_timer.tick(time.delta());
        ship.secondary_weapon.cd_timer.tick(time.delta());
    }
    for mut phase in phase_query.iter_mut() {
        phase.cd_timer.tick(time.delta());
        phase.cd_timer.tick(time.delta());
    }
}
