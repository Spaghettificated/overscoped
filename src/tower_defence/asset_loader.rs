
use bevy::{prelude::*, sprite::Anchor};

use crate::{sprites::{SpriteBundle, SpriteInfo, SpriteScale, Sprites}, tower_defence::towers::TowerType};




pub fn load_td_sprites(
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut sprites: ResMut<Sprites<TowerType>>,
){
    let texture = asset_server.load("low pixel/tower_defence.png");
    let mut layout = TextureAtlasLayout::from_grid(UVec2::splat(16), 8, 3, None, None);
    layout.add_texture(layout.textures[5].union(layout.textures[5+8]));
    let layout = texture_atlas_layouts.add(layout);

    for (tower, sprite_info) in [
        (TowerType::Small, SpriteInfo::new(0, Anchor::BOTTOM_CENTER,  SpriteScale(4.))),
        (TowerType::Big,   SpriteInfo::new(8, Anchor::BOTTOM_CENTER,  SpriteScale(4.))),
        (TowerType::Fire,  SpriteInfo::new(9, Anchor::BOTTOM_CENTER,  SpriteScale(4.))),
        (TowerType::Water, SpriteInfo::new(10, Anchor::BOTTOM_CENTER, SpriteScale(4.))),
        (TowerType::Air,   SpriteInfo::new(11, Anchor::BOTTOM_CENTER, SpriteScale(4.))),
        (TowerType::Earth, SpriteInfo::new(12, Anchor::BOTTOM_CENTER, SpriteScale(4.))),
    ]{
        sprites.insert(tower, SpriteBundle::from_sprite_info(sprite_info, texture.clone(), layout.clone()));
    }
    

}