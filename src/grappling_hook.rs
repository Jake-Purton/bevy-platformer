// click and it sends out a circle hitbox from the player
// when it hits something 
// - the player is brought towards the object 
// - the object is brought towards the player
// on right click
use bevy::prelude::*;

use crate::{startup_plugin::PlayerCamera, player::Player, GameState};

pub struct GrapplePlugin;

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(send_out_hook)
            );
    }
}

#[derive(Component)]
pub struct GrappleHook {
    direction: Vec2,
}

// sends out a hitbox to act as the hook
fn send_out_hook (
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    camera: Query<&Transform, With<PlayerCamera>>,
    player: Query<&Transform, With<Player>>,
) {
    if mouse.just_pressed(MouseButton::Right) {

        let window = windows.get_primary().unwrap();
        let camera = camera.single();

        if let Some(mut position) = window.cursor_position() {
            position.x -= (window.width() / 2.0) - camera.translation.x;
            position.y -= (window.height() / 2.0) - camera.translation.y;

            let player = player.single();

            let mut direction = position - player.translation.truncate();

            direction /= direction.length();

            // commands.spawn(bundle)

        }
    }
}

// direction * velocity * delta_s