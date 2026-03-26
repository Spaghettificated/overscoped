use bevy::prelude::*;

use crate::cooldowns::{Cooldown, CooldownEnded};

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpawner;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    transform: Transform,
    sprite: Sprite,
}

impl EnemyBundle {
    pub fn new(transform: Transform, asset_server: &Res<AssetServer>) -> Self {
        Self { 
            enemy: Enemy, 
            transform, 
            sprite: Sprite{
                image: asset_server.load("low pixel/enemy.png"),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
        }
    }
}

pub fn move_enemies(
    enemies: Query<&mut Transform, With<Enemy>>,
    time: Res<Time>,
){
    for mut transform in enemies{
        let target = (Vec3::ZERO - transform.translation).normalize();
        transform.translation += target * time.delta_secs() * 20.;
    }
}

#[derive(Bundle)]
pub struct EnemySpawnerBundle {
    spawner: EnemySpawner,
    transform: Transform,
    cooldown: Cooldown,
}

impl EnemySpawnerBundle {
    pub fn new(transform: Transform, interval: f32) -> Self {
        Self { spawner: EnemySpawner, transform, cooldown: Cooldown::new(interval) }
    }
}

pub fn spawn_enemies(
    trigger: On<CooldownEnded>,
    mut commands: Commands, 
    spawner: Single<(Entity, &Transform, &Cooldown), With<EnemySpawner>>,
    asset_server: Res<AssetServer>,
){
    let (spawner, transform, cooldown) = spawner.into_inner();
    commands.spawn(EnemyBundle::new(
        transform.clone(), 
        &asset_server,
    ));
}