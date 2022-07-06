use std::time::Duration;

use bevy::prelude::*;
use rand::Rng;

use crate::GameState;

// use crate::bird::Bird;
// use rand::prelude::*;

#[derive(Component)]
pub struct Pipe;

pub struct PipeSpawnConfig {
    pub timer: Timer,
}

pub fn spawn_pipe_timer(mut commands: Commands) {
    commands.insert_resource(PipeSpawnConfig {
        timer: Timer::new(Duration::from_millis(2000), true),
    });
}

pub fn spawn_pipe(
    mut commands: Commands,
    mut pipe_spawn_config: ResMut<PipeSpawnConfig>,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    gamestate: Res<GameState>,
) {
    if *gamestate == GameState::Pause { return }
    let random = rand::thread_rng().gen_range(-200..=200);
    pipe_spawn_config.timer.tick(time.delta());
    if pipe_spawn_config.timer.finished() {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: Transform {
                    translation: Vec3::new(700., random as f32 - 352., 0.),
                    scale: Vec3::new(8., 8., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe);

        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("pipe.png"),
                transform: Transform {
                    translation: Vec3::new(700., random as f32 + 360., 0.),
                    scale: Vec3::new(8., -8., 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(Pipe);
    }
}

pub fn despawn_pipe(
    mut pipe_query: Query<(Entity, &Transform), With<Pipe>>,
    mut commands: Commands,
) {
    for (entity, transform) in pipe_query.iter_mut() {
        if transform.translation.x < -700. {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_pipe(
    mut query: Query<&mut Transform, With<Pipe>>,
    res: Res<GameState>,
    time: Res<Time>,
) {
    if *res == GameState::Pause { return }
    for mut transform in query.iter_mut() {
        transform.translation.x -= 150. * time.delta_seconds();
    }
}