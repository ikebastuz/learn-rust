use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Store {
    pub font: Handle<Font>,
    pub sprite: Handle<Image>,
    pub layout_enemy_1: Handle<TextureAtlasLayout>,
    pub layout_enemy_2: Handle<TextureAtlasLayout>,
    pub layout_enemy_3: Handle<TextureAtlasLayout>,
    pub layout_hero: Handle<TextureAtlasLayout>,
}

#[derive(Component, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

impl Default for AnimationIndices {
    fn default() -> Self {
        AnimationIndices { first: 0, last: 1 }
    }
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
