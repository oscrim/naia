
use std::any::{TypeId};
use std::collections::HashMap;

use crate::{Event, EventTypeGetter, EventType, Entity, EntityTypeGetter, EntityType};

pub struct Manifest<T: EventType, U: EntityType> {
    event_gaia_id_count: u16,
    event_gaia_id_map: HashMap<u16, T>,
    event_type_id_map: HashMap<TypeId, u16>,
    ////
    entity_gaia_id_count: u16,
    entity_gaia_id_map: HashMap<u16, U>,
    entity_type_id_map: HashMap<TypeId, u16>,
}

impl<T: EventType, U: EntityType> Manifest<T, U> {
    pub fn new() -> Self {
        Manifest {
            event_gaia_id_count: 0,
            event_gaia_id_map: HashMap::new(),
            event_type_id_map: HashMap::new(),
            ///
            entity_gaia_id_count: 0,
            entity_gaia_id_map: HashMap::new(),
            entity_type_id_map: HashMap::new()
        }
    }

    pub fn register_event<S: Event<T>>(&mut self, some_type: &S) {
        let new_gaia_id = self.event_gaia_id_count;
        let type_id = EventTypeGetter::get_type_id(some_type);
        self.event_type_id_map.insert(type_id, new_gaia_id);
        self.event_gaia_id_map.insert(new_gaia_id, Event::<T>::to_type(some_type));
        self.event_gaia_id_count += 1;
    }

    pub fn get_event_gaia_id(&self, type_id: &TypeId) -> u16 {
        let gaia_id = self.event_type_id_map.get(type_id)
            .expect("hey I should get a TypeId here...");
        return *gaia_id;
    }

    pub fn create_event(&self, gaia_id: u16) -> Option<T> {
        match self.event_gaia_id_map.get(&gaia_id) {
            Some(event_type) => {
                return Some(event_type.clone());
            }
            None => {}
        }

        return None;
    }

    pub fn register_entity<S: Entity<U>>(&mut self, some_type: &S) {
        let new_gaia_id = self.entity_gaia_id_count;
        let type_id = EntityTypeGetter::get_type_id(some_type);
        self.entity_type_id_map.insert(type_id, new_gaia_id);
        self.entity_gaia_id_map.insert(new_gaia_id, Entity::<U>::to_type(some_type));
        self.entity_gaia_id_count += 1;
    }

    pub fn get_entity_gaia_id(&self, type_id: &TypeId) -> u16 {
        let gaia_id = self.entity_type_id_map.get(type_id)
            .expect("hey I should get a TypeId here...");
        return *gaia_id;
    }

    pub fn create_entity(&self, gaia_id: u16) -> Option<U> {
        match self.entity_gaia_id_map.get(&gaia_id) {
            Some(entity_type) => {
                return Some(entity_type.init());
            }
            None => {}
        }

        return None;
    }
}