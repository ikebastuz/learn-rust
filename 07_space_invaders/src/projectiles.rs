use crate::{
    enemy::{Enemy, EnemyProjectile, ENEMY_SIZE},
    hero::{Hero, HeroProjectile, HERO_SIZE},
    walls::{BOTTOM_WALL, TOP_WALL, WALL_THICKNESS},
};
use bevy::prelude::*;
use rand::seq::SliceRandom;
use rand::thread_rng;

pub const PROJECTILE_SIZE: Vec3 = Vec3::new(5.0, 5.0, 0.0);
pub const HERO_SHOOTING_INTERVAL: f32 = 1.0;
pub const ENEMY_SHOOTING_INTERVAL: f32 = 1.0;
const PROJECTILE_SPEED: f32 = 100.0;

pub fn shoot_hero(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Hero)>,
    mut commands: Commands,
) {
    for (transform, mut hero) in &mut query.iter_mut() {
        hero.hero_shooting_timer.0.tick(time.delta());

        if hero.hero_shooting_timer.0.finished() {
            commands.spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: transform.translation,
                        scale: PROJECTILE_SIZE,
                        ..Default::default()
                    },
                    sprite: Sprite {
                        color: Color::rgb(0.0, 1.0, 0.0),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                HeroProjectile,
            ));
        }
    }
}

pub fn shoot_enemy(
    time: Res<Time>,
    mut hero_query: Query<&mut Hero>,
    mut commands: Commands,
    enemy_query: Query<(&mut Transform, &Enemy)>,
) {
    for mut hero in &mut hero_query.iter_mut() {
        hero.enemy_shooting_timer.0.tick(time.delta());

        if hero.enemy_shooting_timer.0.finished() {
            let indexes: Vec<usize> = enemy_query.iter().map(|(_, e)| e.row_index).collect();
            let max_index: usize = *indexes.iter().max().unwrap_or(&0);

            let bottom_row_enemies: Vec<(&Transform, &Enemy)> = enemy_query
                .iter()
                .filter(|(_, e)| e.row_index == max_index)
                .map(|(t, e)| (t, e))
                .collect();

            if let Some((transform, _)) = bottom_row_enemies.choose(&mut thread_rng()) {
                commands.spawn((
                    SpriteBundle {
                        transform: Transform {
                            translation: transform.translation,
                            scale: PROJECTILE_SIZE,
                            ..Default::default()
                        },
                        sprite: Sprite {
                            color: Color::rgb(1.0, 0.0, 0.0),
                            ..Default::default()
                        },
                        ..Default::default()
                    },
                    EnemyProjectile,
                ));
            }
        }
    }
}

pub fn move_projectiles(
    mut commands: Commands,
    mut hero_query: Query<(Entity, &mut Transform, &mut HeroProjectile), Without<EnemyProjectile>>,
    mut enemy_query: Query<(Entity, &mut Transform, &mut EnemyProjectile), Without<HeroProjectile>>,
    time: Res<Time>,
) {
    for (entity, mut transform, _projectile) in &mut hero_query.iter_mut() {
        transform.translation.y += PROJECTILE_SPEED * time.delta_seconds();

        if transform.translation.y >= TOP_WALL - WALL_THICKNESS / 2.0 {
            commands.entity(entity).despawn();
        }
    }
    for (entity, mut transform, _projectile) in &mut enemy_query.iter_mut() {
        transform.translation.y -= PROJECTILE_SPEED * time.delta_seconds();

        if transform.translation.y <= BOTTOM_WALL + WALL_THICKNESS / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn collides(object_vector: &Vec3, projectile_vector: &Vec3, object_size: Vec3) -> bool {
    let object_left = object_vector.x - object_size.x / 2.0;
    let object_right = object_vector.x + object_size.x / 2.0;
    let object_top = object_vector.y + object_size.y / 2.0;
    let object_bottom = object_vector.y - object_size.y / 2.0;

    let projectile_left = projectile_vector.x - PROJECTILE_SIZE.x / 2.0;
    let projectile_right = projectile_vector.x + PROJECTILE_SIZE.x / 2.0;
    let projectile_top = projectile_vector.y + PROJECTILE_SIZE.y / 2.0;
    let projectile_bottom = projectile_vector.y - PROJECTILE_SIZE.y / 2.0;

    return projectile_right >= object_left
        && projectile_left <= object_right
        && projectile_top >= object_bottom
        && projectile_bottom <= object_top;
}

pub fn check_for_collisions(
    mut commands: Commands,
    hero_projectile_query: Query<(Entity, &Transform), With<HeroProjectile>>,
    enemy_projectile_query: Query<(Entity, &Transform), With<EnemyProjectile>>,
    hero_query: Query<(Entity, &Transform), With<Hero>>,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
) {
    for (hero_projectile_entity, hero_projectile_transform) in hero_projectile_query.iter() {
        for (enemy_entity, enemy_transform) in enemy_query.iter() {
            if collides(
                &enemy_transform.translation,
                &hero_projectile_transform.translation,
                ENEMY_SIZE,
            ) {
                commands.entity(enemy_entity).despawn();
                commands.entity(hero_projectile_entity).despawn();
            }
        }
    }

    for (enemy_projectile_entity, enemy_projectile_transform) in enemy_projectile_query.iter() {
        for (hero_entity, hero_transform) in hero_query.iter() {
            if collides(
                &hero_transform.translation,
                &enemy_projectile_transform.translation,
                HERO_SIZE,
            ) {
                commands.entity(hero_entity).despawn();
                commands.entity(enemy_projectile_entity).despawn();
            }
        }
    }
}
