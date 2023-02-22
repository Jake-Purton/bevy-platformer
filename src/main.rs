mod collision;
mod platform;
mod player;
mod death;
mod startup_plugin;
mod next_level;

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use death::DeathPlugin;
use next_level::NextLevelPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use startup_plugin::StartupPlugin;

const FELLA_SPRITE: &str = "images/fella.png";
const RESPAWN_PNG: &str = "death-messages/respawn.png";
const SPRITE_SCALE: f32 = 0.707106;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE);
const GRAVITY_CONSTANT: f32 = -2800.0;
const MAP_SCALE: f32 = 80.0;

pub fn level_directory(level_number: u8) -> String {
    format!("assets/levels/level-{}.txt", level_number)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Gameplay,
    Death,
    NextLevel,
}

fn main() {
    App::new()
        .add_state(GameState::Gameplay)
        .insert_resource(Msaa { samples: 4 })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            window: WindowDescriptor {
                width: 1080.0,
                height: 720.0,
                title: "To do".to_string(),
                resizable: true,
                ..Default::default()
            },
            ..default()
        }))
        .insert_resource(CurrentLevel {level_number: 4})
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(DeathPlugin)
        .add_plugin(StartupPlugin)
        .add_plugin(NextLevelPlugin)
        .run();
}

#[derive(Resource)]
pub struct CurrentLevel {
    level_number: u8,
}