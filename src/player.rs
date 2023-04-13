use ::bevy::prelude::*;
use bevy::sprite::collide_aabb::collide;
use bevy_rapier2d::prelude::{KinematicCharacterController, KinematicCharacterControllerOutput};

use crate::{
    platform::{LowestPoint, KillerWall},
    GRAVITY_CONSTANT, GameState,
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
    mut controllers: Query<(&mut KinematicCharacterController, &mut Player, &KinematicCharacterControllerOutput)>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for (mut controller, mut player, output) in controllers.iter_mut() {

        let delta_s = time.delta_seconds();
        let mut movement = Vec2::new(0.0, 0.0);

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

        let x = player.velocity.x;
        let y = player.velocity.y;
        let xy = Vec2::new(x, y);

        controller.translation = Some((movement + xy) * delta_s);
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
