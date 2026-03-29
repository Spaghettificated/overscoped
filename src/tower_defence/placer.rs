
use std::clone;

use bevy::{input::{ButtonState, keyboard::{Key, KeyboardInput}}, prelude::*, sprite::Anchor, window::PrimaryWindow};
use crate::{sprites::{SpriteBundle, SpriteColorTint, SpriteScale, Sprites}, tower_defence::towers::{Tower, TowerBundle}};

const GHOST_COLOR: Color = Color::linear_rgba(0.36, 0.81, 0.88, 0.5);

#[derive(Component, Deref, DerefMut)]
pub struct TowerPlacer(pub Option<Tower>);
#[derive(Component)]
pub struct TowerGhost;

// #[derive(Bundle)]
// pub struct TowerGhostBundle {
//     tower: TowerGhost,
//     transform: Transform,
//     sprite_bundle: SpriteBundle,
// }

// impl TowerGhostBundle {
//     pub fn new(
//         tower: TowerGhost,
//         transform: Transform, 
//         sprites: Res<Sprites<Tower>>,
//     ) -> Self {
//         let sprite_bundle = sprites.get(&tower).expect("cannot access tower sprite").clone();
//         Self { 
//             tower, 
//             transform, 
//             sprite_bundle,
//         }
//     }
// }

pub fn spawn_placer(
    mut commands: Commands,
){
    commands.spawn((TowerPlacer(None)));
    // commands.spawn((TowerGhost));
}

pub fn place_towers(
    mut commands: Commands,
    mut keyboard: MessageReader<KeyboardInput>,
    mouse_buttons: Res<ButtonInput<MouseButton>>,
    placer: Single<(Entity, &mut TowerPlacer)>,
    ghost: Option<Single<(Entity, &mut Transform, &mut Sprite, &mut Anchor, &mut SpriteScale), With<TowerGhost>>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    sprites: Res<Sprites<Tower>>,
){
    let (camera, camera_transform) = camera.into_inner();
    let mouse = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate());

    let (placer, mut chosen_tower) = placer.into_inner();
    for ev in keyboard.read(){
        if ev.state == ButtonState::Released { continue; }
        match &ev.logical_key{
            Key::Character(s) => {
                if let Some(tower) = s.parse::<usize>().ok().and_then(|n|{
                    [
                        Tower::Small,
                        Tower::Big,
                        Tower::Fire,
                        Tower::Water,
                        Tower::Air,
                        Tower::Earth,
                    ].get(n.checked_sub(1).unwrap_or(10))
                }){
                    chosen_tower.0 = Some(*tower).filter(|t1| chosen_tower.is_none_or(|t0| t0!=*t1));
                }
            },
            _ => {}
        }
    }


    let sprite_bundle = chosen_tower.0.and_then(|tower| { sprites.get(&tower).cloned() });


    if let Some((ghost, mut ghost_transform, mut ghost_sprite, mut ghost_anchor, mut ghost_scale)) = ghost.map(|g| g.into_inner()){
        if let Some(mouse) = mouse{
            ghost_transform.translation = mouse.extend(0.);
        }
        
        if let Some(SpriteBundle {sprite, anchor, scale }) = sprite_bundle{
            *ghost_sprite = sprite;
            *ghost_anchor = anchor;
            *ghost_scale = scale;
        } else {
            commands.entity(ghost).despawn();
        }
    } else if let (Some(bundle), Some(mouse)) = (sprite_bundle, mouse) {
        commands.spawn((
            TowerGhost,
            bundle,
            Transform::from_translation(mouse.extend(0.)),
            SpriteColorTint(GHOST_COLOR),
        ));
    }

    if let Some(mouse) = mouse {
        if mouse_buttons.just_pressed(MouseButton::Left){
            if let Some(tower) = chosen_tower.0{
                commands.spawn(TowerBundle::new(
                    tower,
                    Transform::from_translation(mouse.extend(0.)),
                    sprites
                ));
                chosen_tower.0 = None;
            }
        }
    }

}