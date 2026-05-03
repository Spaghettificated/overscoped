

use bevy::{input::{ButtonState, keyboard::{Key, KeyboardInput}}, prelude::*, sprite::Anchor};
use crate::{sprites::{SpriteBundle, SpriteColorTint, SpriteScale, Sprites}, tower_defence::towers::{Tower, TowerBundle}, utils::MouseQuery};

const GHOST_COLOR: Color = Color::linear_rgba(0.36, 0.81, 0.88, 0.5);

#[derive(Component, Deref, DerefMut)]
pub struct TowerPlacer(pub Option<Tower>);
#[derive(Component)]
pub struct TowerGhost;

pub fn spawn_placer(
    mut commands: Commands,
){
    commands.spawn(TowerPlacer(None));
    commands.spawn((
            TowerGhost,
            SpriteBundle::default(),
            Transform::default(),
            SpriteColorTint(GHOST_COLOR),
            Visibility::Hidden,
        ));
}

// #[derive(Event)]
// pub struct SelectTower(pub Tower);

// pub fn select_towers(
//     mut keyboard: MessageReader<KeyboardInput>,
// )



pub fn place_towers(
    mut commands: Commands,
    mut keyboard: MessageReader<KeyboardInput>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    placer: Single<(Entity, &mut TowerPlacer)>,
    ghost: Single<(Entity, &mut Transform, &mut Sprite, &mut Anchor, &mut SpriteScale, &mut SpriteColorTint, &mut Visibility), With<TowerGhost>>,
    mouse: MouseQuery,
    sprites: Res<Sprites<Tower>>,
){
    let mouse = mouse.position();

    let (_placer, mut chosen_tower) = placer.into_inner();
    let (ghost, mut ghost_transform, ..) = ghost.into_inner();
    
    let new_tower = keyboard.read()
        .filter(|ev| {ev.state == ButtonState::Released })
        .find_map(|ev| {
            match &ev.logical_key{
                Key::Character(s) => {
                    s.parse::<usize>().ok().and_then(|n|{
                        [
                            Tower::Small,
                            Tower::Big,
                            Tower::Fire,
                            Tower::Water,
                            Tower::Air,
                            Tower::Earth,
                        ].get(n.checked_sub(1).unwrap_or(10))
                    }).cloned()
                },
                _ => { None }
            }
        });

    if let Some(new_tower) = new_tower{
        chosen_tower.0 = if chosen_tower.0 != Some(new_tower) {Some(new_tower)} else {None};
        
        let sprite_bundle = chosen_tower.0.and_then(|tower| { sprites.get(&tower).cloned() });
        
        if let Some(SpriteBundle {sprite, anchor, scale }) = sprite_bundle{
            commands.entity(ghost).insert((
                sprite,
                anchor,
                scale,
                Visibility::Visible,
            ));
        } else {        
            commands.entity(ghost).insert(Visibility::Hidden);
        }
    }

    

    if let Some(mouse) = mouse{
        ghost_transform.translation = mouse.extend(0.);
    }

    if let Some(mouse) = mouse {
        if mouse_buttons.just_pressed(MouseButton::Left){
            if let Some(tower) = chosen_tower.0{
                commands.spawn(TowerBundle::new(
                    tower,
                    Transform::from_translation(mouse.extend(0.)),
                ));
                chosen_tower.0 = None;
                commands.entity(ghost).insert(Visibility::Hidden);
            }
        }
    }

}