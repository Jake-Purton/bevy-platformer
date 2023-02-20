use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{PlayerCamera, MAP, MAP_SCALE};

#[derive(Component)]
pub struct Wall {
    pub size: Vec2,
}

#[derive(Component)]
pub struct MovableWall;

#[derive(Component)]
pub struct MovingWall;

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_startup_system(platform_from_map_system)
            .add_system(movable_walls)
            .add_system(moving_wall)
            ;
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
            .insert(Wall { size: $size });
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
            .insert(Wall { size: $size })
            .insert(MovableWall);
    }};
}

fn platform_from_map_system(mut commands: Commands) {

    for (y, array) in MAP.iter().enumerate() {
        for (x, val) in array.iter().enumerate() {
            if *val == 1 {
                create_wall!(
                    commands, 
                    (x as f32 * MAP_SCALE) - MAP[0].len() as f32 * MAP_SCALE / 2.0, 
                    (y as f32 * MAP_SCALE) - MAP.len() as f32 * MAP_SCALE / 2.0, 
                    Vec2::new(MAP_SCALE, MAP_SCALE)
                )
            } else if *val == 2 {
                create_movable_wall!(
                    commands, 
                    (x as f32 * MAP_SCALE) - MAP[0].len() as f32 * MAP_SCALE / 2.0, 
                    (y as f32 * MAP_SCALE) - MAP.len() as f32 * MAP_SCALE / 2.0, 
                    Vec2::new(MAP_SCALE, MAP_SCALE)
                )
            }
        }
    }
    // create_platform!(commands, 0.0, 0.0, Vec2::new(500.0, 500.0))
}

fn movable_walls (
    walls: Query<(&Transform, &Wall, Entity), With<MovableWall>>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
    camera: Query<&Transform, With<PlayerCamera>>
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
                    Vec2::new(1.0, 1.0)
                ).is_some() {
                    commands.entity(entity).insert(MovingWall);
                    break;
                }
            }
        }
    }
}

fn moving_wall (
    mut moving_walls: Query<(&mut Transform, Entity), With<MovingWall>>,
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
    
            for (mut transform, _) in moving_walls.iter_mut() {
    
                let pos = Vec3::new(
                    pos.x - (window.width() / 2.0) + camera.translation.x, 
                    pos.y - (window.height() / 2.0) + camera.translation.y, 
                    transform.translation.z
                );
                transform.translation = pos;
    
            }
        } else {
            for (_, entity) in moving_walls.iter() {
                commands.entity(entity).remove::<MovingWall>();
            }
        }
    }
}
