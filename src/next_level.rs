use bevy::prelude::*;

use crate::GameState;

#[derive(Resource)]
pub struct LevelTimer {
    timer: Timer,
}

pub struct NextLevelPlugin;

impl Plugin for NextLevelPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(
                SystemSet::on_enter(GameState::NextLevel)
                    .with_system(next_level_system)
            )
            .add_system_set(
                SystemSet::on_update(GameState::NextLevel)
                    .with_system(back_to_gameplay)
            );
    }
}

fn next_level_system (
    mut commands: Commands,
) {
    commands.insert_resource(LevelTimer{timer: Timer::from_seconds(0.5, TimerMode::Once)});
    commands.spawn(Camera2dBundle::default());
    commands.insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
}

fn back_to_gameplay (
    mut game_state: ResMut<State<GameState>>,
    mut timer: ResMut<LevelTimer>,
    time: Res<Time>,
    entities: Query<Entity>,
    mut commands: Commands,
) {

    timer.timer.tick(time.delta());
    let percent = timer.timer.percent_left();
    commands.insert_resource(ClearColor(Color::rgb(7.0, percent, percent)));

    if timer.timer.finished() {
        for entity in entities.iter() {
            commands.entity(entity).despawn()
        }
        match game_state.set(GameState::Gameplay) {
            Ok(a) => a,
            Err(a) => println!("{a}, (NextLevel to Gameplay)"),
        }
    }

}