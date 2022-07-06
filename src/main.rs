// mod bird;

use bevy::prelude::*;
use flappy_bird::*;

fn main() {
    let window_descriptor = WindowDescriptor {
        title: "Flappy Bird".to_string(),
        height: WINDOW_HEIGHT,
        width: WINDOW_WIDTH,
        ..Default::default()
    };

    App::new()
        .insert_resource(ClearColor(Color::rgb(90. / 255., 140. / 255., 160. / 255.)))
        .insert_resource(window_descriptor)
        .insert_resource(GameState::Play)
        .add_plugins(DefaultPlugins)
        .add_plugin(FlappyPlugin)
        .run();
}
