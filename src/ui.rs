use bevy::{ecs::component::Mutable, prelude::*};

pub fn ui_plugin(app: &mut App){
    // let mut styles = NodeStyleCollection::new();
    // styles.init(Node::default());
    app.add_systems(Update, (button_dynamics::<BorderColor>, button_dynamics::<BorderRadius>, button_dynamics::<BackgroundColor>, button_dynamics::<UiTransform>));
    app.add_systems(Update, (button_children_dynamics::<TextColor>, button_children_dynamics::<BorderRadius>, button_children_dynamics::<BackgroundColor>));
}

// #[derive(Resource, Deref, DerefMut, Debug)]
// struct NodeStyle(Node);

#[derive(Component, Debug)]
// #[require(Button)]
pub struct ButtonDynamic<T>{
    pub default: T,
    pub hovered: T,
    pub pressed: T,
}
impl<T> ButtonDynamic<T> {
    fn get(&self, button_state: Interaction) -> &T{
        match button_state{
            Interaction::Pressed => &self.pressed,
            Interaction::Hovered => &self.hovered,
            Interaction::None => &self.default,
        }
    }
    fn get_mut(&mut self, button_state: Interaction) -> &mut T{
        match button_state{
            Interaction::Pressed => &mut self.pressed,
            Interaction::Hovered => &mut self.hovered,
            Interaction::None    => &mut self.default,
        }
    }
}

#[derive(Component, Debug)]
#[require(Button)]
pub struct ButtonChildrenDynamic<T>{
    pub default: T,
    pub hovered: T,
    pub pressed: T,
}
impl<T> ButtonChildrenDynamic<T> {
    fn get(&self, button_state: Interaction) -> &T{
        match button_state{
            Interaction::Pressed => &self.pressed,
            Interaction::Hovered => &self.hovered,
            Interaction::None => &self.default,
        }
    }
    fn get_mut(&mut self, button_state: Interaction) -> &mut T{
        match button_state{
            Interaction::Pressed => &mut self.pressed,
            Interaction::Hovered => &mut self.hovered,
            Interaction::None    => &mut self.default,
        }
    }
}

#[derive(Bundle, Debug)]
pub struct ButtonDynamicBundle<T: Component>{
    property: T,
    property_values: ButtonDynamic<T>,
}
impl<T: Component + Clone> ButtonDynamicBundle<T> {
    pub fn new(property_values: ButtonDynamic<T>) -> Self {
        Self { property: property_values.default.clone(), property_values }
    }
    pub fn from_values(default: T, hovered: T, pressed: T) -> Self {
        Self::new(ButtonDynamic { default, hovered, pressed })
    }
}

#[derive(Bundle, Debug)]
pub struct ButtonChildrenDynamicBundle<T: Component>{
    property: T,
    property_values: ButtonChildrenDynamic<T>,
}
impl<T: Component + Clone> ButtonChildrenDynamicBundle<T> {
    pub fn new(property_values: ButtonChildrenDynamic<T>) -> Self {
        Self { property: property_values.default.clone(), property_values }
    }
    pub fn from_values(default: T, hovered: T, pressed: T) -> Self {
        Self::new(ButtonChildrenDynamic { default, hovered, pressed })
    }
}


// #[derive(Component, Debug, Clone, Default)]
// struct ButtonStyle{
//     default: NodeStyle,
//     hovered: NodeStyle,
//     pressed: NodeStyle,
// }


// #[derive(Resource, Debug, Clone, Default, PartialEq, Eq, Deref, DerefMut)]
// struct NodeStyle(usize);

// #[derive(Resource, Debug, Clone)]
// struct NodeStyleCollection(pub Vec<Node>);

// impl NodeStyleCollection {
//     fn new() -> Self {
//         Self(Vec::new())
//     }
//     fn init(&mut self, style: Node) -> NodeStyle{
//         self.0.push(style);
//         NodeStyle(self.0.len()-1)
//     }
//     fn remove(&mut self, id: NodeStyle){
//         self.0.remove(*id);
//     }
//     fn get(& self, id: NodeStyle) -> Option<&Node>{
//         self.0.get(*id)
//     }
//     fn get_mut(&mut self, id: NodeStyle) -> Option<&mut Node>{
//         self.0.get_mut(*id)
//     }
// }

#[derive(Component)]
pub struct ScreenUI;


pub fn setup_ui(
    mut commands: Commands,
){
    commands
        .spawn((
                ScreenUI,
            Node{
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::SpaceEvenly,
                flex_direction: FlexDirection::Column, 
                ..default()
            }));
}

fn button_dynamics<T>
 (
    mut interaction_query: Query<
        (
            &Interaction,
            &mut T,
            &ButtonDynamic<T>,
        ),
        (Changed<Interaction>, With<Button>),
    >
) 
where T: Component<Mutability = Mutable> + Clone
{

    for (interaction, mut property, dynamics) in &mut interaction_query {
        *property = dynamics.get(*interaction).clone();
    }
}

fn button_children_dynamics<T>
 (
    mut interaction_query: Query<
        (
            &Interaction,
            &mut Children,
        ),
        (Changed<Interaction>, With<Button>),
    >,
    mut text_query: Query<(
    &mut T,
    &ButtonChildrenDynamic<T>,
    )>
) 
where T: Component<Mutability = Mutable> + Clone
{

    for (interaction, children) in &mut interaction_query {
        // for child in children.{
            // }
        for child_id in &children{
            if let Ok((mut child_property,  dynamics)) = text_query.get_mut(*child_id){
                *child_property = dynamics.get(*interaction).clone();
            }

        }
    }
}
