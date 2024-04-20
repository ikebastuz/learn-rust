use bevy::prelude::*;

mod enemy;
mod game;
mod hero;
mod projectiles;
mod walls;
use enemy::move_enemies;
use game::start;
use hero::{level_up, move_hero};
use projectiles::{check_for_collisions, move_projectiles, shoot_enemy, shoot_hero};
use walls::spawn_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_walls(&mut commands);

    start(commands);
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
        .add_systems(
            FixedUpdate,
            (
                move_hero,
                move_enemies,
                shoot_hero,
                shoot_enemy,
                move_projectiles,
                check_for_collisions,
                level_up,
            )
                .chain(),
        )
        .run();
}
