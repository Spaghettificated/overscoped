use std::{marker::PhantomData, ops::DerefMut};

use bevy::prelude::*;


pub fn add_number_resource<T>(app: &mut App)
// where T: Resource + ToString + DerefMut<Target = u32> + From<u32>
where T: Resource + ToString + DerefMut<Target = u32> + Default
{ // make separate plugin for each resource ?
    app.init_resource::<T>();
    app.add_systems(Update, display_numbers::<T>.run_if(resource_changed::<T>));
    app.add_systems(Update, change_number::<T>);
}


#[derive(Component)]
pub struct TextDisplay<T>{
    _marker: PhantomData<T>
}
impl<T> TextDisplay<T> {
    pub fn new() -> Self { TextDisplay { _marker: PhantomData::<T> } }
}


#[derive(Component)]
pub struct NumberChanger<T>{
    pub amount: i32,
    _marker: PhantomData<T>,
}

impl<T> NumberChanger<T> {
    pub fn new(amount: i32) -> Self {
        Self { amount, _marker: PhantomData }
    }
}


fn display_numbers<T: Resource + ToString>(
    text_fields: Query<&mut Text, With<TextDisplay<T>>>,
    number: Res<T>,
){
    for mut txt in text_fields{
        txt.0 = number.to_string();
    }
}
fn change_number<T: Resource + DerefMut<Target = u32>>(  // change for Integer trait
    interactions: Query<
            (&Interaction, &NumberChanger<T>),
            (Changed<Interaction>, With<Button>),
        >,
    mut number: ResMut<T>,
){
    for (interaction, change) in interactions{
        if *interaction == Interaction::Pressed{
            **number = number.saturating_add_signed(change.amount);
        }
    }
}