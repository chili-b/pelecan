use confy;
use serde_derive::{Serialize, Deserialize};

use crate::Persistent;

// MAKE ALL FIELDS PUBLIC

// DATA THAT IS ONLY IN MEMORY
#[derive(Clone)]
pub struct VolatileData {
    // VOLATILE DATA FIELDS //
}

impl VolatileData {
    pub fn new() -> Self {
        Self {
            // VOLATILE DATA VALUES //
        }
    }
}

// DATA THAT IS STORED ON DISK
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PersistentData {
    // PERSISTENT DATA FIELDS //
}

impl PersistentData {
    pub fn new() -> Self {
        Self {
            // PERSISTENT DATA VALUES //
        }
    }
}

impl Persistent for PersistentData {}

impl std::default::Default for PersistentData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone)]
pub struct Data {
    pub volatile_data: VolatileData,
    pub persistent_data: PersistentData,
    pub name: String,
}

impl Data {
    pub fn new(name: &str) -> Self {
        Self {
            volatile_data: VolatileData::new(),
            persistent_data: PersistentData::load(name),
            name: name.to_owned()
        }
    }

    pub fn store(&self) {
        self.persistent_data.store(&self.name);
    }
}
