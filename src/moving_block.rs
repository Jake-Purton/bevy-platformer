use bevy::{prelude::*, sprite::collide_aabb::{collide}};

use crate::{GameState, platform::Wall, startup_plugin::PlayerCamera, collision::{velocity_collision, VelocityCollision, BetterCollision}};

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
    wall_query: Query<(&Transform, &Wall), Without<MovingWall>>,
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

            for (mut block_transform, _, block) in moving_walls.iter_mut() {

                let pos = Vec3::new(
                    pos.x - (window.width() / 2.0) + camera.translation.x,
                    pos.y - (window.height() / 2.0) + camera.translation.y,
                    block_transform.translation.z,
                );
                let velocity = Vec2::new(pos.x - block_transform.translation.x, pos.y - block_transform.translation.y);
                let mut top_collision = false;
                let mut bottom_collision = false;
                let mut side_collision = false;
                let mut depth: Vec<VelocityCollision> = Vec::new();

                for (wall_transform, wall) in wall_query.iter() {
                    let collision = velocity_collision(
                        block_transform.translation,
                        block.size,
                        velocity,
                        wall_transform.translation,
                        wall.size,
                        Vec2 { x: 0.0, y: 0.0 },
                    );
            
                    if let Some(velocity_collision) = collision {
                        match velocity_collision.collision {
                            BetterCollision::Left => {
                                side_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::Right => {
                                side_collision = true;
                                depth.push(velocity_collision);
                            },
                            BetterCollision::Top => {
                                top_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::Bottom => {
                                bottom_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::TopLeft => {
                                // side_collision = true; 
                                top_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::TopRight => {
                                // side_collision = true; 
                                top_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::BottomRight => {
                                // side_collision = true; 
                                bottom_collision = true;
                                depth.push(velocity_collision);
                            }
                            BetterCollision::BottomLeft => {
                                // side_collision = true; 
                                bottom_collision = true;
                                depth.push(velocity_collision);
                            }
                        }
            
                    }
                }

                depth.sort_by(|a, b| a.depth.abs().partial_cmp(&b.depth.abs()).unwrap());
                depth.reverse();

                if !side_collision {
                    block_transform.translation.x = pos.x;
                } else {
                    let mut new_x = 0.0;
            
                    for i in &depth {
                        if i.collision == BetterCollision::Left || i.collision == BetterCollision::Right {
                            new_x = i.new_position;
                            break;
                        }
                    }
            
                    block_transform.translation.x = new_x;
                }
            
                if top_collision || bottom_collision {

                    let mut new_y = 0.0;
            
                    for i in &depth {
                        if i.collision == BetterCollision::Top || i.collision == BetterCollision::Bottom {
                            new_y = i.new_position;
                            break;
                        }
                    }
            
                    block_transform.translation.y = new_y

                } else {
                    block_transform.translation.y = pos.y;
                }
            }
        } else {
            for (_, entity, _) in moving_walls.iter() {
                commands.entity(entity).remove::<MovingWall>();
            }
        }
    }
}