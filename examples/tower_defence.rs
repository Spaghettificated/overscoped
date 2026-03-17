use bevy::{color::palettes::basic::*, prelude::*, sprite_render::Wireframe2dPlugin};
use overscoped::{
    tower_defence::*, ui::{self, ui_plugin}, utils::as_rgb
};

// use crate::{clicker::{clicker_plugin, TheNumber}, ui::{ui_plugin, ButtonChildrenDynamic, ButtonChildrenDynamicBundle, ButtonDynamic, ButtonDynamicBundle, ScreenUI}};

// pub mod ui;
// pub mod clicker;
// pub mod number_resources;
// pub mod cooldowns;
// pub mod square_lines;
// pub mod connectors;
// pub mod syncing;



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ui_plugin,
            td_plugin,
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(1.00, 1.00, 1.00).into()))
        .add_systems(Startup, (ui::setup_ui, setup_td, setup).chain())
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){
    let color = Color::hsl(360. * 1 as f32 / 2 as f32, 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 1000.0))),
        MeshMaterial2d(materials.add(as_rgb(0xEEC39A) ).into()),
        // MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., -500., 0.0)
    ));
    commands.spawn((
        // Sprite::from_image(asset_server.load("low pixel/tower.png")),
        Sprite{
            image: asset_server.load("low pixel/tower.png"),
            custom_size: Some(Vec2::new(64., 64.)),
            // image_mode: rect.image_mode,
            
            ..default()
        },
        Transform::from_xyz(300., -200., 0.0),
    ));
}
