use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::components::*;

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let window: &Window = window_query.get_single().unwrap();
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(window.width() / 2.0, window.height() / 2.0, 0.0),
            texture: asset_server.load("sprites/ships/playerShip1_blue.png"),
            ..default()
        },
        Player {},
        Ship {speed: 400.0, angle: f32::to_radians(90.0), turn_speed: f32::to_radians(1.25)}
    ));
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Ship, &mut Transform), With<Player>>,
    time: Res<Time>,
) {
    if let Ok((mut ship, mut transform)) = player_query.get_single_mut() {

        let mut direction = Vec3::ZERO;

        if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
            let move_dir = transform.up() * ship.speed * time.delta_seconds();
            transform.translation += move_dir;
        }
        if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
            // Using angles, so if turning left hits 360.0 degrees, it wraps around to 0.0.
            transform.rotate_z(ship.turn_speed);
            println!("Angle: {:?}", transform.rotation);
        }
        if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
            // Using angles, so if turning right hits 0.0 degrees, it wraps around to 360.0.
            transform.rotate_z(-ship.turn_speed);
            println!("Angle: {:?}", transform.rotation);
        }

        if direction.length() > 0.0 {
            direction = direction.normalize();
        }

        transform.translation += direction * ship.speed * time.delta_seconds();
    }
}