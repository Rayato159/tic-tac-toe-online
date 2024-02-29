use bevy::prelude::*;

use super::marker::Marker;

pub const FPS: f32 = 10.;

#[derive(Component, Debug, Clone)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(&Marker, &mut AnimationTimer, &mut TextureAtlas)>,
) {
    for (markers, mut timer, mut atlas) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            atlas.index = if atlas.index == markers.animation_indices.last {
                markers.animation_indices.first
            } else {
                atlas.index + 1
            };
        }
    }
}
