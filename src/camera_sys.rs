// use bevy::core_pipeline::bloom::BloomSettings;
use crate::components::*;
use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub fn spawn_camera_system(
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
        MainCamera {},
        // BloomSettings::OLD_SCHOOL,
    ));
}

pub fn confine_player_movement_system(
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

pub fn wrap_clipping_location_system(
    mut nophase_query: Query<&mut Transform, With<Clipping>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    for mut transform in nophase_query.iter_mut() {
        let window = window_query.get_single().unwrap();

        let x_min = 0.0;
        let x_max = window.width();
        let y_min = 0.0;
        let y_max = window.height();

        let mut translation = transform.translation;

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

        transform.translation = translation;
    }
}

pub fn mouse_world_coords_system(
    // Adapted from Ida Iyes' code
    mut coords: ResMut<WorldCoords>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // query to get camera transform
    camera_query: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so Query::single() is OK
    let (camera, camera_transform) = camera_query.single();

    // There is only one primary window, so we can similarly get it from the query:
    let window = window_query.single();

    // check if the cursor is inside the window and get its position
    // then, ask bevy to convert into world coordinates, and truncate to discard Z
    if let Some(world_position) = window
        .cursor_position()
        .and_then(|cursor| camera.viewport_to_world(camera_transform, cursor))
        .map(|ray| ray.origin.truncate())
    {
        coords.coords = world_position;
    }
}
