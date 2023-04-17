// click and it sends out a circle hitbox from the player
// when it hits something 
// - the player is brought towards the object 
// - the object is brought towards the player
// on right click
use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{startup_plugin::{PlayerCamera, GameTextures}, player::Player, GameState, HOOK_SPRITE_SIZE, platform::Wall, HOOK_SPEED};

pub struct GrapplePlugin;

impl Plugin for GrapplePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(send_out_hook)
                    .with_system(hook_sensor.after(hook_movement))
                    .with_system(hook_movement)
                    .with_system(player_movement_by_hook)
            );
    }
}

#[derive(Component)]
pub struct MovingGrappleHook {
    direction: Vec2,
    size: Vec2,
    timer: Timer,
}

#[derive(Component)]
pub struct Hook;

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

            let angle = Vec2::Y.angle_between(direction);

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
                .insert(MovingGrappleHook {
                    direction,
                    size: HOOK_SPRITE_SIZE,
                    timer: Timer::from_seconds(0.7, TimerMode::Once)
                })
                .insert(Hook);
        }
    }
}

fn hook_sensor (
    hooks: Query<(Entity, &MovingGrappleHook, &Transform)>,
    walls: Query<(&Wall, &Transform)>,
    mut commands: Commands,
) {

    for (entity, hook, hook_transform) in hooks.iter() {

        for (wall, wall_transform) in walls.iter() {

            if collide(
                hook_transform.translation, 
                hook.size, 
                wall_transform.translation, 
                wall.size
            ).is_some() {

                commands.entity(entity).remove::<MovingGrappleHook>();

            }
        }
    }
}

fn hook_movement (
    mut hooks: Query<(Entity, &mut MovingGrappleHook, &mut Transform)>,
    time: Res<Time>,
    mut commands: Commands,
) {

    for (entity, mut hook, mut transform)in hooks.iter_mut() {

        hook.timer.tick(time.delta());

        if hook.timer.just_finished() {

            commands.entity(entity).despawn();

        } else {

            transform.translation += (HOOK_SPEED * hook.direction * time.delta_seconds()).extend(0.0);

        }


    }
    
}