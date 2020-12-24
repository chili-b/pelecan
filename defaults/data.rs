use serde_derive::{Serialize, Deserialize};
use confy::ConfyError;
use crate::Persistent;
use dirs;
use std::path::PathBuf;

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
    pub path: PathBuf,
}

impl Data {
    pub fn new(name: &str) -> Self {
        let local_data_path = dirs::data_local_dir().unwrap();
        let path = local_data_path.join(format!("pelecan/{}", name));
        Self {
            volatile_data: VolatileData::new(),
            persistent_data: PersistentData::load(path.join("persistent_data.toml")),
            path: path
        }
    }

    pub fn store(&self) -> Result<(), ConfyError> {
        self.persistent_data.store(&self.path.join("persistent_data.toml"))
    }
}
