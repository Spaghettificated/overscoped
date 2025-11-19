use bevy::prelude::*;

use crate::number_resources::add_number_resource;

pub fn clicker_plugin(app: &mut App) { // make separate plugin for each resource ?
    // app.insert_resource(TheNumber(10));
    // app.add_systems(Update, display_numbers::<TheNumber>.run_if(resource_changed::<TheNumber>));
    // app.add_systems(Update, change_number::<TheNumber>);
    app.add_plugins(add_number_resource::<TheNumber>);
}

#[derive(Resource, Deref, DerefMut, Default)]
pub struct TheNumber(pub u32);
impl ToString for  TheNumber {
    fn to_string(&self) -> String { self.0.to_string() } // for implementing 1k 1b itd.
}
impl From<u32> for TheNumber {
    fn from(value: u32) -> Self { Self(value) }
}
