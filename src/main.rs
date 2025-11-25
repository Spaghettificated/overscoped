use bevy::{color::palettes::basic::*, prelude::*};

use crate::{clicker::{clicker_plugin, TheNumber}, ui::{ui_plugin, ButtonChildrenDynamic, ButtonChildrenDynamicBundle, ButtonDynamic, ButtonDynamicBundle, ScreenUI}};

pub mod ui;
pub mod clicker;
pub mod number_resources;
pub mod cooldowns;
// pub mod square_lines;
// pub mod connectors;
// pub mod syncing;



fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui_plugin,
            clicker_plugin,
        ))
        .insert_resource(ClearColor(WHITE.into()))
        .add_systems(Startup, (ui::setup_ui, setup).chain())
        .run();
}



fn setup(
    mut commands: Commands, 
    number: Res<clicker::TheNumber>,
    screen: Single<Entity, With<ScreenUI>>
) {
    // ui camera
    commands.spawn(Camera2d);

    // commands.spawn((button(0), UiTransform::from_translation(Val2::px(100., 100.))));
    let the_button = commands.spawn((
        button(number.0 as usize), 
        ButtonDynamicBundle::new( ButtonDynamic { 
            default: UiTransform::IDENTITY, 
            hovered: UiTransform::IDENTITY, 
            pressed: UiTransform::from_translation(Val2::percent(0., 10.)) })
    )).id();
    commands.get_entity(*screen).unwrap()
        .add_children(&[the_button]);

}

// fn button(asset_server: &AssetServer) -> impl Bundle + use<>  {
fn button(i: usize) -> impl Bundle {
    (
        number_resources::NumberChanger::<clicker::TheNumber>::new(1),
        Button,
        Node {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            position_type: PositionType::Absolute,
            ..default()
        },
        // BorderColor::all(Color::BLACK),
        BorderRadius::all(Val::Percent(15.)),
        // BackgroundColor(NORMAL_BUTTON),
        ButtonDynamicBundle::new(ButtonDynamic {
             default: BorderColor::all(BLACK), 
             hovered: BorderColor::all(Color::srgb(0.31, 0.31, 0.31) ), 
             pressed: BorderColor::all(Color::srgb(0.31, 0.31, 0.31) ), 
        }),
        ButtonDynamicBundle::new(ButtonDynamic {
             default: BackgroundColor(WHITE.into()), 
             hovered: BackgroundColor(WHITE.into()), 
             pressed: BackgroundColor(WHITE.into()) 
        }),
        TextColor(Color::srgb(0.91, 0.61, 0.04).into()),
        children![(
            number_resources::TextDisplay::<TheNumber>::new(),
            Text::new(i.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            // TextColor(BLACK.into()),
            ButtonChildrenDynamicBundle::new(ButtonChildrenDynamic {
                default: TextColor(BLACK.into()), 
                hovered: TextColor(Color::srgb(0.31, 0.31, 0.31)), 
                pressed: TextColor(Color::srgb(0.31, 0.31, 0.31)),
            }),
            TextShadow::default(),
        )]
    )
}
