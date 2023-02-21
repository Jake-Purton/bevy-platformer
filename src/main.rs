mod collision;
mod platform;
mod player;

use bevy::prelude::*;
use bevy_kira_audio::{prelude::*, Audio};
use platform::PlatformPlugin;
use player::{player_movement, Player, PlayerPlugin};

const FELLA_SPRITE: &str = "fella.png";
const SPRITE_SCALE: f32 = 0.707106;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE);
const GRAVITY_CONSTANT: f32 = -2800.0;
const MAP: &str = "assets/map.txt";
const MAP_SCALE: f32 = 80.0;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Copy)]
pub enum GameState {
    Gameplay,
    End,
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
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_system_set(
            SystemSet::on_update(GameState::Gameplay)
                .with_system(camera_follow_player.after(player_movement))
                .with_system(toggle_mute)
        )
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .add_plugin(AudioPlugin)
        .run();
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>, audio: Res<Audio>) {
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands
        .spawn(Camera2dBundle::default())
        .insert(PlayerCamera);
    commands.insert_resource(GameTextures {
        player: asset_server.load(FELLA_SPRITE),
    });

    let music = asset_server.load("chordy.wav");
    audio.play(music).looped().with_volume(0.2);
}

fn toggle_mute (audio: Res<Audio>, keys: Res<Input<KeyCode>>) {

    if keys.just_pressed(KeyCode::M) {
        if audio.is_playing_sound() {
            audio.pause();
        } else {
            audio.resume();
        }
    }
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let mut camera = camera.single_mut();
    let player = player.single();
    (camera.translation.x, camera.translation.y) = (player.translation.x, player.translation.y);
}
