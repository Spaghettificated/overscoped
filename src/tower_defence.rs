
use bevy::{color::palettes::css::{BLACK, RED, WHITE}, prelude::*};

use crate::{number_resources::{self, add_number_resource}, ui::{ButtonDynamicBundle, ScreenUI}};

pub fn td_plugin(app: &mut App) { // make separate plugin for each resource ?
    // app.insert_resource(TheNumber(10));
    // app.add_systems(Update, display_numbers::<TheNumber>.run_if(resource_changed::<TheNumber>));
    // app.add_systems(Update, change_number::<TheNumber>);
    app.add_plugins(add_number_resource::<Life>);
    // app.add_systems(Startup, (setup_buy_menu, setup_buy_item).chain());
    // app.add_systems(Update, (buy_items, produce));
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct Life(pub u32);
impl ToString for  Life {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}
#[derive(Resource, Deref, DerefMut, Default)]
pub struct Round(pub u32);
impl ToString for  Round {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}

pub fn setup_td(
    mut commands: Commands, 
    mut life: ResMut<Life>,
    screen: Single<Entity, With<ScreenUI>>
) {
    // ui camera

    commands.spawn(Camera2d);

    **life = 10;

    let display = commands.spawn((
        (
        number_resources::NumberChanger::<Life>::new(**life as i32),
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
        BorderRadius::all(Val::Percent(15.)),
        BorderColor::all(BLACK), 
        BackgroundColor(WHITE.into()),
        TextColor(Color::srgb(0.91, 0.61, 0.04).into()),
        children![(
            number_resources::TextDisplay::<Life>::new(),
            Text::new(life.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(RED.into()), 
            // TextShadow::default(),
        )]
    )
    )).id();
    commands.get_entity(*screen).unwrap()
        .add_children(&[display]);

}