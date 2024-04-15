use bevy::prelude::*;

use crate::walls::{LEFT_WALL, RIGHT_WALL, TOP_WALL};

pub const ENEMY_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
pub const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
const ENEMY_ROW_GAP: f32 = 10.0;
const ENEMIES_PER_ROW: usize = 10;
const ENEMY_ROW_MOVE_SPACES: usize = 2;
const INITIAL_ENEMY_ROWS: usize = 3;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_row(commands: &mut Commands, row_index: usize) {
    let enemy_y = TOP_WALL - (row_index + 1) as f32 * (ENEMY_SIZE.y + ENEMY_ROW_GAP as f32);
    let enemy_x_gap = (RIGHT_WALL - LEFT_WALL) as f32
        / (ENEMIES_PER_ROW + ENEMY_ROW_MOVE_SPACES) as f32
        - ENEMY_SIZE.x;

    for x in 0..ENEMIES_PER_ROW {
        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(
                        LEFT_WALL + (x + 1) as f32 * (ENEMY_SIZE.x + enemy_x_gap),
                        enemy_y,
                        0.0,
                    ),
                    scale: ENEMY_SIZE,
                    ..default()
                },
                sprite: Sprite {
                    color: ENEMY_COLOR,
                    ..default()
                },
                ..default()
            },
            Enemy,
        ));
    }
}

pub fn spawn_initial_enemies(commands: &mut Commands) {
    for y in 0..INITIAL_ENEMY_ROWS {
        spawn_row(commands, y);
    }
}
