use bevy::prelude::*;

#[derive(Default, Resource)]
pub struct Store {
    pub font: Handle<Font>,
    pub sprite: Handle<Image>,
    pub layout: Handle<TextureAtlasLayout>,
    animation_indices: AnimationIndices,
}

#[derive(Component)]
struct AnimationIndices {
    first: usize,
    last: usize,
}

impl Default for AnimationIndices {
    fn default() -> Self {
        AnimationIndices { first: 1, last: 2 }
    }
}

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);
