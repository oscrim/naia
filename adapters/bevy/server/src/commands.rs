use std::marker::PhantomData;

use bevy::ecs::entity::Entity;

use naia_server::{ProtocolType, Replicate, Server, UserKey};

use naia_bevy_shared::WorldMut;

// Command Trait

pub trait Command<P: ProtocolType>: Send + Sync + 'static {
    fn write(self: Box<Self>, server: &mut Server<P, Entity>, world: WorldMut);
}

//// Despawn Component ////

#[derive(Debug)]
pub(crate) struct DespawnEntity {
    entity: Entity,
}

impl DespawnEntity {
    pub fn new(entity: &Entity) -> Self {
        return DespawnEntity { entity: *entity };
    }
}

impl<P: ProtocolType> Command<P> for DespawnEntity {
    fn write(self: Box<Self>, server: &mut Server<P, Entity>, world: WorldMut) {
        server.entity_mut(world, &self.entity).despawn();
    }
}

//// Insert Component ////

#[derive(Debug)]
pub(crate) struct InsertComponent<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> {
    entity: Entity,
    component: R,
    phantom_p: PhantomData<P>,
}

impl<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> InsertComponent<P, R> {
    pub fn new(entity: &Entity, component: R) -> Self {
        return InsertComponent {
            entity: *entity,
            component,
            phantom_p: PhantomData,
        };
    }
}

impl<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> Command<P> for InsertComponent<P, R> {
    fn write(self: Box<Self>, server: &mut Server<P, Entity>, world: WorldMut) {
        server
            .entity_mut(world, &self.entity)
            .insert_component(self.component);
    }
}

//// Remove Component ////

#[derive(Debug)]
pub(crate) struct RemoveComponent<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> {
    entity: Entity,
    phantom_p: PhantomData<P>,
    phantom_r: PhantomData<R>,
}

impl<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> RemoveComponent<P, R> {
    pub fn new(entity: &Entity) -> Self {
        return RemoveComponent {
            entity: *entity,
            phantom_p: PhantomData,
            phantom_r: PhantomData,
        };
    }
}

impl<P: ProtocolType, R: Replicate<P> + bevy::prelude::Component> Command<P> for RemoveComponent<P, R> {
    fn write(self: Box<Self>, server: &mut Server<P, Entity>, world: WorldMut) {
        server
            .entity_mut(world, &self.entity)
            .remove_component::<R>();
    }
}

//// Own Entity ////

#[derive(Debug)]
pub(crate) struct OwnEntity {
    entity: Entity,
    user_key: UserKey,
}

impl OwnEntity {
    pub fn new(entity: &Entity, user_key: &UserKey) -> Self {
        return OwnEntity {
            entity: *entity,
            user_key: *user_key,
        };
    }
}

impl<P: ProtocolType> Command<P> for OwnEntity {
    fn write(self: Box<Self>, server: &mut Server<P, Entity>, world: WorldMut) {
        server
            .entity_mut(world, &self.entity)
            .set_owner(&self.user_key);
    }
}
