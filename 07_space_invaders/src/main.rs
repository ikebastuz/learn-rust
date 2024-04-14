use bevy::prelude::*;

mod hero;
mod walls;
use hero::{move_hero, Hero, HERO_COLOR, HERO_SIZE};
use walls::{WallBundle, WallLocation};

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let hero_y = -280.0;

    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, hero_y, 0.0),
                scale: HERO_SIZE,
                ..default()
            },
            sprite: Sprite {
                color: HERO_COLOR,
                ..default()
            },
            ..default()
        },
        Hero,
    ));

    // Walls
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
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
        .add_systems(FixedUpdate, (move_hero).chain())
        .run();
}
