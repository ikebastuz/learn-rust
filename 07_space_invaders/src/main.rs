use bevy::prelude::*;

mod enemy;
mod hero;
mod walls;
use enemy::{move_enemies, spawn_initial_enemies};
use hero::{move_hero, spawn_hero};
use walls::spawn_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    spawn_hero(&mut commands);
    spawn_walls(&mut commands);
    spawn_initial_enemies(&mut commands);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Space ikevaders".to_string(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (move_hero, move_enemies).chain())
        .run();
}
