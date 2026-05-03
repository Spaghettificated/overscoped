use avian2d::prelude::{Collider, CollisionEventsEnabled, Sensor};
use bevy::prelude::*;

use crate::{cooldowns::{Cooldown, CooldownEnded}, tower_defence::placer::TowerPlacer, utils::MouseQuery};


#[derive(Component, Deref, DerefMut)]
pub struct Health(i32);

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct EnemySpawner;

#[derive(Bundle)]
pub struct EnemyBundle {
    enemy: Enemy,
    transform: Transform,
    health: Health,
    sprite: Sprite,
    collision_enabled: CollisionEventsEnabled,
}

impl EnemyBundle {
    pub fn new(transform: Transform, asset_server: &Res<AssetServer>) -> Self {
        Self { 
            enemy: Enemy, 
            transform, 
            health: Health(10),
            sprite: Sprite{
                image: asset_server.load("low pixel/enemy.png"),
                custom_size: Some(Vec2::new(64., 64.)),
                ..default()
            },
            collision_enabled: CollisionEventsEnabled,
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
    spawner: Single<(Entity, &Transform), With<EnemySpawner>>,
    asset_server: Res<AssetServer>,
){
    let (spawner, transform) = spawner.into_inner();
    if trigger.event_target() != spawner { return; }
    commands.spawn((EnemyBundle::new(
        transform.clone(), 
        &asset_server,
    ),
        Collider::circle(20.),
        Sensor,
    ));
}

pub fn kill_enemies(
    mut commands: Commands, 
    enemies: Query<(Entity, &Health), With<Enemy>>,
){
    for (enemy, health) in enemies{
        if health.0 <= 0{
            commands.entity(enemy).despawn();
        }
    }
}

pub fn attack_on_click(
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    placer: Single<&TowerPlacer>,
    enemies: Query<(&Transform, &mut Health, &Collider), With<Enemy>>,
    mouse: MouseQuery,
){
    if placer.0 != None {return;}
    let mouse = mouse.position();

    if let Some(mouse) = mouse {
        if mouse_buttons.just_pressed(MouseButton::Left){
            for (transform, mut health, collider) in enemies {
                if collider.contains_point(transform.translation.xy(), transform.rotation, mouse){
                    health.0 -= 10;
                }
            }
        }
    }
}