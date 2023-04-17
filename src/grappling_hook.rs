// click and it sends out a circle hitbox from the player
// when it hits something 
// - the player is brought towards the object 
// - the object is brought towards the player
// on right click
use bevy::prelude::*;

use crate::startup_plugin::PlayerCamera;

// sends out a hitbox to act as the hook
fn send_out_hook (
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    camera: Query<&Transform, With<PlayerCamera>>,
) {
    let window = windows.get_primary().unwrap();
    let camera = camera.single();

    if let Some(mut position) = window.cursor_position() {
        position.x -= (window.width() / 2.0) - camera.translation.x;
        position.y -= (window.height() / 2.0) - camera.translation.y;

        if mouse.just_pressed(MouseButton::Right) {

        }
    }
}