use std::ops::{Deref, DerefMut};

use bevy::{ecs::component::Mutable, prelude::*};

pub trait SyncSource {
    type Target;
    fn send(&self) -> Self::Target;
}
impl<T: Clone, S: Deref<Target = T>> SyncSource for S {
    type Target = T;
    fn send(&self) -> T { (*self).clone() }
}



pub trait SyncRecive<T> {
    fn get_mut_reciver(&mut self) -> &mut T;
    fn recive(&mut self, val: T){
        *self.get_mut_reciver() = val;
    }
}
impl<T, S: DerefMut<Target = T>> SyncRecive<T> for S {
    fn get_mut_reciver(&mut self) -> &mut T { self.deref_mut() }
}


pub fn sync_components<S,R,T>(
    sender: Single<&S>,
    recivers: Query<&mut R>,
) 
where 
S: SyncSource<Target = T> + Component,
R: SyncRecive<T> + Component<Mutability = Mutable>, 
T: Clone {
    for mut reciver in recivers{
        (*reciver).recive((*sender).send());
    }
}

pub fn sync_resource_to_components<S,R,T>(
    sender: Res<S>,
    recivers: Query<&mut R>,
) 
where 
S: SyncSource<Target = T> + Resource,
R: SyncRecive<T> + Component<Mutability = Mutable>, 
T: Clone {
    for mut reciver in recivers{
        (*reciver).recive((*sender).send());
    }
}

pub fn sync_component_to_resource<S,R,T>(
    sender: Single<&S>,
    mut reciver: ResMut<R>,
) 
where 
S: SyncSource<Target = T> + Component,
R: SyncRecive<T> + Resource, 
T: Clone {
    (*reciver).recive((*sender).send());
}

pub fn sync_resources<S,R,T>(
    sender: Res<S>,
    mut reciver: ResMut<R>,
) 
where 
S: SyncSource<Target = T> + Resource,
R: SyncRecive<T> + Resource, 
T: Clone {
    (*reciver).recive((*sender).send());
}