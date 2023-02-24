use crate::moving_block::MovableWall;
use std::{fs::File, io::Read};

use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    player::Player, FELLA_SPRITE_SIZE, MAP_SCALE, SPRITE_SCALE, GameState, startup_plugin::GameTextures, CurrentLevel, level_directory,
};

#[derive(Component)]
pub struct Wall {
    pub size: Vec2,
    pub killer: bool,
}

#[derive(Component)]
pub struct Goal {
    size: Vec2
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(platform_from_map_system)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(next_level_system)
            );
    }
}

macro_rules! create_wall {
    ($commands:expr, $x:expr, $y:expr, $size:expr) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 1.0, 1.0, 1.0),
                    custom_size: Some($size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: $x,
                        y: $y,
                        z: 10.0,
                    },
                    ..default()
                },
                ..Default::default()
            })
            .insert(Wall {
                size: $size,
                killer: false,
            });
    }};
}

macro_rules! create_level_end {
    ($commands:expr, $x:expr, $y:expr, $size:expr) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 1.0, 0.0, 1.0),
                    custom_size: Some($size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: $x,
                        y: $y,
                        z: 10.0,
                    },
                    ..default()
                },
                ..Default::default()
            }).insert(Goal {size: $size});
    }}
}

macro_rules! create_killer_wall {
    ($commands:expr, $x:expr, $y:expr, $size:expr) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(1.0, 0.0, 0., 1.0),
                    custom_size: Some($size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: $x,
                        y: $y,
                        z: 10.0,
                    },
                    ..default()
                },
                ..Default::default()
            })
            .insert(Wall {
                size: $size,
                killer: true,
            });
    }};
}

macro_rules! create_movable_wall {
    ($commands:expr, $x:expr, $y:expr, $size:expr) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgba(0.0, 1.0, 1.0, 0.7),
                    custom_size: Some($size),
                    ..default()
                },
                transform: Transform {
                    translation: Vec3 {
                        x: $x,
                        y: $y,
                        z: 15.0,
                    },
                    ..default()
                },
                ..Default::default()
            })
            .insert(Wall {
                size: $size,
                killer: false,
            })
            .insert(MovableWall);
    }};
}

#[derive(Resource)]
pub struct LowestPoint {
    pub point: f32,
}

fn platform_from_map_system(
    mut commands: Commands, 
    game_textures: Res<GameTextures>,
    current_level: Res<CurrentLevel>,
) {

    let mut file = File::open(level_directory(current_level.level_number)).unwrap();

    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut map: Vec<Vec<u8>> = Vec::new();

    for line in contents.lines() {
        map.push(
            line.split_whitespace()
                .map(|a| a.parse::<u8>().unwrap())
                .collect(),
        );
    }

    map.reverse();
    commands.insert_resource(LowestPoint{ point: (map.len() as f32 * MAP_SCALE / 2.0) + MAP_SCALE + 100.0 });

    for (y, array) in map.iter().enumerate() {
        for (x, val) in array.iter().enumerate() {
            if *val == 1 {
                create_wall!(
                    commands,
                    (x as f32 * MAP_SCALE) - map[0].len() as f32 * MAP_SCALE / 2.0,
                    (y as f32 * MAP_SCALE) - map.len() as f32 * MAP_SCALE / 2.0,
                    Vec2::new(MAP_SCALE, MAP_SCALE)
                )
            } else if *val == 2 {
                create_movable_wall!(
                    commands,
                    (x as f32 * MAP_SCALE) - map[0].len() as f32 * MAP_SCALE / 2.0,
                    (y as f32 * MAP_SCALE) - map.len() as f32 * MAP_SCALE / 2.0,
                    Vec2::new(MAP_SCALE, MAP_SCALE)
                )
            } else if *val == 3 {
                let x = (x as f32 * MAP_SCALE) - map[0].len() as f32 * MAP_SCALE / 2.0;
                let y = (y as f32 * MAP_SCALE) - map.len() as f32 * MAP_SCALE / 2.0;

                commands
                    .spawn(SpriteBundle {
                        texture: game_textures.player.clone(),
                        transform: Transform {
                            translation: Vec3::new(x, y, 20.0),
                            scale: Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    })
                    .insert(Player {
                        run_speed: 800.0,
                        velocity: Vec2 { x: 0.0, y: 0.0 },
                        jump_velocity: 1000.0,
                        can_jump: true,
                        size: FELLA_SPRITE_SIZE,
                    });
            } else if *val == 4 {
                create_killer_wall!(
                    commands,
                    (x as f32 * MAP_SCALE) - map[0].len() as f32 * MAP_SCALE / 2.0,
                    (y as f32 * MAP_SCALE) - map.len() as f32 * MAP_SCALE / 2.0,
                    Vec2::new(MAP_SCALE, MAP_SCALE - 10.0)
                )
            } else if *val == 5 {
                create_level_end!(
                    commands,
                    (x as f32 * MAP_SCALE) - map[0].len() as f32 * MAP_SCALE / 2.0,
                    (y as f32 * MAP_SCALE) - map.len() as f32 * MAP_SCALE / 2.0,
                    Vec2::new(MAP_SCALE, MAP_SCALE)
                )
            }
        }
    }    
}

fn next_level_system (
    player: Query<(&Player, &Transform)>, 
    goals: Query<(&Goal, &Transform)>, 
    mut level: ResMut<CurrentLevel>,
    mut game_state: ResMut<State<GameState>>,
) {
    let (player, player_transform) = player.single();
    for (goal, goal_transform) in goals.iter(){
        if collide(
            player_transform.translation, 
            player.size, 
            goal_transform.translation, 
            goal.size)
            .is_some()
        {
            level.level_number += 1;
            match game_state.set(GameState::NextLevel) {
                Ok(a) => a,
                Err(a) => println!("{a}, (gameplay to next level)"),
            }
        }
    }
}
