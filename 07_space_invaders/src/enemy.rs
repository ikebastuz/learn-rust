use bevy::prelude::*;
use std::collections::HashMap;

use crate::walls::{LEFT_WALL, RIGHT_WALL, TOP_WALL, WALL_THICKNESS};

pub const ENEMY_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
pub const ENEMY_COLOR: Color = Color::rgb(1.0, 0.0, 0.0);
pub const ENEMY_ROW_GAP: f32 = 20.0;
const ENEMIES_PER_ROW: usize = 10;
const ENEMY_ROW_MOVE_SPACES: usize = 2;
const INITIAL_ENEMY_ROWS: usize = 3;
const TEMP_DEBUG_OFFSET: f32 = 40.0;

#[derive(Component)]
pub struct Enemy {
    direction: f32,
    pub row_index: usize,
    pub speed: f32,
}

impl Enemy {
    fn build(row_index: usize) -> Self {
        Self {
            direction: 1.0,
            row_index,
            speed: 1.0,
        }
    }
}

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
                        LEFT_WALL
                            + (x + 1) as f32 * (ENEMY_SIZE.x + enemy_x_gap)
                            + (row_index as f32 * TEMP_DEBUG_OFFSET) as f32,
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
            Enemy::build(row_index),
        ));
    }
}

pub fn spawn_initial_enemies(commands: &mut Commands) {
    for y in 0..INITIAL_ENEMY_ROWS {
        spawn_row(commands, y);
    }
}

pub fn move_enemies(mut query: Query<(&mut Transform, &mut Enemy)>) {
    let mut row_positions: HashMap<usize, (f32, f32)> = HashMap::new();

    // Update enemy positions and track row positions
    for (mut transform, enemy) in &mut query.iter_mut() {
        transform.translation.x += enemy.direction * enemy.speed;

        let (rightmost, leftmost) = row_positions
            .entry(enemy.row_index)
            .or_insert((f32::NEG_INFINITY, f32::INFINITY));

        *rightmost = rightmost.max(transform.translation.x);
        *leftmost = leftmost.min(transform.translation.x);
    }

    // Update enemy directions based on row positions
    for (row_id, (rightmost, leftmost)) in &row_positions {
        let direction = if *rightmost >= RIGHT_WALL - WALL_THICKNESS / 2.0 - ENEMY_SIZE.x / 2.0 {
            -1.0
        } else if *leftmost <= LEFT_WALL + WALL_THICKNESS / 2.0 + ENEMY_SIZE.x / 2.0 {
            1.0
        } else {
            continue;
        };

        for (_transform, mut enemy) in &mut query
            .iter_mut()
            .filter(|(_, e)| e.row_index == *row_id)
            .map(|(t, e)| (t, e))
        {
            enemy.direction = direction;
        }
    }
}
