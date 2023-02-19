mod collision;
mod platform;
mod player;

use bevy::prelude::*;
use platform::{PlatformPlugin, Wall};
use player::PlayerPlugin;
use bevy_ecs_ldtk::{*, prelude::RegisterLdtkObjects};

const FELLA_SPRITE: &str = "fella.png";
const SPRITE_SCALE: f32 = 1.0;
const FELLA_SPRITE_SIZE: Vec2 = Vec2::new(64.0 * SPRITE_SCALE, 64.0 * SPRITE_SCALE);
const FLOOR_HEIGHT: f32 = -300.0;
const GRAVITY_CONSTANT: f32 = -2800.0;

#[derive(Default, Component)]
pub struct LDTK;

#[derive(Bundle, LdtkEntity)]
pub struct MyBundle {
    a: LDTK,
    #[sprite_sheet_bundle]
    #[bundle]
    sprite_bundle: SpriteSheetBundle,
}

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
        .add_plugin(LdtkPlugin)
        .add_startup_system_to_stage(StartupStage::PreStartup, setup)
        .add_plugin(PlayerPlugin)
        .add_plugin(PlatformPlugin)
        .insert_resource(LevelSelection::Index(0))
        .register_ldtk_entity::<MyBundle>("MyEntityIdentifier")
        .run();
}

#[derive(Resource)]
struct GameTextures {
    player: Handle<Image>,
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(GameTextures {
            player: asset_server.load(FELLA_SPRITE)
        });

    commands.spawn(LdtkWorldBundle {
        ldtk_handle: asset_server.load("ldtk_test.ldtk"),
        ..Default::default()
    });
}

fn setup_ldtk (ldtk: Query<&LDTK>) {

}