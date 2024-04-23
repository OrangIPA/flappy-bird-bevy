use bevy::prelude::*;
use flappy_bird::*;

fn main() {
    // let window_descriptor = WindowDescriptor {
    //     title: "Flappy Bird".to_string(),
    //     height: WINDOW_HEIGHT,
    //     width: WINDOW_WIDTH,
    //     ..Default::default()
    // };

    App::new()
        .insert_resource(ClearColor(Color::rgb(90. / 255., 140. / 255., 160. / 255.)))
        .insert_resource(GameState::Play)
        .add_plugins(
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Flappy Bird".into(),
                        name: Some("flappybird.app".into()),
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
        )
        .add_plugins(FlappyPlugin)
        .run();
}
