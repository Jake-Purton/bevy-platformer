use bevy::{prelude::*, sprite::collide_aabb::collide};

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
            .add_startup_system(platform_system)
            .add_system(moveable_walls)
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
            .insert(Wall { size: $size })
            .insert(MovableWall);
    }};
}

fn platform_system(mut commands: Commands) {
    create_wall!(commands, 0.0, -100.0, Vec2::new(100.0, 100.0));
    create_wall!(commands, 150.0, -200.0, Vec2::new(200.0, 100.0));
    create_wall!(commands, -150.0, -250.0, Vec2::new(200.0, 100.0));

    // create_platform!(commands, 0.0, 0.0, Vec2::new(500.0, 500.0))
}

fn moveable_walls (
    walls: Query<(&Transform, &Wall, Entity), With<MovableWall>>,
    mouse: Res<Input<MouseButton>>,
    windows: Res<Windows>,
    mut commands: Commands,
) {

    let window = windows.get_primary().unwrap();

    if let Some(mut position) = window.cursor_position() {

        position.x -= window.width() / 2.0;
        position.y -= window.height() / 2.0;

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
    mut commands: Commands,
) {
    if !moving_walls.is_empty() {
        if mouse.pressed(MouseButton::Left) {

            let window = windows.get_primary().unwrap();
            let pos = window.cursor_position().unwrap();
    
            for (mut transform, _) in moving_walls.iter_mut() {
    
                let pos = Vec3::new(pos.x - window.width() / 2.0, pos.y - window.height() / 2.0, transform.translation.z);
                transform.translation = pos;
    
            }
        } else {
            for (_, entity) in moving_walls.iter() {
                commands.entity(entity).remove::<MovingWall>();
            }
        }
    }
}
