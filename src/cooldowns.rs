use bevy::prelude::*;

pub fn cooldown_plugin(app: &mut App) {
    app.add_systems(Update, progress_cooldowns);
}

#[derive(EntityEvent)]
pub struct CooldownEnded{
    entity: Entity
}

#[derive(Component)]
pub struct Cooldown{
    progress: f32,
    duration: f32,
}

impl Cooldown {
    pub fn new(duration: f32) -> Self {
        Self { progress: 0., duration }
    }
}
fn progress_cooldowns(
    mut commands: Commands,
    cooldowns: Query<(Entity, &mut Cooldown)>,
    time: Res<Time>,
){
    for (entity, mut cooldown) in cooldowns{
        cooldown.progress += time.delta_secs();
        let times_ended = cooldown.progress.div_euclid(cooldown.duration) as usize;
        // cooldown.progress %= cooldown.duration;
        cooldown.progress = cooldown.progress.rem_euclid(cooldown.duration);
        for _ in 0..times_ended {
            commands.trigger(CooldownEnded {entity} );
        }
    }
}