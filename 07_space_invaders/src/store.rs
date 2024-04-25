use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Store {
    pub font: Handle<Font>,
    pub sprite: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    pub animation_indices: AnimationIndices,
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
