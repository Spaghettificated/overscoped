use bevy::{color::palettes::basic::*, prelude::*, sprite_render::Wireframe2dPlugin};
use overscoped::{
    ui::{self, ui_plugin},
    tower_defence::*,
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
            DefaultPlugins,
            ui_plugin,
            td_plugin,
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(ClearColor(WHITE.into()))
        .add_systems(Startup, (ui::setup_ui, setup_td, setup).chain())
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
){
    let color = Color::hsl(360. * 1 as f32 / 2 as f32, 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 1000.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.82, 0.64, 0.41)).into()),
        // MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., 500., 0.0)
    ));
}
