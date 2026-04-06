use bevy::{prelude::*, sprite_render::Wireframe2dPlugin};
use overscoped::{
    cooldowns::Cooldown, sprites::Sprites, tower_defence::{enemies::{EnemyBundle, EnemySpawnerBundle}, projectiles::ProjectileSpawner, towers::{Tower, TowerBundle}, *}, ui::{self, ui_plugin}, utils::as_rgb
};

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
    asset_server: Res<AssetServer>,
    sprites: Res<Sprites<Tower>>,
){
    let color = Color::hsl(360. * 1 as f32 / 2 as f32, 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(40.0, 1000.0))),
        MeshMaterial2d(materials.add(as_rgb(0xEEC39A) ).into()),
        // MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., -500., 0.0)
    ));
    // commands.spawn((TowerBundle::new(
    //     Tower::Small,
    //     Transform::from_xyz(300., 300., 0.),
    // ), Visibility::Visible));
    commands.spawn(EnemySpawnerBundle::new(
        Transform::from_xyz(0., -340., 0.), 
        5.
    ));
    // commands.spawn((
    //     ProjectileSpawner,
    //     Cooldown::new(3.),
    //     Transform::from_xyz(100., 100., 0.)
    // ));
}
