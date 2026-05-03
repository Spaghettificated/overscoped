
use avian2d::prelude::{Collider, CollisionEventsEnabled, CollisionStart, LinearVelocity, RigidBody};
use bevy::{math::NormedVectorSpace, prelude::*};
use ordered_float::OrderedFloat;
use crate::{cooldowns::CooldownEnded, tower_defence::enemies::{Enemy, Health}};

#[derive(Component)]
pub struct Projectile;

#[derive(Component)]
pub struct ProjectileSpawner;

pub fn spawn_projectiles(
    trigger: On<CooldownEnded>,
    mut commands: Commands, 
    enemies: Query<&Transform, With<Enemy>>,
    spawners: Query<(Entity, &Transform), With<ProjectileSpawner>>,
){
    for (_, transform) in spawners.iter().filter(|&(spawner,_)| spawner == trigger.event_target()){
        if let Some(to_target) = enemies.iter()
            .map(|target| {target.translation.xy() - transform.translation.xy()})
            .min_by_key(|d| OrderedFloat(d.norm_squared()) )
            {
            let speed = 100.;
            let mut trans = Transform::from_translation(transform.translation);
            trans.rotate_z(Vec2::X.angle_to(to_target));
            commands.spawn((
                Projectile,
                trans,
                RigidBody::Kinematic,
                LinearVelocity(to_target.normalize() * speed),
                Collider::rectangle(10., 4.),
                CollisionEventsEnabled
            ))
            .observe(on_enemy_hit);
        }
    }
}

pub fn on_enemy_hit(
    collision: On<CollisionStart>,
    mut commands: Commands,
    mut enemies: Query<&mut Health,With<Enemy>>
){
    let projectile = collision.collider1;
    let enemy = collision.collider2;
    if let Ok(mut enemy_health) = enemies.get_mut(enemy){
        **enemy_health -= 5;
        commands.entity(projectile).despawn();
    }

}

