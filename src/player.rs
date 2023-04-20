use ::bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{
    platform::{LowestPoint, KillerWall},
    GRAVITY_CONSTANT, GameState, grappling_hook::{Hook, MovingGrappleHook},
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(rapier_player_movement)
                    .with_system(player_death_fall_off_the_map)
                    .with_system(killer_wall)
            );
    }
}

#[derive(Component)]
pub struct Player {
    pub run_speed: f32,
    pub velocity: Vec2,
    pub jump_velocity: f32,
    pub size: Vec2,
}

pub fn rapier_player_movement (
    mut controllers: Query<(
        &mut KinematicCharacterController, 
        &mut Player, 
        &KinematicCharacterControllerOutput,
        &Transform,
    )>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    grappling_hook: Query<(&Hook, &Transform), Without<MovingGrappleHook>>,
) {
    for (mut controller, mut player, output, player_transform) in controllers.iter_mut() {

        let delta_s = time.delta_seconds();

        let mut movement = Vec2::new(0.0, 0.0);

        // make sure it hits the ceiling
        if output.effective_translation.y.is_sign_positive() && (output.effective_translation.y * 10.0).round() == 0.0 {
            player.velocity.y = 0.0;
        }

        if keys.pressed(KeyCode::D) {
            movement += Vec2::new(player.run_speed, 0.0);
        }        
        if keys.pressed(KeyCode::A) {
            movement += Vec2::new(-player.run_speed, 0.0);
        }        

        if !output.grounded {
            player.velocity += GRAVITY_CONSTANT * delta_s;
        } else {
            if keys.pressed(KeyCode::Space) {
                player.velocity.y = player.jump_velocity;
            } else {
                player.velocity.y = 0.0;
            }
        }

        movement += player.velocity;

        if grappling_hook.is_empty() {

            controller.translation = Some(movement * delta_s);

        } else {

            let (hook, hook_transform) = grappling_hook.single();

            // find hook direction, angle of velocity to hook normal, resolve forces each frame?

            let mut direction = (hook_transform.translation - player_transform.translation).truncate();
            let angle = direction.angle_between(Vec2::X);

            let resolved_x = movement.x * 

        }

    }
}

fn player_death_fall_off_the_map (
    player: Query<&Transform, With<Player>>,
    lowest_point: Res<LowestPoint>,
    mut game_state: ResMut<State<GameState>>,
) {

    let player = player.single();
    if player.translation.y <= -lowest_point.point {
        match game_state.set(GameState::Death) {
            Ok(a) => a,
            Err(a) => println!("{a}, player_fall_off_map"),
        }
    }
}

fn killer_wall (
    walls: Query<(&KillerWall, &Transform)>,
    player: Query<(&Transform, &Player)>,
    mut game_state: ResMut<State<GameState>>,
) {

    let player = player.single();

    for wall in walls.iter() {
        if collide(wall.1.translation, wall.0.size + Vec2::ONE, player.0.translation, player.1.size).is_some() {
            match game_state.set(GameState::Death) {
                Ok(a) => a,
                Err(a) => println!("{a}, player_dead_to_killer_walls"),
            }
        }
    }
}
