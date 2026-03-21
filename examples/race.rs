use bevy::{color::palettes::basic::*, prelude::*, sprite_render::Wireframe2dPlugin};
use bevy::window::PrimaryWindow;
use overscoped::{
    ui::{self, ui_plugin}, utils::as_rgb
};


#[derive(Component)]
struct Flag(usize);
#[derive(Component)]
struct FlagIndicator(usize);

#[derive(Component)]
struct Racer{
    id: usize,
    lap: usize,
    flag: usize,
}
impl Racer {
    fn new(id: usize) -> Self {
        Self {id, lap: 0, flag: 0 }
    }
}

#[derive(Component)]struct Tortoise{
    speed: f32
}
impl Tortoise {
    fn new(speed: f32) -> Self { Self { speed } }
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
        .add_systems(Update, (move_tortoise, move_hare, raise_flag, finish_laps))
        .run();
}



fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>
){
    commands.spawn(Camera2d);

    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(50.0, 1000.0))),
        MeshMaterial2d(materials.add(as_rgb(0xEEC39A) ).into()),
        Transform::from_xyz(0., -500., 0.0)
    ));
    commands.spawn((
        Mesh2d(meshes.add(Rectangle::new(900.0, 7.0))),
        MeshMaterial2d(materials.add(Color::srgb(0.00, 0.00, 0.00) ).into()),
        Transform::from_xyz(0., 0., 0.0)
    ));
    commands.spawn((
        Tortoise::new(5.),
        Racer::new(0),
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::srgb(0.14, 0.53, 0.20) ).into()),
        Transform::from_xyz(0., -200., 0.0)
    ));
    commands.spawn((
        Hare::new(),
        Racer::new(1),
        Mesh2d(meshes.add(Circle::new(20.))),
        MeshMaterial2d(materials.add(Color::srgb(0.67, 0.62, 0.58) ).into()),
        Transform::from_xyz(0., -300., 0.0)
    ));
    for (flag_i, flag_transform) in [
        Transform::from_xyz(550., -250., 0.0),
        Transform::from_xyz(550., 250., 0.0),
        Transform::from_xyz(-550., 250., 0.0),
        Transform::from_xyz(-550., -250., 0.0),
    ].into_iter().enumerate(){
        commands.spawn((
            Visibility::Visible,
            flag_transform,
            Flag(flag_i),
            children![
                (
                    Mesh2d(meshes.add(Rectangle::new(5.0, 60.0))),
                    MeshMaterial2d(materials.add(Color::srgb(0.00, 0.00, 0.00) ).into()),
                    Transform::IDENTITY,
                ),
                (
                    Mesh2d(meshes.add(Triangle2d::new(10.*Vec2::Y, 30.*Vec2::X, -10.*Vec2::Y))),
                    MeshMaterial2d(materials.add(Color::srgb(0.84, 0.09, 0.09) ).into()),
                    Transform::from_xyz(2.5, 20., 0.0),
                ),
                (
                    Text2d(flag_i.to_string()),
                    TextFont { font_size: 10., ..default()},
                    Transform::from_xyz(7., 20., 0.0),
                    TextColor(WHITE.into())
                ),
                (
                    Mesh2d(meshes.add(Triangle2d::new(10.*Vec2::Y, -30.*Vec2::X, -10.*Vec2::Y))),
                    MeshMaterial2d(materials.add(Color::srgb(0.67, 0.62, 0.58)).into()),
                    Transform::from_xyz(-2.5, -25., 0.0).with_scale(0.5*Vec3::ONE),
                    FlagIndicator(1),
                ),
                (
                    Mesh2d(meshes.add(Triangle2d::new(10.*Vec2::Y, -30.*Vec2::X, -10.*Vec2::Y))),
                    MeshMaterial2d(materials.add(Color::srgb(0.14, 0.53, 0.20) ).into()),
                    Transform::from_xyz(-2.5, -15., 0.0).with_scale(0.5*Vec3::ONE),
                    FlagIndicator(0),
                ),
            ]
        ));
    }
    
}

fn move_tortoise(
    tortoise: Single<(&mut Tortoise, &Racer, &mut Transform), Without<Flag>>,
    flags: Query<(&Flag, &Transform)>,
    time: Res<Time>,
){
    let (mut tortoise, racer, mut transform) = tortoise.into_inner();
    let flag_i = (racer.flag) % 4;
    if let Some(flag_transform) = flags.iter().find_map(|(flag, transform)| {
        if flag.0 == flag_i {Some(transform)} else {None}}){
            let target = flag_transform.translation - 30.*Vec3::Y;
        let dir = (target-transform.translation).normalize();
        transform.translation += dir * time.delta_secs() * tortoise.speed;
    }

    // tortoise.progress += time.delta_secs() * 0.02;
    // // tortoise.progress += time.delta_secs() * 0.1;

    // transform.translation = Vec3 { x: f32::sin(tortoise.progress) * 550., y: -f32::cos(tortoise.progress) * 200., z: 0. }
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
    transform.translation += (hare.v * time.delta_secs()).extend(0.);
    hare.v *= 1. - time.delta_secs();

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


    gizmos.line_2d(pos, pos + mouse_dir * 75. * hare.windup, BLACK);

}
fn raise_flag(
    racers: Query<(&mut Racer, &Transform)>,
    flags: Query<(&Flag, &Transform, &Children)>,
    mut indicators: Query<(&FlagIndicator, &mut Transform), (Without<Racer>, Without<Flag>)>,
){
    for (mut racer, racer_transform) in racers{
        for (flag, flag_transform, children) in flags{
            if racer_transform.translation.distance_squared(flag_transform.translation - 30.*Vec3::Y) < 400. && racer.flag == flag.0 {
                racer.flag += 1;
                for child in children{
                    let Ok((indi, mut indi_transofrm))  = indicators.get_mut(*child) else {continue;};
                    
                    if indi.0 == racer.id{
                        indi_transofrm.translation += 40. * Vec3::Y;
                    }
                }
            }
        }
    }
}
fn finish_laps(
    racers: Query<(&mut Racer, &Transform)>,
    flags: Query<(&Flag, &Transform, &Children)>,
    mut indicators: Query<(&FlagIndicator, &mut Transform), (Without<Racer>, Without<Flag>)>,
){
    for (mut racer, racer_transform) in racers{
        if racer_transform.translation.y < 0. 
            && racer_transform.translation.x > 0.
            && racer_transform.translation.x < 50.
            && racer.flag==4 {
            racer.flag = 0;
            racer.lap += 1;
            for (flag, flag_transform, children) in flags{
                for child in children{
                    let Ok((indi, mut indi_transofrm))  = indicators.get_mut(*child) else {continue;};
                    
                    if indi.0 == racer.id{
                        indi_transofrm.translation -= 50. * Vec3::Y;
                    }
                }
            }
        }
    }
}