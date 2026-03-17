use bevy::{color::palettes::basic::*, math::VectorSpace, prelude::*, sprite_render::Wireframe2dPlugin};
use bevy::window::PrimaryWindow;
use overscoped::{
    tower_defence::*, ui::{self, ui_plugin}, utils::as_rgb
};


#[derive(Component)]
struct Tortoise{
    progress: f32
}
impl Tortoise {
    fn new() -> Self { Self { progress: 0. } }
}

#[derive(Component)]
struct Hare{
    windup: f32,
    v: Vec2
}
impl Hare {
    fn new() -> Self { Self { windup: 0. , v: Vec2::ZERO} }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(ImagePlugin::default_nearest()),
            ui_plugin,
            Wireframe2dPlugin::default(),
        ))
        .insert_resource(ClearColor(Color::srgb(1.00, 1.00, 1.00).into()))
        .add_systems(Startup, (ui::setup_ui, setup).chain())
        .add_systems(Update, (move_tortoise, move_hare))
        .run();
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){
    commands.spawn(Camera2d);
    let color = Color::hsl(360. * 1 as f32 / 2 as f32, 0.95, 0.7);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 1000.0))),
        MeshMaterial2d(materials.add(as_rgb(0xEEC39A) ).into()),
        // MeshMaterial2d(materials.add(color)),
        Transform::from_xyz(0., -500., 0.0)
    ));
    commands.spawn((
        Tortoise::new(),
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::srgb(0.14, 0.53, 0.20) ).into()),
        Transform::from_xyz(0., -200., 0.0)
    ));
    commands.spawn((
        Hare::new(),
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::srgb(0.67, 0.62, 0.58) ).into()),
        Transform::from_xyz(0., -300., 0.0)
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

fn move_tortoise(
    tortoise: Single<(&mut Tortoise, &mut Transform)>,
    time: Res<Time>,
){
    let (mut tortoise, mut transform) = tortoise.into_inner();
    tortoise.progress += time.delta_secs() * 0.02;
    // tortoise.progress += time.delta_secs() * 0.1;

    transform.translation = Vec3 { x: f32::sin(tortoise.progress) * 550., y: -f32::cos(tortoise.progress) * 200., z: 0. }
}
fn move_hare(
    hare: Single<(&mut Hare, &mut Transform)>,
    time: Res<Time>,
    keys: Res<ButtonInput<KeyCode>>,
    mut gizmos: Gizmos,
    window: Single<&Window, With<PrimaryWindow>>,
    camera: Single<(&Camera, &GlobalTransform)>,
){
    let (mut hare, mut transform) = hare.into_inner();
    let (camera, camera_transform) = camera.into_inner();

    let pos = transform.translation.xy();
    let Some(mouse) = window.cursor_position()
        .and_then(|cursor| Some(camera.viewport_to_world(camera_transform, cursor)))
        .map(|ray| ray.unwrap().origin.truncate()) 
        else { return; };
    let mouse_dir = (mouse - pos).normalize();

    if keys.pressed(KeyCode::Space) {
        hare.windup = f32::min(hare.windup + time.delta_secs() * 1.5, 1.)
    }

    if keys.just_released(KeyCode::Space){
        hare.v = hare.v + mouse_dir * 150. * f32::powf(hare.windup,1.);
        hare.windup = 0.;
    }

    transform.translation += (hare.v * time.delta_secs()).extend(0.);
    hare.v *= 1. - time.delta_secs();

    gizmos.line_2d(pos, pos + mouse_dir * 75. * hare.windup, BLACK);

}