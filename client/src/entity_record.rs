use std::any::TypeId;

use naia_shared::LocalComponentKey;

use super::component_record::ComponentRecord;

#[derive(Debug)]
pub struct EntityRecord {
    component_record: ComponentRecord<LocalComponentKey>,
    pub is_prediction: bool,
}

impl EntityRecord {
    pub fn new() -> Self {
        EntityRecord {
            component_record: ComponentRecord::new(),
            is_prediction: false,
        }
    }

    pub fn get_key_from_type(&self, type_id: &TypeId) -> Option<&LocalComponentKey> {
        return self.component_record.get_key_from_type(type_id);
    }

    pub fn insert_component(&mut self, key: &LocalComponentKey, type_id: &TypeId) {
        return self.component_record.insert_component(key, type_id);
    }

    pub fn remove_component(&mut self, key: &LocalComponentKey) {
        return self.component_record.remove_component(key);
    }

    pub fn get_component_keys(&self) -> Vec<LocalComponentKey> {
        return self.component_record.get_component_keys();
    }
}
