use bevy::ecs::entity::Entity;

use naia_server::{ProtocolType, Replicate, RoomKey, UserKey};

use super::{
    commands::{DespawnEntity, InsertComponent, OwnEntity, RemoveComponent},
    server::Server,
};

// EntityMut

pub struct EntityMut<'a, 'b, P: ProtocolType> {
    entity: Entity,
    server: &'b mut Server<'a, 'a, P>,
}

impl<'a, 'b, P: ProtocolType> EntityMut<'a, 'b, P> {
    pub fn new(entity: Entity, server: &'b mut Server<'a, 'a, P>) -> Self {
        return EntityMut { entity, server };
    }

    #[inline]
    pub fn id(&self) -> Entity {
        self.entity
    }

    // Despawn

    pub fn despawn(&mut self) {
        self.server.add(DespawnEntity::new(&self.entity))
    }

    // Components

    pub fn insert<R: Replicate<P> + bevy::prelude::Component>(&mut self, component: R) -> &mut Self {
        self.server
            .add(InsertComponent::new(&self.entity, component));
        self
    }

    pub fn remove<R: Replicate<P> + bevy::prelude::Component>(&mut self) -> &mut Self {
        self.server.add(RemoveComponent::<P, R>::new(&self.entity));
        self
    }

    // Users

    pub fn set_owner(&mut self, user_key: &UserKey) -> &mut Self {
        self.server.add(OwnEntity::new(&self.entity, user_key));
        self
    }

    pub fn disown(&mut self) -> &mut Self {
        self.server.entity_disown(&self.entity);

        self
    }

    // Rooms

    pub fn enter_room(&mut self, room_key: &RoomKey) -> &mut Self {
        self.server.room_add_entity(room_key, &self.entity);

        self
    }

    pub fn leave_room(&mut self, room_key: &RoomKey) -> &mut Self {
        self.server.room_remove_entity(room_key, &self.entity);

        self
    }

    // Exit

    pub fn server(&mut self) -> &mut Server<'a, 'a, P> {
        self.server
    }
}
