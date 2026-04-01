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
}

impl TowerBundle {
    pub fn new(
        tower: Tower,
        transform: Transform, 
    ) -> Self {
        Self { 
            tower, 
            transform, 
        }
    }
}


