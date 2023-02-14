use ::bevy::prelude::*;

#[derive(Component)]
pub struct Wall {
    pub size: Vec2,
}

pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(platform_system);
    }
}

macro_rules! create_platform {
    ($commands:expr , $x:expr, $y:expr, $size:expr) => {{
        $commands
            .spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.0, 0.0),
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

fn platform_system(mut commands: Commands) {
    create_platform!(commands, 0.0, -100.0, Vec2::new(100.0, 100.0));
    create_platform!(commands, 150.0, -200.0, Vec2::new(200.0, 100.0));
    create_platform!(commands, -150.0, -250.0, Vec2::new(200.0, 100.0));
}
