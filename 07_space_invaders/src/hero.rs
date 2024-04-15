use bevy::prelude::*;

use crate::walls::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};

const HERO_SPEED: f32 = 500.0;
pub const HERO_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const HERO_PADDING: f32 = 0.0;
pub const HERO_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);

#[derive(Component)]
pub struct Hero;

pub fn move_hero(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Hero>>,
    time: Res<Time>,
) {
    let mut hero_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        direction += 1.0;
    }

    let new_hero_position =
        hero_transform.translation.x + direction * HERO_SPEED * time.delta_seconds();

    let left_bound = LEFT_WALL + WALL_THICKNESS / 2.0 + HERO_SIZE.x / 2.0 + HERO_PADDING;
    let right_bound = RIGHT_WALL - WALL_THICKNESS / 2.0 - HERO_SIZE.x / 2.0 - HERO_PADDING;

    hero_transform.translation.x = new_hero_position.clamp(left_bound, right_bound);
}

pub fn spawn_hero(commands: &mut Commands) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                translation: Vec3::new(0.0, -280.0, 0.0),
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
}
