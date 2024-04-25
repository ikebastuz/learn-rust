use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, prelude::*};

mod enemy;
mod game;
mod hero;
mod projectiles;
mod stats;
mod store;
mod walls;
use enemy::{animate_sprite, move_enemies};
use game::start;
use hero::{level_up, move_hero};
use projectiles::{check_for_collisions, move_projectiles, shoot_enemy, shoot_hero};
use stats::{setup_stats, update_stats};
use store::Store;
use walls::spawn_walls;

const BACKGROUND_COLOR: Color = Color::rgb(0.0, 0.0, 0.0);
const ASSET_TILE_SIZE: Vec2 = Vec2::new(16.0, 8.0);
const ASSET_TILE_PADDING: Option<Vec2> = Some(Vec2::new(2.0, 2.0));

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut store: ResMut<Store>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let layout_enemy_1 = TextureAtlasLayout::from_grid(
        ASSET_TILE_SIZE,
        1,
        2,
        ASSET_TILE_PADDING,
        Some(Vec2::new(1.0, 1.0)),
    );
    let layout_enemy_2 = TextureAtlasLayout::from_grid(
        ASSET_TILE_SIZE,
        1,
        2,
        ASSET_TILE_PADDING,
        Some(Vec2::new(19.0, 1.0)),
    );
    let layout_enemy_3 = TextureAtlasLayout::from_grid(
        ASSET_TILE_SIZE,
        1,
        2,
        ASSET_TILE_PADDING,
        Some(Vec2::new(37.0, 1.0)),
    );
    let layout_hero = TextureAtlasLayout::from_grid(
        ASSET_TILE_SIZE,
        1,
        1,
        ASSET_TILE_PADDING,
        Some(Vec2::new(1.0, 49.0)),
    );
    store.layout_enemy_1 = texture_atlas_layouts.add(layout_enemy_1);
    store.layout_enemy_2 = texture_atlas_layouts.add(layout_enemy_2);
    store.layout_enemy_3 = texture_atlas_layouts.add(layout_enemy_3);
    store.layout_hero = texture_atlas_layouts.add(layout_hero);
    store.font = asset_server.load("fonts/Roboto-Regular.ttf");
    store.sprite = asset_server.load("sprites.png");

    commands.spawn(Camera2dBundle::default());
    spawn_walls(&mut commands);
    setup_stats(&mut commands, &store.font);

    start(&mut commands, &store);
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
        .init_resource::<Store>()
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
                animate_sprite,
            )
                .chain(),
        )
        .run();
}
