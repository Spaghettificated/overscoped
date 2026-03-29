use bevy::prelude::*;

use crate::sprites::{SpriteBundle, Sprites};


#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Tower {
    Small,
    Big,
    Water,
    Earth,
    Fire,
    Air,
    Wizard,
    French,
}

#[derive(Bundle)]
pub struct TowerBundle {
    tower: Tower,
    transform: Transform,
    sprite_bundle: SpriteBundle,
}

impl TowerBundle {
    pub fn new(
        tower: Tower,
        transform: Transform, 
        sprites: Res<Sprites<Tower>>,
    ) -> Self {
        let sprite_bundle = sprites.get(&tower).expect("cannot access tower sprite").clone();
        Self { 
            tower, 
            transform, 
            sprite_bundle,
        }
    }
}


