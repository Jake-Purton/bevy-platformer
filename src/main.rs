mod platform;
mod player;
mod death;
mod startup_plugin;
mod next_level;
mod win;
mod main_menu;
mod moving_block;
mod grappling_hook;

use bevy::{prelude::*, render::texture::ImageSampler};
use bevy_rapier2d::prelude::*;
use bevy_kira_audio::prelude::*;
use death::DeathPlugin;
use grappling_hook::GrapplePlugin;
use main_menu::MenuPlugin;
use moving_block::MovingBlockPlugin;
use next_level::NextLevelPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use startup_plugin::StartupPlugin;
use win::WinPlugin;

const SPRITE_SCALE: f32 = 0.707106;
const HOOK_SPRITE_SIZE: Vec2 = Vec2::new(24.0, 24.0);
const HOOK_SPEED: f32 = 2000.0;
const GRAPPLE_SPEED: f32 = 200.0;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE);
const GRAVITY_CONSTANT: Vec2 = Vec2::new(0.0, -1200.0);
const PLAYER_JUMP_VELOCITY: f32 = 800.0;
const PLAYER_RUN_SPEED: f32 = 300.0;
const MAP_SCALE: f32 = 80.0;

pub fn level_directory(level_number: u8) -> String {
    format!("assets/levels/level-{}.txt", level_number)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Gameplay,
    Death,
    NextLevel,
    Win,
    Menu,
}

fn main() {

    println!("Usage: run with \"server\" or \"client\" argument");
    let args: Vec<String> = std::env::args().collect();

    let exec_type = &args[1];
    let is_host = match exec_type.as_str() {
        "client" => false,
        "server" => true,
        _ => panic!("Invalid argument, must be \"client\" or \"server\"."),
    };

    App::new()
        .add_state(GameState::Menu)
        .insert_resource(Msaa { samples: 4 })
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 1080.0,
                    height: 720.0,
                    title: "To do".to_string(),
                    resizable: true,
                    ..Default::default()
                },
                ..default()
            })
            .set(ImagePlugin {
                default_sampler: ImageSampler::nearest_descriptor(),
        }))
        .insert_resource(CurrentLevel {level_number: 1})
        .add_plugin(GrapplePlugin)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(AudioPlugin)
        .add_plugin(DeathPlugin)
        .add_plugin(StartupPlugin)
        .add_plugin(NextLevelPlugin)
        .add_plugin(WinPlugin)
        .add_plugin(MenuPlugin)
        .add_plugin(MovingBlockPlugin)
        // .add_plugin(RapierDebugRenderPlugin::default())
        .run();
}

#[derive(Resource)]
pub struct CurrentLevel {
    level_number: u8,
}