use std::array;

use bevy::{color::palettes::{basic::*, css::PINK}, ecs::spawn, prelude::*};

use crate::ui::{ui_plugin, ButtonDynamic, ButtonDynamicBundle, ButtonChildrenDynamic, ButtonChildrenDynamicBundle};

pub mod ui;

#[derive(Resource, Deref, DerefMut)]
struct TheNumber(u32);

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            ui_plugin,
        ))
        .insert_resource(ClearColor(WHITE.into()))
        .insert_resource(TheNumber(0))
        .add_systems(Startup, setup)
        .run();
}



fn setup(mut commands: Commands, number: Res<TheNumber>) {
    // ui camera
    commands.spawn(Camera2d);
    // commands.spawn(button(&assets));
    let days_container = Node {
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        align_items: AlignItems::Center,
        justify_content: JustifyContent::FlexStart,
        flex_wrap: FlexWrap::Wrap,  
        ..default()
    };

    // commands.spawn((button(0), UiTransform::from_translation(Val2::px(100., 100.))));
    let the_button = commands.spawn((
        button(0), 
        ButtonDynamicBundle::new( ButtonDynamic { 
            default: UiTransform::IDENTITY, 
            hovered: UiTransform::IDENTITY, 
            pressed: UiTransform::from_translation(Val2::percent(0., 10.)) })
    )).id();
    commands
        .spawn(Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column, 
                ..default()
            })
        .add_children(&[the_button]);

}

// fn button(asset_server: &AssetServer) -> impl Bundle + use<>  {
fn button(i: usize) -> impl Bundle {
    (
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
             hovered: BorderColor::all(BLACK), 
             pressed: BorderColor::all(BLACK) 
        }),
        ButtonDynamicBundle::new(ButtonDynamic {
             default: BackgroundColor(WHITE.into()), 
             hovered: BackgroundColor(WHITE.into()), 
             pressed: BackgroundColor(WHITE.into()) 
        }),
        TextColor(Color::srgb(0.91, 0.61, 0.04).into()),
        children![(
            Text::new(i.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            ButtonChildrenDynamicBundle::new(ButtonChildrenDynamic {
                default: TextColor(BLACK.into()), 
                hovered: TextColor(Color::srgb(0.07, 0.22, 0.69).into()), 
                pressed: TextColor(BLACK.into()),
            }),
            TextShadow::default(),
        )]
    )
}
