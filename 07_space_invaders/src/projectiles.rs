use crate::{
    hero::{Hero, HeroProjectile},
    walls::{TOP_WALL, WALL_THICKNESS},
};
use bevy::prelude::*;

pub const PROJECTILE_SIZE: Vec3 = Vec3::new(5.0, 5.0, 0.0);
pub const HERO_SHOOTING_INTERVAL: f32 = 1.0;
const PROJECTILE_SPEED: f32 = 100.0;

pub fn shoot_hero(
    time: Res<Time>,
    mut query: Query<(&Transform, &mut Hero)>,
    mut commands: Commands,
) {
    for (transform, mut hero) in &mut query.iter_mut() {
        hero.shooting_timer.0.tick(time.delta());

        if hero.shooting_timer.0.finished() {
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

pub fn move_projectiles(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Transform, &mut HeroProjectile)>,
    time: Res<Time>,
) {
    for (entity, mut transform, _projectile) in &mut query.iter_mut() {
        transform.translation.y += PROJECTILE_SPEED * time.delta_seconds();

        if transform.translation.y >= TOP_WALL - WALL_THICKNESS / 2.0 {
            commands.entity(entity).despawn();
        }
    }
}
