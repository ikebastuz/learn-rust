use bevy::prelude::*;

use crate::enemy::{spawn_initial_enemies, Enemy, EnemyProjectile};
use crate::hero::{spawn_hero, Hero, HeroProjectile};

pub type AllEntitiesQuery<'a, 'b> = Query<
    'a,
    'b,
    Entity,
    Or<(
        With<Hero>,
        With<Enemy>,
        With<HeroProjectile>,
        With<EnemyProjectile>,
    )>,
>;

pub fn start(commands: &mut Commands) {
    spawn_hero(commands);
    spawn_initial_enemies(commands);
}

pub fn stop(commands: &mut Commands, query: &mut AllEntitiesQuery) {
    let mut entity_counter: usize = 0;
    for entity in query.iter_mut() {
        commands.entity(entity).despawn();
        entity_counter += 1;
    }
    println!("Despawned {} entities", entity_counter);
}
