// #![allow(dead_code)]

use crate::*;

use bevy::{
    math::{bounding::{Aabb2d, IntersectsVolume}, vec2},
    prelude::*,
};

#[derive(Component)]
pub struct Bird {
    pub is_jump: bool,
    pub velocity: f32,
    pub is_ded: bool,
}

pub fn bird_spawn(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let texture: Handle<Image> = asset_server.load("bird.png");
    let layout = TextureAtlasLayout::from_grid(Vec2::new(8., 8.), 3, 1, None, None);
    let texture_atlas_layout = texture_atlas_layouts.add(layout);
    // let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(8., 8.), 3, 1);
    // let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands
        .spawn(SpriteSheetBundle {
            texture,
            atlas: TextureAtlas {
                layout: texture_atlas_layout,
                index: 0,
            },
            transform: Transform {
                scale: Vec3::new(8., 8., 0.),
                translation: Vec3::new(-200., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Bird {
            is_jump: false,
            velocity: 0.,
            is_ded: false,
        });
}

pub fn bird_update(
    time: Res<Time>,
    state: ResMut<GameState>,
    mut query: Query<(&mut Transform, &mut TextureAtlas, &mut Bird)>,
) {
    if *state == GameState::Pause {
        return;
    }

    for (mut transform, mut sprite, mut bird) in query.iter_mut() {
        // Jump the bird if key pressed
        if bird.is_jump {
            bird.velocity = 520.;
            bird.is_jump = false;
        }

        // Gravity (Falling down part)
        transform.translation.y += bird.velocity * time.delta_seconds();

        // Animation
        match bird.velocity {
            x if x < -90. => sprite.index = 2,
            x if x < 80. => sprite.index = 1,
            _ => sprite.index = 0,
        }

        // Gravity (Increase velocity part)
        bird.velocity -= 40.;

        // Terminal velocity
        if bird.velocity < -1000. {
            bird.velocity = -1000.
        }

        // Upper bound
        if transform.translation.y + 24. > (WINDOW_HEIGHT as f32 / 2.) {
            transform.translation.y = (WINDOW_HEIGHT as f32 / 2.) - 24.;
        }

        // Lower bound
        if transform.translation.y - 24. < -(WINDOW_HEIGHT as f32 / 2.) {
            transform.translation.y = -(WINDOW_HEIGHT as f32 / 2.) + 24.;
        }
    }
}

pub fn ded_detect(
    bird_transform: Query<&Transform, With<Bird>>,
    pipe: Query<&Transform, With<Pipe>>,
    mut bird: Query<&mut Bird>,
    mut timer: ResMut<PipeSpawnConfig>,
) {
    let bird_transform = bird_transform.single();
    for pipe_transform in pipe.iter() {
        // Check for Bird collision with Pipe
        let berd_bounding = Aabb2d::new(
            vec2(bird_transform.translation.x, bird_transform.translation.y),
            vec2(64. / 2., 64. / 2.),
        );
        let pipe_bounding = Aabb2d::new(
            vec2(pipe_transform.translation.x, pipe_transform.translation.y),
            vec2(12. * 8. / 2., 496. / 2.),
        );

        let res = berd_bounding.intersects(&pipe_bounding);

        if res {
            // Aqcuire Bird reference
            let mut bird = bird.single_mut();

            // Reset Pipe spawning timer
            (*timer).timer.reset();

            // Set bird state to Dead
            bird.is_ded = true;
        }
    }
}
