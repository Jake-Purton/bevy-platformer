mod player;
mod platform;
mod collision;

use bevy::prelude::*;
use player::PlayerPlugin;
use platform::PlatformPlugin;

const FELLA_SPRITE: &str = "fella.png";
const SPRITE_SCALE: f32 = 1.0;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0, 64.0);
const FLOOR_HEIGHT: f32 = -300.0;

fn main() {

    App::new()
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1080.0,
                height: 720.0,
                title: "To do".to_string(),
                resizable: true,
                ..Default::default()
            }, ..default()
        }))
        .add_startup_system(setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .run();
}

fn setup(mut commands: Commands){
    
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands.spawn(Camera2dBundle::default());

}

