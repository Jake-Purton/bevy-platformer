use ::bevy::prelude::*;
use bevy::sprite::collide_aabb::Collision;

use crate::{
    collision::velocity_collision, platform::Wall, FELLA_SPRITE, FELLA_SPRITE_SIZE, FLOOR_HEIGHT,
    SPRITE_SCALE,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_fella)
            .add_system(player_movement)
            .add_system(respawn_system);
    }
}

#[derive(Component)]
pub struct Player {
    run_speed: f32,
    velocity: Vec2,
    jump_velocity: f32,
    can_jump: bool,
    size: Vec2,
}

fn respawn_system (
    mut query: Query<&mut Transform, With<Player>>, 
    keys: Res<Input<KeyCode>>
) {

    if keys.just_pressed(KeyCode::R) {

        query.single_mut().translation = Vec3::new(0.0, 0.0, 10.0);

    }
}

fn spawn_fella(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn(SpriteBundle {
            texture: asset_server.load(FELLA_SPRITE),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 0.0),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Player {
            run_speed: 500.0,
            velocity: Vec2 { x: 0.0, y: 0.0 },
            jump_velocity: 500.0,
            can_jump: true,
            size: FELLA_SPRITE_SIZE,
        });
}

fn player_movement(
    mut player_query: Query<(&mut Player, &mut Transform)>,
    wall_query: Query<(&Transform, &Wall), Without<Player>>,
    keyboard: Res<Input<KeyCode>>,
    time: Res<Time>,
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
    let mut depth: Vec<(Collision, f32)> = Vec::new();

    for (wall_transform, wall) in wall_query.iter() {
        let collision = velocity_collision(
            transform.translation,
            player.size,
            Vec2::new(x_movement, y_movement),
            wall_transform.translation,
            wall.size,
            Vec2 { x: 0.0, y: 0.0 },
        );

        if let Some(collision) = collision {


            match collision.0 {
                Collision::Left => side_collision = true,
                Collision::Right => side_collision = true,
                Collision::Top => top_collision = true,
                Collision::Bottom => bottom_collision = true,
                Collision::Inside => (),
            }

            depth.push(collision);
        }
    }

    if target.y - player.size.y / 2.0 < FLOOR_HEIGHT {
        top_collision = true;
        depth.push((Collision::Top, 0.0001 + FLOOR_HEIGHT + player.size.y / 2.0))
    }

    if !side_collision {
        transform.translation.x = target.x;
    } else {

        let mut new_x = 0.0;

        for i in &depth {
            if i.0 == Collision::Left || i.0 == Collision::Right {
                new_x = i.1; 
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
            if i.0 == Collision::Top {
                new_y = i.1; 
            }
        }

        transform.translation.y = new_y

    } else if !bottom_collision {

        // if not on the floor or on the celing
        player.can_jump = false;
        transform.translation.y = target.y;
        player.velocity.y -= 1000.0 * time_delta;

    } else {

        player.velocity.y = 0.0;
        player.can_jump = false;
        let mut new_y = 0.0;

        for i in &depth {
            if i.0 == Collision::Bottom {
                new_y = i.1; 
            }
        }

        transform.translation.y = new_y

    }
}
