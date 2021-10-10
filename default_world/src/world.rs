use std::{any::TypeId, collections::HashMap};

use slotmap::DenseSlotMap;

use naia_shared::{ImplRef, ProtocolType, Ref, Replicate};

use naia_server::{EntityType, WorldMutType};

#[allow(missing_docs)]
#[allow(unused_doc_comments)]
mod entity_key {
    // The Key used to reference an Entity
    new_key_type! { pub struct EntityKey; }
}

use entity_key::EntityKey as Key;

pub type EntityKey = Key;

impl EntityType for EntityKey {}

/// A default World which implements WorldMutType and that Naia can use to store
/// Entities/Components. It's recommended to use this only when you do not have
/// another ECS library's own World available.
pub struct World<P: ProtocolType> {
    entities: DenseSlotMap<entity_key::EntityKey, HashMap<TypeId, P>>,
}

impl<P: ProtocolType> World<P> {
    /// Create a new default World
    pub fn new() -> Self {
        World {
            entities: DenseSlotMap::with_key(),
        }
    }
}

impl<P: ProtocolType> WorldMutType<P, EntityKey> for World<P> {
    fn has_entity(&self, entity_key: &EntityKey) -> bool {
        return self.entities.contains_key(*entity_key);
    }

    fn entities(&self) -> Vec<EntityKey> {
        let mut output = Vec::new();

        for (key, _) in &self.entities {
            output.push(key);
        }

        return output;
    }

    fn spawn_entity(&mut self) -> EntityKey {
        let component_map = HashMap::new();
        return self.entities.insert(component_map);
    }

    fn despawn_entity(&mut self, entity_key: &EntityKey) {
        self.entities.remove(*entity_key);
    }

    fn has_component<R: Replicate<P>>(&self, entity_key: &EntityKey) -> bool {
        if let Some(component_map) = self.entities.get(*entity_key) {
            return component_map.contains_key(&TypeId::of::<R>());
        }

        return false;
    }

    fn has_component_of_type(&self, entity_key: &EntityKey, component_type: &TypeId) -> bool {
        if let Some(component_map) = self.entities.get(*entity_key) {
            return component_map.contains_key(component_type);
        }

        return false;
    }

    fn get_component<R: Replicate<P>>(&self, entity_key: &EntityKey) -> Option<Ref<R>> {
        if let Some(component_map) = self.entities.get(*entity_key) {
            if let Some(component_protocol) = component_map.get(&TypeId::of::<R>()) {
                return component_protocol.to_typed_ref::<R>();
            }
        }

        return None;
    }

    fn get_component_from_type(
        &self,
        entity_key: &EntityKey,
        component_type: &TypeId,
    ) -> Option<P> {
        if let Some(component_map) = self.entities.get(*entity_key) {
            if let Some(protocol) = component_map.get(component_type) {
                return Some(protocol.clone());
            }
        }

        return None;
    }

    fn insert_component<R: ImplRef<P>>(&mut self, entity_key: &EntityKey, component_ref: R) {
        if let Some(component_map) = self.entities.get_mut(*entity_key) {
            let protocol = component_ref.protocol();
            let type_id = protocol.get_type_id();
            if component_map.contains_key(&type_id) {
                panic!("Entity already has a Component of that type!");
            }
            component_map.insert(type_id, protocol);
        }
    }

    fn remove_component<R: Replicate<P>>(&mut self, entity_key: &EntityKey) {
        if let Some(component_map) = self.entities.get_mut(*entity_key) {
            component_map.remove(&TypeId::of::<R>());
        }
    }

    fn get_components(&self, entity_key: &EntityKey) -> Vec<P> {
        let mut output: Vec<P> = Vec::new();

        if let Some(component_map) = self.entities.get(*entity_key) {
            for (_, component_protocol) in component_map {
                output.push(component_protocol.clone());
            }
        }

        return output;
    }
}
