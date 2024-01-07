// use bevy::core_pipeline::bloom::BloomSettings;
use crate::components::Player;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera(
    mut commands: Commands,
    mut window_query: Query<&mut Window, With<PrimaryWindow>>,
) {
    // Also setting the resolution to 1200x900 here so we have more screen to work.
    // Window resizing will probably get moved later on.
    let mut window = window_query.get_single_mut().unwrap();
    window.resolution.set(1200.0, 900.0);

    commands.spawn((
        Camera2dBundle {
            // camera: Camera {
            //     hdr: true,
            //     ..default()
            // },
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            ..default()
        },
        // BloomSettings::OLD_SCHOOL,
    ));
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size = 32.0;
        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_min;
        } else if translation.x > x_max {
            translation.x = x_max;
        }

        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_min;
        } else if translation.y > y_max {
            translation.y = y_max;
        }

        player_transform.translation = translation;
    }
}

pub fn wrap_player_location(
    mut player_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0;
        let x_max = window.width();
        let y_min = 0.0;
        let y_max = window.height();

        let mut translation = player_transform.translation;

        // Bound the player x position
        if translation.x < x_min {
            translation.x = x_max;
        } else if translation.x > x_max {
            translation.x = x_min;
        }

        // Bound the player y position
        if translation.y < y_min {
            translation.y = y_max;
        } else if translation.y > y_max {
            translation.y = y_min;
        }

        player_transform.translation = translation;
    }
}
