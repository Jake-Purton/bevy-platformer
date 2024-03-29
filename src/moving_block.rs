use bevy::{prelude::*, sprite::collide_aabb::collide};
use bevy_rapier2d::prelude::Velocity;

use crate::{GameState, startup_plugin::PlayerCamera};

pub struct MovingBlockPlugin;

impl Plugin for MovingBlockPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(movable_walls)
                    .with_system(moving_wall)
            );
    }
}

// ideas:
// walls that the block cannot go through but the player can
// blocks that fall when not being held
// button

#[derive(Component)]
pub struct MovableWall {
    pub size: Vec2
}

#[derive(Component)]
pub struct MovingWall;

fn movable_walls(
    walls: Query<(&Transform, &MovableWall, Entity)>,
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

        if mouse.just_pressed(MouseButton::Left) {
            for (transform, wall, entity) in walls.iter() {
                if collide(
                    transform.translation,
                    wall.size,
                    Vec3::new(position.x, position.y, 0.0),
                    Vec2::new(1.0, 1.0),
                )
                .is_some()
                {
                    commands.entity(entity).insert(MovingWall);
                    break;
                }
            }
        }
    }
}

fn moving_wall(
    mut moving_walls: Query<(&mut Velocity, Entity, &Transform), With<MovingWall>>,
    windows: Res<Windows>,
    mouse: Res<Input<MouseButton>>,
    camera: Query<&Transform, (With<PlayerCamera>, Without<MovingWall>)>,
    mut commands: Commands,
) {
    if !moving_walls.is_empty() {
        if mouse.pressed(MouseButton::Left) {
            let camera = camera.single();
            let window = windows.get_primary().unwrap();
            let pos = window.cursor_position().unwrap();

            for (mut vel, _, block_transform) in moving_walls.iter_mut() {

                let pos = Vec3::new(
                    pos.x - (window.width() / 2.0) + camera.translation.x,
                    pos.y - (window.height() / 2.0) + camera.translation.y,
                    block_transform.translation.z,
                );
                let velocity = (pos - block_transform.translation).truncate();
                vel.linvel = (velocity + vel.linvel) * 0.8;
            }   
        } else {
            for (_, entity, _) in moving_walls.iter() {
                commands.entity(entity).remove::<MovingWall>();
            }
        }
    }
}