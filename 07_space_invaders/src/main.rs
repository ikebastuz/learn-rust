use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod enemy;
mod game;
mod hero;
mod projectiles;
mod stats;
mod walls;
use enemy::move_enemies;
use game::start;
use hero::{level_up, move_hero};
use projectiles::{check_for_collisions, move_projectiles, shoot_enemy, shoot_hero};
use stats::{setup_stats, update_stats};
use walls::spawn_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    spawn_walls(&mut commands);
    setup_stats(&mut commands, asset_server);

    start(&mut commands);
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Space ikevaders".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            FrameTimeDiagnosticsPlugin,
        ))
        .insert_resource(ClearColor(BACKGROUND_COLOR))
        .add_systems(Startup, setup)
        .add_systems(
            FixedUpdate,
            (
                update_stats,
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
