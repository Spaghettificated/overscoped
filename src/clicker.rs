use bevy::{color::palettes::css::{BLACK, WHITE}, prelude::*};

use crate::{cooldowns::CooldownEnded, number_resources::add_number_resource, ui::ButtonDynamicBundle};

pub fn clicker_plugin(app: &mut App) { // make separate plugin for each resource ?
    // app.insert_resource(TheNumber(10));
    // app.add_systems(Update, display_numbers::<TheNumber>.run_if(resource_changed::<TheNumber>));
    // app.add_systems(Update, change_number::<TheNumber>);
    app.add_plugins(add_number_resource::<TheNumber>);
    app.add_systems(Startup, (setup_buy_menu, setup_buy_item).chain());
    app.add_systems(Update, (buy_items, produce));
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TheNumber(pub u32);
impl ToString for  TheNumber {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}
impl From<u32> for TheNumber {
    fn from(value: u32) -> Self { Self(value) }
}

// pub enum BuildingType{
//     Tier1,
//     Tier2,
// }



#[derive(Component)]
struct BuyMenu;

#[derive(Component, Clone)]
struct BuyItem{
    pub name: String,
    pub amount: u32,
    pub price: u32,
}
#[derive(Component)]
struct AmountDisplay(String);
#[derive(Component)]
struct PriceDisplay(String);
#[derive(Component)]
struct ProductionBuilding{
    gain: u32,
}
impl ProductionBuilding {
    fn new(gain: u32) -> Self {
        Self { gain }
    }
}


#[derive(Component, Deref, DerefMut)]
struct Cooldown(pub Timer);

impl Cooldown {
    fn new(timer: Timer) -> Self {
        Self(timer)
    }
    fn repeating(duration: f32) -> Self{
        Self(Timer::from_seconds(duration, TimerMode::Repeating))
    }
}


fn setup_buy_menu(
    mut commands: Commands,
){
    commands.spawn((
        BuyMenu,
        Node{
            width: Val::Px(350.0),
            height: Val::Px(500.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Start,
            // position_type: PositionType::Absolute,
            margin: UiRect::all(Val::Px(10.)),
            ..default()
        },
        BorderColor::all(Color::BLACK),
        // UiTransform::from_translation(Val2::px(10., 10.)),
        children![],
    ));
}
fn setup_buy_item(
    mut commands: Commands,
    menu_node: Single<Entity, With<BuyMenu>>,
    // item: BuyItem,
){
    let item = BuyItem{
        name: "Basic Tower".to_string(),
        amount: 0,
        price: 10,
    };
    let button = commands.spawn((
        item.clone(),
        ProductionBuilding::new(1),
        Cooldown::repeating(3.),
        Button,
        Node {
            width: Val::Percent(100.),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            margin: UiRect::all(Val::Px(3.)),
            justify_content: JustifyContent::SpaceAround,
            align_items: AlignItems::Center,
            ..default()
        },
        ButtonDynamicBundle::from_values(
            BorderColor::all(BLACK), 
            BorderColor::all(Color::srgb(0.31, 0.31, 0.31) ), 
            BorderColor::all(Color::srgb(0.87, 0.56, 0.56) )
        ),
        BackgroundColor(WHITE.into()),
        children![(
            Text::new("Tower"),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(BLACK.into()),
        ),
        (
            AmountDisplay(item.name.clone()),
            Text::new(item.amount.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 33.0,
                ..default()
            },
            TextColor(BLACK.into()),
        ),
        (
            PriceDisplay(item.name),
            Text::new("buy: ".to_string() + &item.price.to_string()),
            TextFont {
                // font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                font_size: 16.0,
                ..default()
            },
            TextColor(Color::srgb(0.93, 0.55, 0.20).into()),
        ),
        ]
    )).id();
    commands.get_entity(*menu_node).unwrap().add_child(button);
}

fn buy_items(
    mut items: Query<(
            &Interaction,
            &mut BuyItem,
        ),
        (Changed<Interaction>, With<Button>)>,
    mut texts_to_update : ParamSet<(
        Query<(&mut Text, &AmountDisplay)>,
        Query<(&mut Text, &PriceDisplay)>,
    )>,
    // mut amounts: Query<(&mut Text, &AmountDisplay)>,
    // mut prices: Query<(&mut Text, &PriceDisplay)>,
    mut number: ResMut<TheNumber>,
){
    for (interaction, mut item) in items{
        if *interaction == Interaction::Pressed
        && item.price <= number.0 {
            number.0 -= item.price;
            item.amount += 1;
            item.price += 10;

            for (mut text, display) in texts_to_update.p0().iter_mut(){
                if display.0 == item.name{
                    text.0 = item.amount.to_string();
                }
            }
            for (mut text, display) in texts_to_update.p1().iter_mut(){
                if display.0 == item.name{
                    text.0 = "buy: ".to_string() + &item.price.to_string();
                }
            }
        }
    }
}

fn produce(
    time: Res<Time>,
    buildings: Query<(&BuyItem, &ProductionBuilding, &mut Cooldown)>,
    mut number: ResMut<TheNumber>,
){
    for (item, production, mut cooldown) in buildings{
        cooldown.tick(time.delta());
        if cooldown.just_finished(){
            number.0 += production.gain * item.amount;
        }
    }
}

