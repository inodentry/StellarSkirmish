// use bevy::core_pipeline::bloom::BloomSettings;
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
