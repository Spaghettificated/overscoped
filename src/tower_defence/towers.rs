use bevy::{prelude::*, window::PrimaryWindow};

use crate::sprites::{SpriteBundle, Sprites};


#[derive(PartialEq, Eq, Hash)]
pub enum TowerType {
    Small,
    Big,
    Water,
    Earth,
    Fire,
    Air,
    Wizard,
    French,
}

#[derive(Component)]
pub struct Tower;

#[derive(Bundle)]
pub struct TowerBundle {
    tower: Tower,
    transform: Transform,
    sprite_bundle: SpriteBundle,
}

impl TowerBundle {
    pub fn new(
        tower: TowerType,
        transform: Transform, 
        sprites: Res<Sprites<TowerType>>,
    ) -> Self {
        Self { 
            tower: Tower, 
            transform, 
            sprite_bundle: sprites.get(&tower).expect("cannot access tower sprite").clone(),
        }
    }
}

pub fn spawn_towers(
    mut commands: Commands,
    buttons: Res<ButtonInput<MouseButton>>,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
    sprites: Res<Sprites<TowerType>>,
){
    let (camera, camera_transform) = camera.into_inner();
    if buttons.just_pressed(MouseButton::Left){
        if let Some(mouse) = window.cursor_position()
            .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
            .map(|ray| ray.unwrap().origin.truncate()) 
            {
            commands.spawn(TowerBundle::new(
                TowerType::Earth,
                Transform::from_translation(mouse.extend(0.)),
                sprites
            ));
        }
    }

}
