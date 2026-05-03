
use avian2d::{PhysicsPlugins, prelude::PhysicsDebugPlugin};
use bevy::{color::palettes::css::{BLACK, RED, WHITE}, prelude::*};

use crate::{
    cooldowns::cooldown_plugin, 
    number_resources::{self, add_number_resource}, 
    sprites::{Sprites, sprite_plugin, sprite_resource_plugin}, 
    tower_defence::{asset_loader::load_td_sprites, 
    placer::{place_towers, spawn_placer}, 
    projectiles::spawn_projectiles}, 
    ui::ScreenUI
};
use crate::tower_defence::{towers::*, enemies::*};

pub mod towers;
pub mod enemies;
pub mod asset_loader;
pub mod placer;
pub mod projectiles;

pub fn td_plugin(app: &mut App) { // make separate plugin for each resource ?
    // app.insert_resource(TheNumber(10));
    // app.add_systems(Update, display_numbers::<TheNumber>.run_if(resource_changed::<TheNumber>));
    // app.add_systems(Update, change_number::<TheNumber>);
    app.add_plugins(PhysicsPlugins::default());
    app.add_plugins(PhysicsDebugPlugin);
    app.add_plugins(add_number_resource::<Life>);
    app.add_plugins(cooldown_plugin);
    app.add_plugins((
        sprite_plugin,
        sprite_resource_plugin::<Tower>,
    ));
    // app.init_resource::<Sprites<Tower>>();
    app.add_systems(Startup, (load_td_sprites, spawn_placer));
    app.add_systems(Update, (place_towers, move_enemies, take_damage));
    app.add_systems(Update, (attack_on_click, kill_enemies));  // for some reason breaks the tower ghost
    // app.add_systems(Update, kill_enemies);  // for some reason breaks the tower ghost
    // app.add_systems(Update, attack_on_click);  // for some reason breaks the tower ghost
    app.add_observer(spawn_enemies);
    app.add_observer(spawn_projectiles);
    app.add_observer(insert_tower_specific_components);
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Life(pub u32);
impl ToString for  Life {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}
#[derive(Resource, Deref, DerefMut, Default)]
pub struct Round(pub u32);
impl ToString for  Round {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}


fn take_damage(
    mut commands: Commands, 
    enemies: Query<(Entity, &Transform), With<Enemy>>,
    mut life: ResMut<Life>,
){
    for (enemy, Transform { translation, .. }) in enemies{
        if translation.y > -30.{
            life.0 -= 1;
            commands.entity(enemy).despawn();
        }
    }
}

pub fn setup_td(
    mut commands: Commands, 
    mut life: ResMut<Life>,
    screen: Single<Entity, With<ScreenUI>>
) {
    // ui camera

    commands.spawn(Camera2d);

    **life = 10;

    let display = commands.spawn((
        number_resources::NumberChanger::<Life>::new(**life as i32),
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            border_radius: BorderRadius::all(Val::Percent(15.)),
            ..default()
        },
        BorderColor::all(BLACK), 
        BackgroundColor(WHITE.into()),
        TextColor(Color::srgb(0.91, 0.61, 0.04).into()),
        children![(
            number_resources::TextDisplay::<Life>::new(),
            Text::new(life.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(RED.into()), 
            // TextShadow::default(),
        )]
    )).id();
    commands.get_entity(*screen).unwrap()
        .add_children(&[display]);

}