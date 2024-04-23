mod bird;
mod pipe;

use bevy::prelude::*;
use bird::*;
use pipe::*;

pub const WINDOW_HEIGHT: f32 = 700.;
pub const WINDOW_WIDTH: f32 = 1200.;

#[derive(PartialEq, Resource)]
pub enum GameState {
    Pause,
    Play,
}

pub struct FlappyPlugin;

impl Plugin for FlappyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (setup, bird_spawn, spawn_pipe_timer))
            .add_systems(
                Update,
                (
                    bird_update,
                    input,
                    spawn_pipe,
                    update_pipe,
                    despawn_pipe,
                    ded_detect,
                    ded,
                ),
            );
    }
}

pub fn setup(
    mut commands: Commands,
    // asset_server: Res<AssetServer>,
    // mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default());
    // bird_spawn(commands, asset_server, texture_atlases);
}

pub fn input(
    // keys: Res<Input<KeyCode>>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut state: ResMut<GameState>,
    mut query: Query<&mut Bird>,
) {
    for mut bird in query.iter_mut() {
        // Jump if space is pressed
        if keyboard_input.just_pressed(KeyCode::Space) {
            bird.is_jump = true;
        }

        // Emulate dead state when 'R' is pressed
        if keyboard_input.just_pressed(KeyCode::KeyR) {
            bird.is_ded = true;
        }

        // Pause mechanism
        if keyboard_input.just_pressed(KeyCode::Escape) {
            match *state {
                GameState::Pause => *state = GameState::Play,
                GameState::Play => *state = GameState::Pause,
            };
        }
    }
}

pub fn ded(
    mut commands: Commands,
    mut bird_query: Query<(&mut Transform, &mut Bird)>,
    mut pipe_query: Query<Entity, With<Pipe>>,
) {
    let (mut transform, mut bird) = bird_query.single_mut();
    let is_ded = bird.is_ded;
    if is_ded {
        transform.translation.y = 0.;
        for pipe in pipe_query.iter_mut() {
            commands.entity(pipe).despawn();
        }
        bird.is_ded = false;
        bird.velocity = 0.;
        transform.translation.y = 0.;
    }
}
