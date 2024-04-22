use bevy::prelude::*;

use crate::enemy::{spawn_row, Enemy, ENEMY_INITIAL_SPEED};
use crate::game::{start, stop, AllEntitiesQuery};
use crate::projectiles::{ENEMY_SHOOTING_INTERVAL, HERO_SHOOTING_INTERVAL};
use crate::stats::{print_final_score, ScoreText};
use crate::walls::{LEFT_WALL, RIGHT_WALL, WALL_THICKNESS};
use std::cmp::max;

const HERO_SPEED: f32 = 500.0;
pub const HERO_SIZE: Vec3 = Vec3::new(30.0, 30.0, 0.0);
const HERO_PADDING: f32 = 0.0;
pub const HERO_COLOR: Color = Color::rgb(0.3, 0.3, 0.7);
const LEVEL_UP_TIMER: f32 = 1.0;
pub const LEVEL_UP_SPEED_MULTIPLIER: f32 = 1.2;

pub struct HeroShootingTimer(pub Timer);
pub struct EnemyShootingTimer(pub Timer);
pub struct LevelupTimer(Timer);

#[derive(Component)]
pub struct Hero {
    pub hero_shooting_timer: HeroShootingTimer,
    pub enemy_shooting_timer: EnemyShootingTimer,
    pub levelup_timer: LevelupTimer,
}

#[derive(Component)]
pub struct HeroProjectile;

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
        Hero {
            hero_shooting_timer: HeroShootingTimer(Timer::from_seconds(
                HERO_SHOOTING_INTERVAL,
                TimerMode::Repeating,
            )),
            enemy_shooting_timer: EnemyShootingTimer(Timer::from_seconds(
                ENEMY_SHOOTING_INTERVAL,
                TimerMode::Repeating,
            )),
            levelup_timer: LevelupTimer(Timer::from_seconds(LEVEL_UP_TIMER, TimerMode::Repeating)),
        },
    ));
}

pub fn level_up(
    time: Res<Time>,
    mut commands: Commands,
    mut hero_query: Query<&mut Hero>,
    mut enemy_query: Query<(&mut Transform, &mut Enemy)>,
    mut score_query: Query<(&mut ScoreText, &mut Text)>,
    mut all_query: AllEntitiesQuery,
) {
    for mut hero in &mut hero_query.iter_mut() {
        hero.levelup_timer.0.tick(time.delta());
        if hero.levelup_timer.0.finished() {
            let mut speed = ENEMY_INITIAL_SPEED;
            let mut current_level = 0;
            for (mut transform, mut enemy) in &mut enemy_query.iter_mut() {
                enemy.level_up(&mut transform);
                speed = enemy.speed;
                current_level = max(current_level, enemy.row_index);
            }

            if current_level >= 10 {
                print_final_score(&mut score_query);
                stop(&mut commands, &mut all_query);
                start(&mut commands);
            } else {
                spawn_row(&mut commands, 0, speed);
            }
        }
    }
}
