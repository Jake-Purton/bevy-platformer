use bevy::prelude::*;

use crate::{GameState, startup_plugin::{despawn_everything, GameTextures}};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Menu)
                    .with_system(setup_menu)
            )
            // .add_system_set(
            //     SystemSet::on_update(GameState::Menu)
            // )
            .add_system_set(
                SystemSet::on_exit(GameState::Menu)
                    .with_system(despawn_everything)
            )
            ;
    }
}

fn setup_menu (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
    windows: Res<Windows>,
) {

    let window = windows.get_primary().unwrap();

    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: game_textures.menu.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, window.height() / 4.0, 20.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: game_textures.play.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 20.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });

    commands.spawn(SpriteBundle {
        texture: game_textures.exit.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, -(window.height() / 4.0), 20.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}