use bevy::prelude::*;

use crate::{GameState, startup_plugin::{GameTextures, despawn_everything}};

pub struct WinPlugin;

impl Plugin for WinPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Win)
                    .with_system(setup_win_screen)
            )
            // .add_system_set(
            //     SystemSet::on_update(GameState::Win)
            // )
            .add_system_set(
                SystemSet::on_exit(GameState::Win)
                    .with_system(despawn_everything)
            )
            ;
    }
}

fn setup_win_screen (
    mut commands: Commands,
    game_textures: Res<GameTextures>,
) {
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: game_textures.you_win.clone(),
        transform: Transform {
            translation: Vec3::new(0.0, 0.0, 20.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..Default::default()
        },
        ..Default::default()
    });
}