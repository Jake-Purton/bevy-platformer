mod collision;
mod platform;
mod player;

use bevy::prelude::*;
use platform::PlatformPlugin;
use player::{PlayerPlugin, Player, player_movement};

const FELLA_SPRITE: &str = "fella.png";
const SPRITE_SCALE: f32 = 1.0;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE);
const GRAVITY_CONSTANT: f32 = -2800.0;
const MAP: [[u16; 8]; 8] = [
    [1, 1, 1, 1, 1, 1, 1, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 0, 0, 0, 1, 0, 0, 1],
    [1, 0, 0, 0, 0, 0, 1, 1],
    [1, 0, 0, 0, 2, 2, 0, 1],
    [1, 0, 0, 1, 0, 0, 0, 1],
    [1, 0, 1, 0, 0, 0, 0, 1],
    [1, 1, 1, 1, 1, 1, 1, 1],
];
const MAP_SCALE: f32 = 80.0;


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
            },
            ..default()
        }))
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_system(camera_follow_player.after(player_movement))
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .run();
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerCamera;

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands.spawn(Camera2dBundle::default()).insert(PlayerCamera);
    commands.insert_resource(GameTextures {
            player: asset_server.load(FELLA_SPRITE)
        });

    
}

fn camera_follow_player(
    mut camera: Query<&mut Transform, (With<PlayerCamera>, Without<Player>)>, 
    player: Query<&Transform, With<Player>>,
) {

    let mut camera = camera.single_mut();
    let player = player.single();

    (camera.translation.x, camera.translation.y) = (player.translation.x, player.translation.y);
}