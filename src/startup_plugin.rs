use bevy_kira_audio::{prelude::*, Audio};
use bevy::prelude::*;
use bevy_rapier2d::{prelude::{RapierConfiguration, Velocity, RigidBody}};
use crate::{GameState, player::{rapier_player_movement, Player}, GRAVITY_CONSTANT, moving_block::MovableWall};

#[derive(Resource)]
pub struct GameTextures {
    pub player: Handle<Image>,
    pub r_to_respawn: Handle<Image>,
    pub you_win: Handle<Image>,
    pub menu: Handle<Image>,
    pub exit: Handle<Image>,
    pub play: Handle<Image>,
    pub hook: Handle<Image>,
}

#[derive(Component)]
pub struct PlayerCamera;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            .add_startup_system_to_stage(StartupStage::PreStartup, pre_startup)
            .add_system_set(
                SystemSet::on_update(GameState::Gameplay)
                    .with_system(camera_follow_player.after(rapier_player_movement))
                    .with_system(spinny_cube)
            )
            .add_system_set(
                SystemSet::on_exit(GameState::Gameplay)
                    .with_system(despawn_everything)
            )        
            .add_system_set(
                SystemSet::on_enter(GameState::Gameplay)
                    .with_system(setup)
            )
            .add_system(toggle_mute);
    }
}

fn pre_startup(
    mut commands: Commands, 
    asset_server: Res<AssetServer>, 
    audio: Res<Audio>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    commands.insert_resource(GameTextures {
        player: asset_server.load("images/fella.png"),
        r_to_respawn: asset_server.load("death-messages/respawn.png"),
        you_win: asset_server.load("death-messages/you-win.png"),
        menu: asset_server.load("death-messages/menu.png"),
        exit: asset_server.load("death-messages/exit.png"),
        play: asset_server.load("death-messages/play.png"),
        hook: asset_server.load("images/hook.png")
    });

    let music = asset_server.load("music/new_bossa.wav");
    audio.play(music).looped().with_volume(0.2);
    audio.pause();

    rapier_config.gravity = GRAVITY_CONSTANT;
}

fn setup(mut commands: Commands) {
    commands.insert_resource(ClearColor(Color::rgb(1.0, 0.5, 0.0)));
    commands
        .spawn(Camera2dBundle::default())
        .insert(PlayerCamera)
        .insert(Velocity {
            linvel: Vec2::ZERO,
            ..Default::default()
        })
        .insert(RigidBody::Dynamic);
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
    mut camera: Query<(&Transform, &mut Velocity), (With<PlayerCamera>, Without<Player>)>,
    player: Query<&Transform, With<Player>>,
) {
    let (camera, mut vel) = camera.single_mut();
    let player = player.single();

    let velocity = (player.translation - camera.translation).truncate() * 2.0;
    vel.linvel = (velocity + vel.linvel) * 0.7;

}

pub fn despawn_everything(query: Query<Entity>, mut commands: Commands) {

    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}

fn spinny_cube (
    mut cubes: Query<(&Velocity, &mut Sprite), With<MovableWall>>,
) {

    for (vel, mut cube) in cubes.iter_mut() {
        if vel.angvel.abs() > 1.0 {

            let x = vel.angvel / vel.angvel.abs();
            cube.color += Color::rgba(0.02 *  x, -0.02 * x, 0.01 * x, 0.0);

        }
    }
}
