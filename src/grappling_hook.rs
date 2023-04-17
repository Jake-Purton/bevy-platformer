// click and it sends out a circle hitbox from the player
// when it hits something 
// - the player is brought towards the object 
// - the object is brought towards the player
// on right click
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{startup_plugin::{PlayerCamera, GameTextures}, player::Player, GameState, HOOK_SPRITE_SIZE, platform::Wall};

pub struct GrapplePlugin;

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(send_out_hook)
                    .with_system(hook_sensor)
            );
    }
}

#[derive(Component)]
pub struct GrappleHook {
    direction: Vec2,
    size: Vec2,
}

// sends out a hitbox to act as the hook
fn send_out_hook (
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    camera: Query<&Transform, With<PlayerCamera>>,
    player: Query<&Transform, With<Player>>,
    game_textures: Res<GameTextures>,
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

            let angle = (direction.y / direction.x).tan();

            commands
                .spawn(SpriteBundle {
                    texture: game_textures.hook.clone(),
                    sprite: Sprite {
                        custom_size: Some(HOOK_SPRITE_SIZE),
                        ..Default::default()
                    },
                    transform: Transform {
                        translation: player.translation + (20.0 * direction).extend(11.0),
                        rotation: Quat::from_rotation_z(angle),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .insert(GrappleHook {
                    direction,
                    size: HOOK_SPRITE_SIZE,
                });
        }
    }
}

fn hook_sensor (
    hooks: Query<(&GrappleHook, &Transform)>,
    walls: Query<(&Wall, &Transform)>
) {

    for (hook, hook_transform) in hooks.iter() {

        for (wall, wall_transform) in walls.iter() {

            if collide(
                hook_transform.translation, 
                hook.size, 
                wall_transform.translation, 
                wall.size
            ).is_some() {

                println!("hook collision")

            }
        }
    }
}