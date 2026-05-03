use bevy::prelude::*;

use crate::{cooldowns::Cooldown, tower_defence::projectiles::ProjectileSpawner};


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

impl Tower{
    pub fn add_tower_components(
        &self, 
        mut commands: Commands,
        entity: Entity,
    ){
        match self{
            Tower::Small => {
                commands.entity(entity).insert((
                    ProjectileSpawner,
                    Cooldown::new(3.),
                ));
            }
            _ => todo!()
        }
    }
}

pub fn insert_tower_specific_components(
    added: On<Add, Tower>,
    commands: Commands,
    towers: Query<&Tower>,
){
    if let Ok(tower) = towers.get(added.entity){
        tower.add_tower_components(commands, added.entity);
    }
}