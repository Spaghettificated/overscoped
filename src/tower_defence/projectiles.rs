use std::f32::consts::PI;

use avian2d::{parry::na::Rotation, prelude::{AngularVelocity, Collider, LinearVelocity, RigidBody}};
use bevy::{math::NormedVectorSpace, prelude::*};
use ordered_float::{NotNan, OrderedFloat};
use crate::{cooldowns::{Cooldown, CooldownEnded}, sprites::Sprites, tower_defence::{enemies::{self, Enemy}, towers::Tower}};

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
            ));
        }
    }
}

