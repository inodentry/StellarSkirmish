mod camera_sys;
mod components;
mod events;
mod physics_sys;
mod player_sys;
mod ship_crafting;
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
        // Bevy Plugins
        .add_plugins(DefaultPlugins)
        // Resources
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WorldCoords {
            coords: Vec2::default(),
        })
        // Startup Systems
        .add_systems(
            Startup,
            (
                spawn_camera_system,
                spawn_player_system,
                spawn_asteroid_system,
            ),
        )
        // Register Events
        .add_event::<DamageEvent>()
        // Update Systems
        .add_systems(
            Update,
            (
                mouse_world_coords_system,
                player_weapons_system,
                //confine_player_movement,
                wrap_nophase_location_system,
                update_player_velocity,
                despawn_dead,
                check_projectile_collisions,
                test_weapon_toggle,
            ),
        )
        .add_systems(
            Update,
            (
                move_projectiles_system,
                update_velocities_system,
                check_nophase_collisions,
                inflict_damage_system,
            ),
        )
        .add_systems(Update, tick_timers)
        .run();
}

fn tick_timers(
    mut ship_query: Query<&mut Ship>,
    mut phase_query: Query<&mut NoPhase>,
    time: Res<Time>,
) {
    for mut ship in ship_query.iter_mut() {
        ship.primary_weapon.cd_timer.tick(time.delta());
        ship.secondary_weapon.cd_timer.tick(time.delta());
        ship.tertiary_weapon.cd_timer.tick(time.delta());
    }
    for mut phase in phase_query.iter_mut() {
        phase.cd_timer.tick(time.delta());
    }
}
