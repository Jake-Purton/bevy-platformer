use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{GameState, platform::Wall, startup_plugin::PlayerCamera, collision::velocity_collision};

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
pub struct MovableWall;

#[derive(Component)]
pub struct MovingWall;

fn movable_walls(
    walls: Query<(&Transform, &Wall, Entity), With<MovableWall>>,
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
    mut moving_walls: Query<(&mut Transform, Entity, &Wall), With<MovingWall>>,
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

            for (mut transform, _, wall) in moving_walls.iter_mut() {

                let pos = Vec3::new(
                    pos.x - (window.width() / 2.0) + camera.translation.x,
                    pos.y - (window.height() / 2.0) + camera.translation.y,
                    transform.translation.z,
                );
                let velocity = Vec2::new(pos.x - transform.translation.x, pos.y - transform.translation.y);

                if velocity_collision(
                    transform.translation, 
                    wall.size, 
                    velocity, 
                    Vec3::new(0.0, 0.0, 0.0), 
                    Vec2::new(80.0, 80.0), 
                    Vec2::new(0.0, 0.0)
                ).is_none() {
                    transform.translation = pos;
                }
            }
        } else {
            for (_, entity, _) in moving_walls.iter() {
                commands.entity(entity).remove::<MovingWall>();
            }
        }
    }
}