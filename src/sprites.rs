use std::{collections::HashMap, hash::Hash};

use bevy::{prelude::*, sprite::Anchor};

pub fn sprite_plugin(app: &mut App) { 
    app.add_systems(PostUpdate, (scale_sprites, color_sprites));
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SpriteColorTint(pub Color);

pub fn color_sprites(
    sprites: Query<(&mut Sprite, &SpriteColorTint), Or<(Changed<SpriteColorTint>, Changed<Sprite>)>>,
){
    for (mut sprite, color) in sprites{
        sprite.color = color.0;
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SpriteScale(pub f32);
impl Default for SpriteScale { fn default() -> Self { SpriteScale(1.) } }

pub fn scale_sprites(
    sprites: Query<(&mut Sprite, &SpriteScale), Or<(Changed<SpriteScale>, Changed<Sprite>)>>,
    atlasess: Res<Assets<TextureAtlasLayout>>,
    images: Res<Assets<Image>>,
){
    for (mut sprite, scale) in sprites{
        let size = if let Some(atlas) = &sprite.texture_atlas{
            atlas.texture_rect(&atlasess).map(|r| r.size())
        } else { 
            images.get(&sprite.image).map(|im| im.size())
        };
        sprite.custom_size = size.map(|s| scale.0 * s.as_vec2());
    }
}


#[derive(Bundle, Clone, Default)]
pub struct SpriteBundle {
    pub sprite: Sprite,
    pub anchor: Anchor,
    pub scale: SpriteScale,
}
impl SpriteBundle {
    pub fn new(
        sprite: Sprite, 
        anchor: Anchor, 
        scale: SpriteScale
    ) -> Self {
        Self { sprite, anchor, scale }
    }
    pub fn from_sprite_info(
        sprite_info: SpriteInfo,
        texture: Handle<Image>,
        layout: Handle<TextureAtlasLayout>,
    ) -> Self {
        Self { 
            sprite: Sprite::from_atlas_image(
                texture,
                TextureAtlas {
                    layout: layout,
                    index: sprite_info.index,
                },
            ),
            anchor: sprite_info.anchor, 
            scale: sprite_info.scale }
    }
}

pub struct SpriteInfo{
    pub index: usize,
    pub anchor: Anchor,
    pub scale: SpriteScale,
}

impl SpriteInfo {
    pub fn new(index: usize, anchor: Anchor, scale: SpriteScale) -> Self {
        Self { index, anchor, scale }
    }
}

#[derive(Resource, Deref, DerefMut)]
pub struct Sprites<T: Hash>(HashMap<T, SpriteBundle>);
impl<T: Hash> Default for Sprites<T>{
    fn default() -> Self {
        Self(HashMap::new())
    }
} 

#[derive(Component)]
pub struct NoSpriteInsert;

pub fn insert_sprites<T: Component + Hash + Eq>(
    mut commands: Commands,
    entities: Query<(Entity, &T), (Added<T>, Without<NoSpriteInsert>)>,
    sprites: Res<Sprites<T>>,
){
    for (entity, component) in entities{
        commands.entity(entity).insert(
            sprites.get(component).expect("cannot access tower sprite").clone()
        );
    }
}

pub fn sprite_resource_plugin<T: Component + Hash + Eq>(app: &mut App) { 
    app.init_resource::<Sprites<T>>();
    app.add_systems(PostUpdate, insert_sprites::<T>.before( color_sprites).before(color_sprites));
}