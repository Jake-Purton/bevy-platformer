use bevy::prelude::*;

use crate::{GameState, startup_plugin::{despawn_everything, GameTextures}};

pub struct DeathPlugin;

impl Plugin for DeathPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::Death)
                    .with_system(setup_death)
            )
            .add_system_set(
                SystemSet::on_update(GameState::Death)
                    .with_system(restart)
                    .with_system(background)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Death)
                    .with_system(despawn_everything)
            )
            ;
    }
}

fn setup_death(mut commands: Commands, game_textures: Res<GameTextures>) {
        commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
        commands.spawn(Camera2dBundle::default());
        commands.spawn(SpriteBundle {
            texture: game_textures.r_to_respawn.clone(),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 20.0),
                scale: Vec3::new(1.0, 1.0, 1.0),
                ..Default::default()
            },
            ..Default::default()
        });
}

fn restart (keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::R) {
        game_state.set(GameState::Gameplay).unwrap();
    }
}

fn background (time: Res<Time>, mut commands: Commands) {

    let time = (time.elapsed_seconds() * 2.0).sin() / 8.0;
    commands.insert_resource(ClearColor(Color::rgb(1.0 + time, 0.6 - time, 0.0)));

}