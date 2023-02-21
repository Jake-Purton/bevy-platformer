use bevy::prelude::*;

use crate::{GameState, startup_plugin::despawn_everything};

pub struct EndPlugin;

impl Plugin for EndPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::End)
                    .with_system(setup_end)
            )
            .add_system_set(
                SystemSet::on_update(GameState::End)
                    .with_system(restart)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::End)
                    .with_system(despawn_everything)
            )
            ;
    }
}

fn setup_end(mut commands: Commands) {
        commands.insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)));
        commands.spawn(Camera2dBundle::default());
}

fn restart (keys: Res<Input<KeyCode>>, mut game_state: ResMut<State<GameState>>) {
    if keys.just_pressed(KeyCode::R) {
        game_state.set(GameState::Gameplay).unwrap();
    }
}