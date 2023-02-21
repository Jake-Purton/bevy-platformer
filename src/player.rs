use ::bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

use crate::{
    collision::{velocity_collision, VelocityCollision},
    platform::Wall,
    GRAVITY_CONSTANT, GameState,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(player_movement)
            );
    }
}

#[derive(Component)]
pub struct Player {
    pub run_speed: f32,
    pub velocity: Vec2,
    pub jump_velocity: f32,
    pub can_jump: bool,
    pub size: Vec2,
}

pub fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<(&Transform, &Wall), Without<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (mut player, mut transform) = player_query.single_mut();

    let time_delta = time.delta_seconds();

    if keyboard.pressed(KeyCode::Space) && player.can_jump {
        player.velocity.y += player.jump_velocity;
    }

    let mut delta_x = 0.0;

    if keyboard.pressed(KeyCode::D) {
        delta_x += player.run_speed * time_delta;
    }

    if keyboard.pressed(KeyCode::A) {
        delta_x -= player.run_speed * time_delta;
    }

    let y_movement = player.velocity.y * time_delta;
    let x_movement = player.velocity.x * time_delta + delta_x;
    let target = transform.translation + Vec3::new(x_movement, y_movement, 0.0);

    // calculate y collisions / borders

    let mut top_collision = false;
    let mut bottom_collision = false;
    let mut side_collision = false;
    let mut depth: Vec<(VelocityCollision, bool)> = Vec::new();

    for (wall_transform, wall) in wall_query.iter() {
        let collision = velocity_collision(
            transform.translation,
            player.size,
            Vec2::new(x_movement, y_movement),
            wall_transform.translation,
            wall.size,
            Vec2 { x: 0.0, y: 0.0 },
        );

        if let Some(velocity_collision) = collision {
            match velocity_collision.collision {
                Collision::Left => side_collision = true,
                Collision::Right => side_collision = true,
                Collision::Top => top_collision = true,
                Collision::Bottom => bottom_collision = true,
                Collision::Inside => (),
            }

            depth.push((velocity_collision, wall.killer));
        }
    }

    depth.sort_by(|a, b| a.0.depth.abs().partial_cmp(&b.0.depth.abs()).unwrap());
    depth.reverse();

    if !side_collision {
        transform.translation.x = target.x;
    } else {
        let mut new_x = 0.0;

        for i in &depth {
            if i.0.collision == Collision::Left || i.0.collision == Collision::Right {
                if i.1 {
                    match game_state.set(GameState::End) {
                        Ok(a) => a,
                        Err(a) => println!("{a}"),
                    }
                }
                new_x = i.0.new_position;
                break;
            }
        }

        transform.translation.x = new_x;
    }

    if top_collision {
        // on the floor
        player.can_jump = true;
        let mut new_y = 0.0;
        player.velocity.y = 0.0;

        for i in &depth {
            if i.0.collision == Collision::Top {
                if i.1 {
                    match game_state.set(GameState::End) {
                        Ok(a) => a,
                        Err(a) => println!("{a}"),
                    }
                }
                new_y = i.0.new_position;
                break;
            }
        }

        transform.translation.y = new_y
    } else if !bottom_collision {
        // if not on the floor or on the celing
        player.can_jump = false;
        transform.translation.y = target.y;
        player.velocity.y += GRAVITY_CONSTANT * time_delta;
    } else {
        player.velocity.y = 0.0;
        player.can_jump = false;
        let mut new_y = 0.0;

        for i in &depth {
            if i.0.collision == Collision::Bottom {
                if i.1 {
                    match game_state.set(GameState::End) {
                        Ok(a) => a,
                        Err(a) => println!("{a}"),
                    }
                }
                new_y = i.0.new_position;
                break;
            }
        }

        transform.translation.y = new_y
    }
}
