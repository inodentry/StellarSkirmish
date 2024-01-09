mod ai_sys;
mod camera_sys;
mod components;
mod events;
mod physics_sys;
mod player_sys;
mod ship_parts;
mod ships;
mod spawn_sys;
mod traits;

use ai_sys::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
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
        .add_plugins((
            DefaultPlugins,
            FrameTimeDiagnosticsPlugin::default(),
            LogDiagnosticsPlugin::default(),
        ))
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
                setup_background_stars_system.after(spawn_camera_system),
                spawn_player_system.after(spawn_camera_system),
                spawn_ship_system.after(spawn_camera_system),
                spawn_drone_system.after(spawn_camera_system),
                //spawn_asteroid_system.after(spawn_camera_system),
            ),
        )
        // Register Events
        .add_event::<DamageEvent>()
        .add_event::<CollisionEvent>()
        .add_event::<SpawnGuidedMissileEvent>()
        .add_event::<MissileDetonationEvent>()
        // Update Systems
        .add_systems(
            Update,
            (
                mouse_world_coords_system,
                player_weapons_system,
                wrap_clipping_location_system,
                despawn_dead_system,
                test_weapon_toggle,
                turret_ai_system,
                speedy_ai_system,
                drone_ai_system,
                rammer_ai_system,
                picket_ai_system,
                handle_self_destruct_system,
                spawn_missile_system,
                guided_missile_ai_system,
            ),
        )
        .add_systems(
            Update,
            (
                move_projectiles_system,
                movement_system,
                update_player_velocity_system,
                inflict_damage_system,
                collision_calculation_system,
                collision_resolution_system,
                check_projectile_collisions,
                check_missile_collisions_system,
                handle_denotation_event_system,
            ),
        )
        .add_systems(Update, tick_timers)
        .run();
}

fn tick_timers(
    mut ship_query: Query<&mut Ship>,
    mut clipping_query: Query<&mut Clipping>,
    mut self_destruct_query: Query<&mut SelfDestruct>,
    time: Res<Time>,
) {
    for mut ship in ship_query.iter_mut() {
        ship.primary_weapon.cd_timer.tick(time.delta());
        ship.secondary_weapon.cd_timer.tick(time.delta());
        ship.tertiary_weapon.cd_timer.tick(time.delta());
    }
    for mut clipping in clipping_query.iter_mut() {
        clipping.cd_timer.tick(time.delta());
    }
    for mut self_destruct in self_destruct_query.iter_mut() {
        self_destruct.cd_timer.tick(time.delta());
    }
}
